//! Convenience functions for parsing files.
use std::fs;
use std::io::prelude::Read;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;
use std::rc::Rc;

use toml;
use serde;
use build::identifier;
use build::num::iter::range;
use build::num::integer::Integer;
use build::num::traits::{ToPrimitive, Zero, NumCast};

// Thanks http://stackoverflow.com/questions/30291584/macro-for-defining-trait-aliases
macro_rules! items
{
	($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias
{
	($name:ident, $($base:tt)+) =>
	{
		items!{
			trait $name: $($base)+ {}
			impl<T: $($base)+> $name for T {}
		}
	};
	(pub $name:ident, $($base:tt)+) =>
	{
		items!{
			pub trait $name: $($base)+ {}
			impl<T: $($base)+> $name for T {}
		}
	};
}

trait_alias!(pub NumericInteger, Clone + Hash + Integer + Zero + NumCast + Display);

/// Returns true if the give map contains sequential keys from 0 to the map length.
pub fn is_sequential_set<NumType, ValueType>(map: &HashMap<NumType, ValueType>) -> Result<(), usize>
	where NumType: NumericInteger
{
	let len = NumType::from(map.len());
	if len.is_none()
	{
		return Err(0);
	}
	for i in range(NumType::zero(), len.unwrap())
	{
		if !map.contains_key(&i)
		{
			return Err(i.to_usize().unwrap());
		}
	}
	Ok(())
}

/// Traveses through the given directory and parses each TOML file before giving it to the callback.
fn toml_traverse_directory<'a, F>(directory: &str, callback: &'a mut F)
	where F: FnMut(toml::Table)
{
	let paths = fs::read_dir(directory);
	if let Err(err) = paths
	{
		panic!("Unable to traverse through directory `{}`: {:?}", directory, err.kind());
	}

	for filepath in paths.unwrap()
	{
		if let Ok(filepath) = filepath
		{
			let file = fs::File::open(filepath.path());
			if let Err(err) = file
			{
				panic!("Unable to open file `{}`: {:?}", filepath.path().display(), err.kind());
			}
			
			let mut file_contents = String::new();
			if let Err(err) = file.unwrap().read_to_string(&mut file_contents)
			{
				panic!("Unable to read file `{}`: {:?}", filepath.path().display(), err.kind());
			}
			
			let mut parser = toml::Parser::new(&file_contents);
			if let Some(toml_base) = parser.parse()
			{
				callback(toml_base);
			}
			else
			{
				println!("Warning: Non-toml file {} found: {:?}", filepath.path().display(),
					parser.errors);
			}
		}
	}
}

pub trait Resource<IdType: NumericInteger>
{
	fn id(&self) -> IdType;
	fn name(&self) -> Rc<String>;
	fn internal(&self) -> Option<String>;
}

macro_rules! resource
{
	($res:ident, $num:ident) =>
	{
		impl Resource<$num> for $res
		{
			fn id(&self) -> $num
			{
				self.id
			}
			fn name(&self) -> Rc<String>
			{
				self.name.clone()
			}
			fn internal(&self) -> Option<String>
			{
				self.internal.clone()
			}
		}
	}
}

pub fn parse_toml_resource_directory<IdType, ResourceType>(directory: &str, resource: &str,
	sequential: bool) -> HashMap<IdType, (ResourceType, identifier::Identifier)>
	where ResourceType: serde::de::Deserialize + Resource<IdType>,
		IdType: NumericInteger
{
	println!("Parsing directory {}...", directory);
	let mut resource_map: HashMap<IdType, (ResourceType, identifier::Identifier)> = HashMap::new();
	
	toml_traverse_directory(directory, &mut |toml_base: toml::Table|
	{
		if let Some(base) = toml_base.get(resource)
		{
			let mut decoder = toml::Decoder::new(base.clone());
			let res: ResourceType = serde::de::Deserialize::deserialize(&mut decoder).unwrap();
			
			println!("Found {}: \"{}\"; ID: {}.", resource, res.name(), res.id());
			
			let ident = identifier::Identifier::from_name(res.name().clone(), res.internal().clone());
			if ident.is_none()
			{
				panic!("Unable to create {} symbol `{}`. Symbol invalid.", resource, res.name());
			}
			
			let resource_id = res.id();
			let resource_name = res.name().clone();
			if let Some(_) = resource_map.insert(res.id(), (res, ident.unwrap()))
			{
				panic!("Unable to insert {} ID `{}` for `{}`.", resource, resource_name, resource_id);
			}
		}
		else
		{
			panic!("File is missing `{}` section.", resource);
		}
	});
	
	if sequential
	{
		println!("Validing {} IDs...", resource);
		if let Err(err_id) = is_sequential_set(&resource_map)
		{
			panic!("Invalid {} ID `{}`. Number not found or invalid range.", resource, err_id);
		}
		else
		{
			println!("IDs for {} validated.", resource);
		}
	}
	else
	{
		println!("Skipping sequential ID validation.");
	}
	
	resource_map
}
