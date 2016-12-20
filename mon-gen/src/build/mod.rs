//! Transpiles code from TOML files.
#[macro_use] mod util;

mod elements;
mod gender;
mod species;
mod locations;
mod monster;
mod attacks;

use std::default::Default;
use std::path::Path;
use std::fs::{File, OpenOptions, create_dir_all, read_dir, metadata};
use std::io::{Read, Write, Seek, SeekFrom};
use std::io;
use std::fmt;
use std::collections::HashSet;
use std::hash::Hash;
use toml;
use serde;
use filetime::FileTime;

use build::util::Identifiable;
use build::elements::ElementFile;
use build::gender::GenderClassifiers;
use build::locations::LocationClassifiers;
use build::monster::MonsterClassifiers;
use build::species::{SpeciesFile, Species, SpeciesClassifiers};
use build::attacks::{AttackFile, Attack, AttackClassifiers};

/// Represents a detailed TOML parser error.
#[derive(Debug)]
pub struct TomlParserError
{
	/// The file name of the TOML file.
	pub filename: String,

	/// A short description of the error.
	pub desc: String,

	/// The starting line position of the error.
	pub start_line: usize,

	/// The starting column relative to the starting line position of the error.
	pub start_col: usize,

	/// The ending line position of the error.
	pub end_line: usize,

	/// The ending column relative to the ending line position of the error.
	pub end_col: usize,
}

impl TomlParserError
{
	/// Generates a list of detailed TOML parser errors using the given `parser`.
	///
	/// This function queries the given `parser` for the list of errors, which normally returns
	/// only the start and end bytes.
	///
	pub fn from_parser(parser: &toml::Parser, name: &String) -> Vec<Self>
	{
		parser.errors.iter().map(|error|
		{
			let (start_line, start_col) = parser.to_linecol(error.lo);
			let (end_line, end_col) = parser.to_linecol(error.hi);
			// Note: The lines are 0 indexed so increment by 1.
			TomlParserError
			{
				filename: name.clone(),
				start_line: start_line + 1,
				start_col: start_col,
				end_line: end_line + 1,
				end_col: end_col,
				desc: error.desc.clone(),
			}
		}).collect()
	}
}

/// Represents an error from generating code out of TOML definitions.
#[derive(Debug)]
pub enum Error
{
	IoError(io::Error),
	TomlError(Vec<TomlParserError>),
	SyntaxError(String),
}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
	{
		match *self
		{
			Error::IoError(ref e) => try!(writeln!(f, "IO error: {:?}", e)),
			Error::TomlError(ref e) =>
			{
				try!(writeln!(f, "Error parsing TOML:"));
				if !e.is_empty()
				{
					for toml_error in e
					{
						try!(writeln!(f,
							" - At file `{}`: \"{}\" at line {}, col {} - line {}, col {}",
							toml_error.filename, toml_error.desc, toml_error.start_line,
							toml_error.start_col, toml_error.end_line, toml_error.end_col));
					}
				}
				else
				{
					try!(writeln!(f, " - Unknown error. Required field are missing or invalid."));
				}
			},
			Error::SyntaxError(ref e) => try!(writeln!(f, "Syntax error: {}", e)),
		}
		Ok(())
	}
}

impl From<io::Error> for Error
{
	fn from(f: io::Error) -> Self
	{
		Error::IoError(f)
	}
}

impl From<Vec<TomlParserError>> for Error
{
	fn from(f: Vec<TomlParserError>) -> Self
	{
		Error::TomlError(f)
	}
}

pub type BuildResult = Result<(), Error>;
type ProcessedResult = Result<bool, Error>;

/// Error message for unimplemented validity check.
const UNIMPLEMENTED_ERROR: &'static str = "Unimplemented validity check.";

/// Functions for generating code.
pub trait CodeGenerate
{
	/// Returns whether or not the current data is valid for code generation.
	fn is_valid(&self) -> BuildResult
	{
		Err(Error::SyntaxError(UNIMPLEMENTED_ERROR.to_string()))
	}

