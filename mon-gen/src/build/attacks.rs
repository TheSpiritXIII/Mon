use std::io::Write;
use std::collections::HashSet;

use types::attack::{Id, PowerType, AccuracyType, LimitType};

use build::{CodeGenerateGroup, BuildResult, Error};
use util::{IdResource, Identifiable, write_disclaimer, write_utf8_escaped};

fn default_side() -> String
{
	"Enemy".to_string()
}

fn default_range() -> String
{
	"Adjacent".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Target
{
	#[serde(default)]
	#[serde(default = "default_side")]
	side: String,
	#[serde(default)]
	#[serde(default = "default_range")]
	range: String,
	#[serde(default)]
	#[serde(rename = "self")]
	includes_self: bool,
}

impl Default for Target
{
	fn default() -> Target
	{
		Target
		{
			side: default_side(),
			range: default_range(),
			includes_self: false,
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Attack
{
	name: String,
	internal: Option<String>,
	id: Id,
	description: String,
	element: String,
	category: String,
	#[serde(default)]
	power: PowerType,
	#[serde(default)]
	accuracy: AccuracyType,
	limit: LimitType,
	#[serde(default)]
	critical: bool,
	#[serde(default)]
	target: Target,
}

derive_for_id!(Attack, Id);

#[derive(Debug, Deserialize)]
pub struct AttackFile
{
	pub attack: Attack,
}

impl CodeGenerateGroup for Attack
{
	fn is_valid(group: &HashSet<Attack>) -> BuildResult
	{
		for attack in group
		{
			if attack.target.side != "Enemy" && attack.target.side != "Ally" &&
				attack.target.side != "All"
			{
				return Err(Error::SyntaxError(format!("Invalid attribute 'side' for attack '{}'",
					attack.name)));
			}
			if attack.target.range != "Adjacent" && attack.target.range != "Opposite" &&
				attack.target.range != "All"
			{
				return Err(Error::SyntaxError(format!("Invalid attribute 'range' for attack '{}'",
					attack.name)));
			}
		}
		IdResource::<Id>::sequential(group)
	}
	fn gen_rust_group(group: &HashSet<Attack>, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "`Attack`"));

		try!(writeln!(out,
"use base::attack::{{Attack, target}};
use base::types::attack::AccuracyType;
use gen::element::Element;
use gen::battle::Category;
"));

	try!(IdResource::gen_rust_enum(out, "AttackType", group));

		try!(writeln!(out,
"impl AttackType
{{
	pub fn attack(&self) -> &'static Attack
	{{
		&ATTACK_LIST[*self as usize]
	}}
}}

const ATTACK_LIST: &'static [Attack] = &["));

		for id in 0 as Id..group.len() as Id
		{
			let attack = group.get::<Id>(&id).unwrap();
			try!(writeln!(out, "\tAttack\n\t{{"));

			try!(write!(out, "\t\tname: "));
			try!(write_utf8_escaped(out, &attack.name));
			try!(writeln!(out, ","));

			try!(write!(out, "\t\tdescription: "));
			try!(write_utf8_escaped(out, &attack.description));
			try!(writeln!(out, ","));

			try!(writeln!(out, "\t\telement: Element::{},", attack.element));
			try!(writeln!(out, "\t\tcategory: Category::{},", attack.category));
			try!(writeln!(out, "\t\tpower: {},", attack.power));
			try!(writeln!(out, "\t\taccuracy: {} as AccuracyType,", attack.accuracy));
			try!(writeln!(out, "\t\tlimit: {},", attack.limit));
			try!(writeln!(out, "\t\tcritical: {:?},", attack.critical));

			try!(write!(out, "\t\ttarget: target::SIDE_{} | target::RANGE_{}",
				attack.target.side.to_uppercase(), attack.target.range.to_uppercase()));

			if attack.target.includes_self
			{
				try!(write!(out, "| target::TARGET_SELF"));
			}
			try!(writeln!(out, ","));

			try!(writeln!(out, "\t}},"));
		}

		try!(writeln!(out, "];"));
		Ok(())
	}
	fn gen_constants_group(group: &HashSet<Attack>, out: &mut Write) -> BuildResult
	{
		IdResource::gen_constants(out, "ATTACK", group)
	}
}
