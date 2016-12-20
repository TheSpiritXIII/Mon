use std::io::Write;
use std::collections::HashSet;

use build::{CodeGenerate, CodeGenerateGroup, BuildResult, Error};
use build::util::{IdNamePairSet, IdResource, Identifiable, write_disclaimer, write_utf8_escaped};
use types::attack::{AttackId, PowerType, AccuracyType, LimitType, PriorityType, CategoryId};

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
	multi: bool,
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
			multi: false,
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
"use rand::Rng;

use base::attack::{{AttackMeta, Target}};
use base::command::CommandAttack;
use base::runner::{{BattleFlags, BattleState, BattleEffects}};
use calculate::common::*;
use calculate::effects::*;
use calculate::lingering;
use calculate::lingering::LingeringType;
use calculate::modifier;
use gen::attack::Category;
use gen::element::Element;
use types::attack::AccuracyType;

/// An individual action that can be done in `Battle` owned by `Monster`."));

		try!(IdResource::gen_rust_enum(out, "AttackType", group));

		try!(writeln!(out,
"impl AttackType
{{
	/// The meta-data for the given attack.
	pub fn attack(&self) -> &'static AttackMeta
	{{
		&ATTACK_LIST[*self as usize]
	}}

	/// The effect the attack has on the parties in battle.
	///
	/// This function must always append its list of actions in `effects`.
	///
	/// Attacks have endless possibilities in what parties and members they can affect. The given
	/// `command` stores who is using the attack and which target they explicity chose (even if the
	/// attack can target multiple members). The attacking party index is stored as `party` and can
	/// be obtained via `parties`.
	///
	/// Any move that is based on chance must used `rng` in order to be deterministic, as is
	/// necessary in order to replay moves given a seed and a list of party and commands.
	///
	pub fn effects<R: Rng>(&self, effects: &mut BattleEffects, command: &CommandAttack,
		party: usize, state: &BattleState, rng: &mut R)
	{{
		match *self
		{{"));

		for id in 0 as AttackId..group.len() as AttackId
		{
			let attack = group.get::<AttackId>(&id).unwrap();
			let attack_name = Identifiable::identifier(attack);

			let default_effect = "miss_or(data, |data| { damage(data) })".to_string();
			let effect: &String = attack.effect.as_ref().unwrap_or(&default_effect);
			let effect_function = effect.replace("data", "effects, command, party, state, rng");
			writeln!(out, "\t\t\tAttackType::{} => {},", attack_name, effect_function)?;
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
			if attack.target.multi
			{
				try!(write!(out, "| Target::MULTI"));
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

#[derive(Debug, Deserialize)]
pub struct AttackClassifiers
{
	categories: IdNamePairSet<CategoryId>,
}

impl CodeGenerate for AttackClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		IdResource::sequential(&self.categories)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "attack classifiers"));
		IdResource::gen_rust_enum(out, "Category", &self.categories)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		IdResource::gen_constants(out, "ATTACK_CATEGORY", &self.categories)
	}
}