	/// Generates Rust code.
	fn gen_rust(&self, _: &mut io::Write) -> BuildResult
	{
		Ok(())
	}

	/// Generates a GM-style constants mapping.
	fn gen_constants(&self, _: &mut io::Write) -> BuildResult
	{
		Ok(())
	}
}

/// Functions for generating code.
pub trait CodeGenerateGroup: Sized + Eq + Hash
{
	/// Returns whether or not the current data is valid for code generation.
	fn is_valid(_: &HashSet<Self>) -> BuildResult
	{
		Err(Error::SyntaxError(UNIMPLEMENTED_ERROR.to_string()))
	}

	/// Generates Rust code for a group of this object.
	fn gen_rust_group(_: &HashSet<Self>, _: &mut io::Write) -> BuildResult
	{
		Ok(())
	}

	/// Generates a GM-style constants mapping for a group of this object.
	fn gen_constants_group(_: &HashSet<Self>, _: &mut io::Write) -> BuildResult
	{
		Ok(())
	}
}

/// Stores build times of each individual generated component for identifiers.
#[derive(Debug, Default, Serialize, Deserialize)]
struct ClassifierBuildTimes
{
	elements: u64,
	genders: u64,
	locations: u64,
	monster: u64,
	battle: u64,
	species: u64,
}

/// Stores build times of each individual generated component.
#[derive(Debug, Default, Serialize, Deserialize)]
struct BuildTimes
{
	classifiers: ClassifierBuildTimes,
	species: u64,
	attacks: u64,
}

fn file_append_to_write(from: &mut File, to: &mut Write) -> io::Result<()>
{
	let mut contents = String::new();
	try!(from.seek(SeekFrom::Start(0)));
	try!(from.read_to_string(&mut contents));
	write!(to, "{}", contents)
}

