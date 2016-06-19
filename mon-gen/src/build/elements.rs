//! Generates an `Element` enum.
use std::io::Write;
use std::collections::{HashSet, HashMap};

use types::element::{Id, EffectType};

use build::{BuildResult, CodeGenerate, Error};
use util::{IdResource, Identifiable, write_disclaimer};

#[derive(Debug, Deserialize)]
pub struct Element
{
	name: String,
	id: Id,
	internal: Option<String>,
	weaknesses: Vec<String>,
	resistances: Vec<String>,
	immunities: Vec<String>,
}

derive_for_id!(Element, Id);

#[derive(Debug, Deserialize)]
pub struct ElementFile
{
	element: HashSet<Element>,
}

impl CodeGenerate for ElementFile
{
	fn is_valid(&self) -> BuildResult
	{
		IdResource::sequential(&self.element)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		let mut symbols: HashMap<&String, Id> = HashMap::new();
		for element in &self.element
		{
			symbols.insert(&element.name, element.id);
		}

		let mut effectiveness = vec![1.0 as EffectType; self.element.len() * self.element.len()];
		for element in &self.element
		{
			let id = element.id;
			for weakness in &element.weaknesses
			{
				if let Some(other) = symbols.get(weakness)
				{
					effectiveness[(*other as usize) * self.element.len() + (id as usize)] = 2.0;
				}
				else
				{
					return Err(Error::SyntaxError(format!(
						"Unable to find element weakness '{}' from '{}'", weakness,
						element.name)));
				}
			}
			for resistance in &element.resistances
			{
				if let Some(other) = symbols.get(resistance)
				{
					effectiveness[(*other as usize) * self.element.len() + (id as usize)] = 0.5;
				}
				else
				{
					return Err(Error::SyntaxError(format!(
						"Unable to find element resistance '{}' from '{}'", resistance,
						element.name)));
				}
			}
			for immunity in &element.immunities
			{
				if let Some(other) = symbols.get(immunity)
				{
					effectiveness[(*other as usize) * self.element.len() + (id as usize)] = 0.0;
				}
				else
				{
					return Err(Error::SyntaxError(format!(
						"Unable to find element immunity '{}' from '{}'", immunity,
						element.name)));
				}
			}
		}

		try!(write_disclaimer(out, "`Element`"));
		try!(writeln!(out, "pub use base::types::element::{{Id, EffectType}};\n"));
		try!(IdResource::gen_rust_enum(out, "Element", &self.element));
		try!(writeln!(out,
"IterVariants!
{{
	(ElementVariants) pub enum Element
	{{"));
		try!(IdResource::gen_rust_enum_bare(out, &self.element, 2));
		try!(writeln!(out,
"	}}
}}

impl Element
{{
	pub const fn count() -> Id
	{{
		{count}
	}}
	pub fn name(&self) -> &'static [u8]
	{{
		const NAMES: [&'static [u8]; {count}] = [", count = self.element.len()));
		try!(IdResource::gen_rust_utf_literal(out, &self.element, 3));
		try!(writeln!(out,
"		];
		NAMES[*self as usize]
	}}
	pub fn effectiveness(&self, against: Element) -> EffectType
	{{
		const EFFECTS: [EffectType; {effectiveness_count}] = [",
			effectiveness_count = effectiveness.len()));
		for (index, effect) in effectiveness.iter().enumerate()
		{
			let defending = (index % self.element.len()) as Id;
			if defending == 0
			{
				let offending = (index / self.element.len()) as Id;
				try!(writeln!(out, "\t\t\t// {}", self.element.get(&offending).unwrap().name));
			}
			try!(writeln!(out, "\t\t\t{} as EffectType, // {}", effect,
				self.element.get(&defending).unwrap().name));
		}
		writeln!(out,
"		];
		EFFECTS[((*self as usize) * Element::count() as usize + (against as usize))]
	}}
}}").map_err(|e| Error::IoError(e))
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		IdResource::gen_constants(out, "ELEMENT", &self.element)
	}
}
