/// Common traits and functions for storing, validating and generating data.
use std::hash::Hash;
use std::collections::HashSet;
use std::borrow::Borrow;
use std::fmt::Display;
use std::io::Write;
use std::io;
use std::ascii::AsciiExt;

use num::{Integer, FromPrimitive, NumCast, One};
use num::iter::range;

use build::{Error, BuildResult};

/// Trait alias for numeric integers.
pub trait Numeric: Copy + Clone + Hash + One + Display + Integer + FromPrimitive + NumCast {}
impl<T: Copy + Clone + Hash + Display + One + Integer + FromPrimitive + NumCast> Numeric for T {}

/// For resources that may have identifiers.
pub trait Identifiable
{
	/// The display name of the resource.
	///
	/// The display name is always required. This name is used as an identifier unless an
	/// alternative identifier name is provided by `internal`.
	///
	fn name(&self) -> &String;

	/// The internal name of the resource.
	///
	/// The internal name is only required if the `name()` returns an invalid code identifier. The
	/// default implementation returns `None`.
	///
	fn internal(&self) -> Option<&String>
	{
		None
	}
}

impl Identifiable
{
	/// Returns the identifier value.
	///
	/// Returns the internal name if provided, or the display name otherwise. This function does
	/// not validate if the strings are valid identifiers.
	///
	pub fn identifier(&self) -> &String
	{
		if let Some(ref s) = self.internal()
		{
			return s;
		}
		self.name()
	}

	/// Returns true if the given string is valid as a code-compatible identifier.
	pub fn valid(identifier: &String) -> bool
	{
		let mut iter = identifier.chars();
		if let Some(c) = iter.next()
		{
			if c.is_alphabetic()
			{
				return identifier.chars().all(|c| (c.is_alphanumeric() || c == '_'));
			}
		}
		false
	}

	/// Returns true if the identifier is valid.
	pub fn identifier_valid(&self) -> bool
	{
		Identifiable::valid(self.identifier())
	}
}

/// A generic resource item.
pub trait IdResource<IdType: Numeric> : Identifiable
{
	/// The unique identifier number for the resource.
	///
	/// This number is used for hashing. As such, another resource with the same name is considered
	/// equal. However, this allows set recovery.
	///
	fn id(&self) -> IdType;
}

/// Prints a Rust style crate comment disclaimer for generated code.
pub fn write_disclaimer(out: &mut Write, name: &str) -> io::Result<()>
{
	try!(writeln!(out, "//! Generated code for {}.", name));
	writeln!(out, "// Edit at your own risk. Files may be modified by build script.\n")
}

/// Prints the given number of variable indentations.
pub fn write_indent(out: &mut Write, mut level: usize) -> io::Result<()>
{
	while level > 0
	{
		try!(write!(out, "\t"));
		level -= 1;
	}
	Ok(())
}

/// Prints a Rust style byte literal escaped version of the given string `s`.
pub fn write_utf8_escaped(out: &mut Write, s: &str) -> io::Result<()>
{
	let utf8 = s.chars().map(|c|
	{
		if !c.is_ascii()
		{
			let mut escaped = String::new();
			for utf8_char in c.encode_utf8()
			{
				escaped.push_str(&format!("\\x{:0width$x}", utf8_char, width = 2));
			}
			return escaped;
		}
		format!("{}", c)
	})
	.collect::<String>();
	write!(out, "b\"{}\\0\"", utf8)
}