/// Builds the entire Mon source code.
///
/// To improve performance, only files with have been modified unless `rebuild` is true. The
/// previous build times are loaded and saved in `build_file`.
///
pub fn build<P1, P2, P3>(build_cache_dir: P1, input_dir: P2, output_dir: P3, rebuild: bool)
	-> Result<bool, io::Error>
		where P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>
{
	println!("Generating Mon source code...");

	let build_file = build_cache_dir.as_ref().join("build.toml");
	let mut times: BuildTimes =
	{
		// Only load build times if necessary.
		if !rebuild && build_file.exists()
		{
			let mut file = try!(File::open(&build_file));
			let mut contents = String::new();
			try!(file.read_to_string(&mut contents));

			match toml::decode_str(&contents)
			{
				Some(t) => t,
				None =>
				{
					// Occurs when decoding has failed. Perhaps file definition has changed?
					// Ignore the older version. A newer one will be written after this.
					Default::default()
				}
			}
		}
		else
		{
			try!(create_dir_all(&build_cache_dir));
			Default::default()
		}
	};

	let mut failure = false;

	// Classifiers:
	let mut constants_genders = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_genders.rs")));
	failure = failure || !build_code::<GenderClassifiers, _, _>(
		input_dir.as_ref().join("classifiers/genders.toml"),
		output_dir.as_ref().join("gender.rs"), &mut times.classifiers.genders, rebuild,
		&mut constants_genders);

	let mut constants_elements = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_elements.rs")));
	failure = failure || !build_code::<ElementFile, _, _>(
		input_dir.as_ref().join("classifiers/elements.toml"),
		output_dir.as_ref().join("element.rs"), &mut times.classifiers.elements, rebuild,
		&mut constants_elements);

	let mut constants_locations = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_locations.rs")));
	failure = failure || !build_code::<LocationClassifiers, _, _>(
		input_dir.as_ref().join("classifiers/locations.toml"),
		output_dir.as_ref().join("locations.rs"), &mut times.classifiers.locations, rebuild,
		&mut constants_locations);

	let mut constants_monsters = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_monsters.rs")));
	failure = failure || !build_code::<MonsterClassifiers, _, _>(
		input_dir.as_ref().join("classifiers/monsters.toml"),
		output_dir.as_ref().join("monster.rs"), &mut times.classifiers.monster, rebuild,
		&mut constants_monsters);

	// TODO
	let mut constants_battle = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_attack.rs")));
	failure = failure || !build_code::<AttackClassifiers, _, _>(
		input_dir.as_ref().join("classifiers/attack.toml"),
		output_dir.as_ref().join("attack.rs"), &mut times.classifiers.battle, rebuild,
		&mut constants_battle);

	let mut constants_species = try!(OpenOptions::new().read(true).write(true).create(true).open(
		build_cache_dir.as_ref().join("constants_species.rs")));
	failure = failure || !build_code::<SpeciesClassifiers, _, _>(
		input_dir.as_ref().join("classifiers/species.toml"),
		output_dir.as_ref().join("species.rs"), &mut times.classifiers.species, rebuild,
		&mut constants_species);

	// Global:
	let mut constants_species_list = try!(OpenOptions::new().read(true).write(true).create(true)
		.open(build_cache_dir.as_ref().join("constants_species_list.rs")));
	failure = failure || !build_code_dir::<SpeciesFile, _, _, _, Species>(
		input_dir.as_ref().join("species"), output_dir.as_ref().join("species_list.rs"),
		&mut times.species, rebuild, &mut constants_species_list, &mut |file| file.species);
	let mut constants_attack_list = try!(OpenOptions::new().read(true).write(true).create(true)
		.open(build_cache_dir.as_ref().join("constants_attack_list.rs")));
	failure = failure || !build_code_dir::<AttackFile, _, _, _, Attack>(
		input_dir.as_ref().join("attacks"), output_dir.as_ref().join("attack_list.rs"),
		&mut times.attacks, rebuild, &mut constants_attack_list, &mut |file| file.attack);


	let mut file = try!(File::create(build_file));
	try!(file.write_all(&toml::encode_str(&times).as_bytes()));

	println!("Building constants");
	if cfg!(feature = "c_api")
	{
		let mut constants = try!(File::create(build_cache_dir.as_ref().join("constants.txt")));
		try!(file_append_to_write(&mut constants_genders, &mut constants));
		try!(file_append_to_write(&mut constants_elements, &mut constants));
		try!(file_append_to_write(&mut constants_locations, &mut constants));
		try!(file_append_to_write(&mut constants_monsters, &mut constants));
		try!(file_append_to_write(&mut constants_battle, &mut constants));
		try!(file_append_to_write(&mut constants_species, &mut constants));
		try!(file_append_to_write(&mut constants_species_list, &mut constants));
	}

	Ok(!failure)
}

fn build_code<T, P1, P2>(input_path: P1, output_dir: P2, build_times: &mut u64,
	rebuild: bool, output_constants: &mut Write) -> bool
		where T: serde::Deserialize + CodeGenerate, P1: AsRef<Path>, P2: AsRef<Path>
{
	build_code_func(&mut ||
	{
		print!("Building file `{:?}`... ", input_path.as_ref());
		build_from_time(&input_path, build_times, rebuild, &mut |t: T|
		{
			print!("WRITING... ");
			let mut output_rust = try!(File::create(&output_dir));
			try!(t.is_valid());
			try!(t.gen_rust(&mut output_rust));
			try!(t.gen_constants(output_constants));
			Ok(())
		})
	})
}

