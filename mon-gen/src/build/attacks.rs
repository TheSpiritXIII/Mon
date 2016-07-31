use std::io::Write;
use std::collections::HashSet;

use build::{CodeGenerateGroup, BuildResult, Error};
use build::util::{IdResource, Identifiable, write_disclaimer, write_utf8_escaped};
use types::attack::{AttackId, PowerType, AccuracyType, LimitType, PriorityType};

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
	#[serde(default = "default_side")]
	side: String,
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
	id: AttackId,
	description: String,
	element: String,
	category: String,
	#[serde(default)]
	power: PowerType,
	#[serde(default)]
	accuracy: AccuracyType,
	limit: LimitType,
	#[serde(default)]
	priority: PriorityType,
	#[serde(default)]
	target: Target,
	effect: Option<String>,
}

derive_for_id!(Attack, AttackId);

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
		IdResource::<AttackId>::sequential(group)
	}
	fn gen_rust_group(group: &HashSet<Attack>, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "`AttackMeta`"));

		try!(writeln!(out,
"use base::attack::{{AttackMeta, Target, AccuracyType}};
use base::command::CommandAttack;
use base::effect::Effect;
use base::party::Party;

use calculate::effects::*;

use gen::element::Element;
use gen::battle::Category;

use rand::Rng;
"));

		try!(IdResource::gen_rust_enum(out, "AttackType", group));

		try!(writeln!(out,
"impl AttackType
{{
	pub fn attack(&self) -> &'static AttackMeta
	{{
		&ATTACK_LIST[*self as usize]
	}}
	pub fn effects<'a, R: Rng>(&self, command: &CommandAttack, party: usize, parties: &[Party<'a>],
		effects: &mut Vec<Effect>, rng: &mut R)
	{{
		match *self
		{{"));

		for id in 0 as AttackId..group.len() as AttackId
		{
			let attack = group.get::<AttackId>(&id).unwrap();

			let default_effect = "default_effect".to_string();
			let effect: &String = attack.effect.as_ref().unwrap_or(&default_effect);
			try!(writeln!(out, "\t\t\tAttackType::{} => {}(command, party, parties, effects, rng),",
				Identifiable::identifier(attack), effect));
		}

		try!(writeln!(out,
"		}}
	}}
}}

const ATTACK_LIST: &'static [AttackMeta] = &["));

		for id in 0 as AttackId..group.len() as AttackId
		{
			let attack = group.get::<AttackId>(&id).unwrap();
			try!(writeln!(out, "\tAttackMeta\n\t{{"));

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
			try!(writeln!(out, "\t\tpriority: {:?},", attack.priority));

			try!(write!(out, "\t\ttarget: Target::SIDE_{} | Target::RANGE_{}",
				attack.target.side.to_uppercase(), attack.target.range.to_uppercase()));

			if attack.target.includes_self
			{
				try!(write!(out, "| Target::TARGET_SELF"));
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
