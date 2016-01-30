use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::prelude::Read;

use super::parse::is_sequential_set;

use toml;
use serde;

#[derive(Eq, Debug, Serialize, Deserialize)]
struct IdNamedPair
{
	id: u8,
	name: Rc<String>,
}

impl PartialEq for IdNamedPair
{
	fn eq(&self, other: &IdNamedPair) -> bool
	{
		self.id == other.id
	}
}

impl Hash for IdNamedPair
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.id.hash(state)
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Ailments
{
	nonvolatile: HashSet<IdNamedPair>,
	volatile: HashSet<IdNamedPair>,
	battle: HashSet<IdNamedPair>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Classifiers
{
	natures: HashSet<IdNamedPair>,
	growth: HashSet<IdNamedPair>,
	groups: HashSet<IdNamedPair>,
	colors: HashSet<IdNamedPair>,
	habitats: HashSet<IdNamedPair>,
	ailments: Ailments,
}

pub fn parse_classifiers()
{
	let filepath = "res/common.toml";
	let file = File::open(filepath);
	if let Err(err) = file
	{
		panic!("Unable to open file `{}`: {:?}", filepath, err.kind());
	}
	
	let mut file_contents = String::new();
	if let Err(err) = file.unwrap().read_to_string(&mut file_contents)
	{
		panic!("Unable to read file `{}`: {:?}", filepath, err.kind());
	}
	
	let mut parser = toml::Parser::new(&file_contents);
	if let Some(toml_base) = parser.parse()
	{
		if let Some(base) = toml_base.get("classifiers")
		{
			let mut decoder = toml::Decoder::new(base.clone());
			let res: Classifiers = serde::de::Deserialize::deserialize(&mut decoder).unwrap();
			
			println!("Opening `constants.txt`...");
			let constants_file_result = OpenOptions::new().append(true).open(
				"import/constants.txt");
			if let Err(err) = constants_file_result
			{
				panic!("Unable to open file `constants.txt`: {:?}", err.kind());
			}
			let mut constants_file = constants_file_result.unwrap();
			
			println!("Generating source code `src/gen/classifiers_gen.rs`...");
			let source_file_result = File::create("src/gen/classifiers_gen.rs");
			if let Err(err) = source_file_result
			{
				panic!("Unable to create source code `src/gen/element_gen.rs`: {:?}", err.kind());
			}
			let mut source_file = source_file_result.unwrap();
			
			if let Err(err) = writeln!(source_file,
"use rand::{{Rand, Rng}};
use rand::distributions::{{IndependentSample, Range}};
use num::traits::FromPrimitive;")
			{
				panic!("Unable to write header to source file `classifiers_gen.rs`: {:?}",
					err.kind());
			}
			
			{
				let write_classifier = &mut |name: &str, data: &HashSet<IdNamedPair>|
				{
					let uppercase = name.to_uppercase();
					if let Err(err) = writeln!(source_file,
"#[derive(Copy, Clone, NumFromPrimitive)]
pub enum {}
{{", name)
					{
						panic!("Unable to write to source file `classifiers_gen.rs`: {:?}",
							err.kind());
					}
					for classifier in data
					{
						if let Err(err) = writeln!(constants_file, "MON_{}_{}={}", uppercase,
							classifier.name.to_uppercase(), classifier.id)
						{
							panic!("Unable to write to file `constants.txt`: {:?}", err.kind());
						}
						if let Err(err) = writeln!(source_file, "\t{} = {},", classifier.name,
							classifier.id)
						{
							panic!("Unable to write to source file `classifiers_gen.rs`: {:?}",
								err.kind());
						}
					}
					if let Err(err) = writeln!(source_file, "}}")
					{
						panic!("Unable to write to source file `classifiers_gen.rs`: {:?}",
							err.kind());
					}
				};
				
				write_classifier("Nature", &res.natures);
				write_classifier("Growth", &res.growth);
				write_classifier("Group", &res.groups);
				write_classifier("Color", &res.colors);
				write_classifier("Habitat", &res.habitats);
			}
			
			let number_map = res.natures.iter().map(|x|
			{
				(x.id, ())
			})
			.collect::<HashMap<u8, ()>>();
			
			if let Err(err) = is_sequential_set(&number_map)
			{
				panic!("Sequence is not sequential for resource type 'Nature' at id: {}.", err);
			}
			
			if let Err(err) = writeln!(source_file,
"impl Rand for Nature
{{
	fn rand<R: Rng>(rng: &mut R) -> Self
	{{
		let range = Range::new(0, 3);
		Nature::from_usize(range.ind_sample(rng)).unwrap()
	}}
}}")
			{
				panic!("Unable to write footer to source file `classifiers_gen.rs`: {:?}",
					err.kind());
			}
		}
	}
	else
	{
		panic!("Warning: Non-toml file {} found: {:?}", filepath, parser.errors);
	}
}