fn build_code_dir<T, P1, P2, F, U>(input_path: P1, output_dir: P2, build_times: &mut u64,
	rebuild: bool, output_constants: &mut Write, convert_func: &mut F) -> bool
	where T: serde::Deserialize, U: 'static + CodeGenerateGroup + Eq + Hash + Identifiable,
		P1: AsRef<Path>, P2: AsRef<Path>, F: FnMut(T) -> U
{
	build_code_func(&mut ||
	{
		print!("Building directory `{:?}`... ",input_path.as_ref());
		build_dir_from_time(&input_path, build_times, rebuild, convert_func,
			&mut |t: &HashSet<U>|
		{
			print!("WRITING... ");
			let mut output_rust = try!(File::create(&output_dir));
			try!(U::is_valid(t));
			try!(U::gen_rust_group(t, &mut output_rust));
			try!(U::gen_constants_group(t, output_constants));
			Ok(())
		})
	})
}

fn build_code_func<F>(build_func: &mut F) -> bool where F: FnMut() -> ProcessedResult
{
	let build = build_func();
	match build
	{
		Ok(b) =>
		{
			if b == true
			{
				println!("DONE.");
			}
			else
			{
				println!("SKIPPED");
			}
			true
		}
		Err(e) =>
		{
			println!("{}", e);
			false
		}
	}
}

pub fn parse_toml<T: serde::Deserialize>(file: &mut File, name: &String) -> Result<T, Error>
{
	let mut contents = String::new();
	try!(file.read_to_string(&mut contents));

	let mut parser = toml::Parser::new(&contents);
	let toml = try!(parser.parse().ok_or(TomlParserError::from_parser(&parser, name)));

	toml::decode::<T>(toml::Value::Table(toml)).ok_or(Error::from(TomlParserError::from_parser(
		&parser, name)))
}

fn build_from_time<T, P, F>(path: P, build_time: &mut u64, rebuild: bool, closure: &mut F)
	-> ProcessedResult where T: serde::Deserialize + CodeGenerate, P: AsRef<Path>,
	F: FnMut(T) -> BuildResult
{
	let mut file = try!(File::open(&path));
	let name = format!("{}", path.as_ref().display());

	let metadata = try!(file.metadata());
	let time = FileTime::from_last_modification_time(&metadata).seconds_relative_to_1970();

	if rebuild || *build_time != time
	{
		let t: T = try!(parse_toml(&mut file, &name));
		try!(closure(t));
		*build_time = time;
		Ok(true)
	}
	else
	{
		Ok(false)
	}
}

fn build_dir_from_time<T, P, F, F2, U>(path: P, build_time: &mut u64, rebuild: bool,
	convert_func: &mut F2, closure: &mut F) -> ProcessedResult
	where T: serde::Deserialize, U: 'static + CodeGenerateGroup + Eq + Hash + Identifiable, P: AsRef<Path>,
		F: FnMut(&HashSet<U>) -> BuildResult, F2: FnMut(T) -> U
{
	let dir = try!(read_dir(&path));
	let (lower_bound_size, _) = dir.size_hint();
	let mut set = HashSet::with_capacity(lower_bound_size);

	let metadata = try!(metadata(&path));
	let time = FileTime::from_last_modification_time(&metadata).seconds_relative_to_1970();

	if rebuild || *build_time != time
	{
		for path in dir
		{
			let filepath = try!(path);
			let mut file = try!(File::open(filepath.path()));
			let name = try!(filepath.file_name().into_string().map_err(|_| Error::SyntaxError(
				"Unable to read OS file metadata".to_string())));

			let toml_contents = parse_toml::<T>(&mut file, &name);
			match toml_contents
			{
				Ok(contents) =>
				{
					let deserialized = convert_func(contents);
					{
						if !Identifiable::identifier_valid(&deserialized)
						{
							return Err(Error::SyntaxError(format!(
								"Invalid resource identifier for file `{}`: `{}`",
									filepath.path().display(),
									Identifiable::identifier(&deserialized))));
						}
					}
					set.insert(deserialized);
				}
				Err(e) =>
				{
					println!("Unable to parse file `{}`.", filepath.path().display());
					return Err(e);
				}
			}
		}
		try!(closure(&set));

		*build_time = time;
		Ok(true)
	}
	else
	{
		Ok(false)
	}
}

// pub use build::build as build_gen;