impl<IdType: Numeric> IdResource<IdType>
{
	/// Returns true if the set sequence is sequential, false otherwise.
	///
	/// The error returned here is a syntax error indicating the missing index.
	///
	pub fn sequential<T>(ids: &HashSet<T>) -> BuildResult
		where T: IdResource<IdType> + Borrow<IdType> + Hash + Eq
	{
		let index_error = |index: usize| -> Error
		{
			Error::SyntaxError(format!("Invalid or missing index `{}`.", index))
		};

		let len = IdType::from(ids.len());
		if len.is_none()
		{
			return Err(index_error(0));
		}
		for i in range(IdType::zero(), len.unwrap())
		{
			if !ids.contains::<IdType>(&i)
			{
				return Err(index_error(i.to_usize().unwrap()));
			}
		}
		Ok(())
	}
	pub fn gen_constants<T>(out: &mut Write, prefix: &str, ids: &std::collections::HashSet<T>)
		-> BuildResult where T: 'static + IdResource<IdType> + Identifiable + Hash + Eq
	{
		for id in ids
		{
			let ident_capital = Identifiable::identifier(id).to_uppercase();
			try!(writeln!(out, "MON_{}_{}={}", prefix, ident_capital, id.id()));
		}
		Ok(())
	}
	pub fn gen_rust_enum<T>(out: &mut Write, name: &str, ids: &std::collections::HashSet<T>)
		-> BuildResult where T: 'static + IdResource<IdType> + Identifiable + Hash + Eq
	{
		Self::gen_rust_enum_indent(out, name, ids, 0)
	}
	pub fn gen_rust_enum_indent<T>(out: &mut Write, name: &str,
		ids: &std::collections::HashSet<T>, mut indent: usize) -> BuildResult
			where T: 'static + IdResource<IdType> + Identifiable + Hash + Eq
	{
		if cfg!(feature = "c_api")
		{
			try!(write_indent(out, indent));
			try!(writeln!(out, "enum_from_primitive!"));
			try!(write_indent(out, indent));
			try!(writeln!(out, "{{"));
			indent += 1;
			try!(write_indent(out, indent));
			try!(writeln!(out, "#[allow(dead_code)]"));
		}

		try!(write_indent(out, indent));
		try!(writeln!(out, "#[repr({})]", unsafe { std::intrinsics::type_name::<IdType>() }));
		try!(write_indent(out, indent));
		try!(writeln!(out, "#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]"));
		try!(write_indent(out, indent));
		try!(writeln!(out, "pub enum {}", name));
		try!(write_indent(out, indent));
		try!(writeln!(out, "{{"));

		try!(Self::gen_rust_enum_bare(out, ids, indent + 1));

		if cfg!(feature = "c_api")
		{
			try!(write_indent(out, indent));
			try!(writeln!(out, "}}"));
			indent -= 1;
		}
		try!(write_indent(out, indent));
		writeln!(out, "}}\n").map_err(|e| Error::IoError(e))
	}
	pub fn gen_rust_enum_bare<T>(out: &mut Write, ids: &std::collections::HashSet<T>,
		indent: usize) -> io::Result<()>
			where T: 'static + IdResource<IdType> + Identifiable + Hash + Eq
	{
		for id in ids
		{
			let ident = Identifiable::identifier(id);
			try!(write_indent(out, indent));
			try!(writeln!(out, "{} = {},", ident, id.id()));
		}
		Ok(())
	}
	pub fn gen_rust_utf_literal<T>(out: &mut Write, ids: &std::collections::HashSet<T>,
		indent: usize) -> io::Result<()>
			where T: 'static + IdResource<IdType> + Borrow<IdType> + Identifiable + Hash + Eq
	{
		for i in range(IdType::zero(), IdType::from_usize(ids.len()).unwrap())
		{
			let ee = ids.get(&i).unwrap();
			let ident = Identifiable::identifier(ee);
			try!(write_indent(out, indent));
			try!(write_utf8_escaped(out, ident));
			try!(writeln!(out, ","));
		}
		Ok(())
	}
}

/// Inserts functions for automatically deriving `Identifiable`.
macro_rules! derive_for_id_identifiable
{
	() =>
	{
		fn name(&self) -> &String
		{
			&self.name
		}
		fn internal(&self) -> Option<&String>
		{
			self.internal.as_ref()
		}
	}
}

/// Derives for a class that includes `id`, `name` and `internal` fields.
///
/// The derived class will be hashed using the `id` valud, will implement `Identifiable` and will
/// contain utility functions for generating code. For deriving a class that utilizes generics, use
/// `derive_for_id_generic`.
///
macro_rules! derive_for_id
{
	($i:ident, $ty:ident) =>
	{
		use std;

		impl PartialEq for $i
		{
			fn eq(&self, other: &Self) -> bool
			{
				self.id == other.id
			}
		}
		impl Eq for $i {}
		impl std::hash::Hash for $i
		{
			fn hash<H: std::hash::Hasher>(&self, state: &mut H)
			{
				self.id.hash(state)
			}
		}
		impl std::borrow::Borrow<$ty> for $i
		{
			fn borrow(&self) -> &$ty
			{
				&self.id
			}
		}
		impl IdResource<$ty> for $i
		{
			fn id(&self) -> $ty
			{
				self.id
			}
		}
		impl Identifiable for $i
		{
			derive_for_id_identifiable!();
		}
	}
}

/// Derives for a class utilizing generics that includes `id`, `name` and `internal` fields.
///
/// The given class must take only one generic, indicating the numerical type. See `derive_for_id`
/// for more information on what this does.
///
macro_rules! derive_for_id_generic
{
	($i:ty) =>
	{
		use std;

		impl<IdType: Numeric> PartialEq for $i
		{
			fn eq(&self, other: &Self) -> bool
			{
				self.id == other.id
			}
		}
		impl<IdType: Numeric> Eq for $i {}
		impl<IdType: Numeric> std::hash::Hash for $i
		{
			fn hash<H: std::hash::Hasher>(&self, state: &mut H)
			{
				self.id.hash(state)
			}
		}
		impl<IdType: Numeric> std::borrow::Borrow<IdType> for IdNamePair<IdType>
		{
			fn borrow(&self) -> &IdType
			{
				&self.id
			}
		}
		impl<IdType: Numeric> IdResource<IdType> for $i
		{
			fn id(&self) -> IdType
			{
				self.id
			}
		}
		impl<IdType: Numeric> Identifiable for $i
		{
			derive_for_id_identifiable!();
		}
	}
}

/// A deserializable struct that includes the bare minimum for hashing and an identifier.
#[derive(Debug, Deserialize)]
pub struct IdNamePair<IdType: 'static + Numeric>
{
	id: IdType,
	name: String,
	internal: Option<String>,
}

derive_for_id_generic!(IdNamePair<IdType>);

/// A hash set containing id-name pairs.
pub type IdNamePairSet<IdType> = HashSet<IdNamePair<IdType>>;
