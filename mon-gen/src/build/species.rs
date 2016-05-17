// Generates static data for specific species, as well as species classifiers.
use std::io::Write;
use std::collections::{HashSet, HashMap};

use types::species::{Id, FormId, MetricType, StatBaseType, StatEvType, ExperienceGrowthType,
	FriendshipType, GrowthId, RarenessType, HatchType, ColorId, HabitatId};
use types::monster::LevelType;

use build::{BuildResult, CodeGenerate, CodeGenerateGroup, Error};
use util::{IdNamePairSet, IdResource, Identifiable, write_disclaimer, write_utf8_escaped};

trait HasForm
{
	fn form(&self) -> &String;
}

trait CustomDisplay
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult;
}

fn form_map_order<'a, T: HasForm>(changes: &'a Vec<T>, who: &str, attribute: &str,
	form_map: &HashMap<&String, FormId>) -> Result<Vec<Option<&'a T>>, Error>
{
	let mut value_list: Vec<Option<&T>> = vec![None; changes.len()];
	for change in changes
	{
		match form_map.get(&change.form())
		{
			Some(form_index) =>
			{
				value_list[*form_index as usize] = Some(change);
			}
			None =>
			{
				return Err(Error::SyntaxError(format!(
					"Unknown form `{}` in attribute `{}` for `{}`.", change.form(), attribute,
					who)));
			}
		}
	}
	Ok(value_list)
}

impl CustomDisplay
{
	fn write_list<T, F>(out: &mut Write, who: &str, attribute: &str, prefix: &str,
		postfix: &str, forms: &IdNamePairSet<FormId>, value_list: &Vec<Option<&T>>,
		func: F) -> BuildResult where T: HasForm, F: Fn(&T, &mut Write, &str, &str) -> BuildResult
	{
		// for index in (0 as FormId)..(forms.len() as FormId)
		for (index, value_option) in value_list.iter().enumerate()
		{
			match *value_option
			{
				Some(ref value) =>
				{
					try!(func(value, out, prefix, postfix))
				}
				None =>
				{
					let i = index as FormId;
					return Err(Error::SyntaxError(format!(
						"Missing value for form `{}` in attribute `{}` for `{}`.",
						forms.get(&i).unwrap().name(), attribute, who)));
				}
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesFormValue<T> where T: CustomDisplay
{
	#[serde(default)]
	form: String,
	value: T,
}

impl<T> HasForm for SpeciesFormValue<T> where T: CustomDisplay
{
	fn form(&self) -> &String
	{
		&self.form
	}
}

impl CustomDisplay for MetricType
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		write!(out, "{}{}{}, ", prefix, self, postfix).map_err(|e| Error::IoError(e))
	}
}

impl CustomDisplay for String
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		write!(out, "{}{}{}, ", prefix, self, postfix).map_err(|e| Error::IoError(e))
	}
}

impl<T> CustomDisplay for Vec<T> where T: CustomDisplay
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		try!(write!(out, "&["));
		for i in self
		{
			try!(i.write_value(out, prefix, postfix));
		}
		try!(write!(out, "], "));
		Ok(())
	}
}

impl<T> CustomDisplay for SpeciesFormValue<T> where T: CustomDisplay
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		T::write_value(&self.value, out, prefix, postfix)
	}
}

#[derive(Debug, Deserialize)]
enum SpeciesFormChange<T> where T: CustomDisplay
{
	Change(Vec<SpeciesFormValue<T>>),
	NoChange(T),
}

impl<T> SpeciesFormChange<T> where T: Clone + CustomDisplay
{
	fn write(&self, out: &mut Write, who: &str, attribute: &str, prefix: &str, postfix: &str,
		forms: &IdNamePairSet<FormId>, form_map: &HashMap<&String, FormId>) -> BuildResult
	{
		return match *self
		{
			SpeciesFormChange::Change(ref changes) =>
			{
				let value_list = try!(form_map_order(changes, who, attribute, form_map));
				CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms, &value_list,
					|value, out, prefix, postfix|
				{
					value.write_value(out, prefix, postfix)
				})
			}
			SpeciesFormChange::NoChange(ref value) =>
			{
				value.write_value(out, prefix, postfix)
			}
		};
	}
}

#[derive(Debug, Deserialize)]
struct SpeciesEvolution
{
	level: LevelType,
	species: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SpeciesAbilities
{
	default: Vec<String>,
	hidden: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesStatisticsGroup<T> where T: Clone
{
	#[serde(default)]
	form: String,
	health: T,
	attack: T,
	defense: T,
	#[serde(rename = "sp-attack")]
	sp_attack: T,
	#[serde(rename = "sp-defense")]
	sp_defense: T,
	speed: T,
}

enum Statistic
{
	Health,
	Attack,
	Defense,
	SpAttack,
	SpDefense,
	Speed,
}

impl<T> HasForm for SpeciesStatisticsGroup<T> where T: Clone
{
	fn form(&self) -> &String
	{
		&self.form
	}
}

impl CustomDisplay for u8
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		write!(out, "{}{}{}, ", prefix, self, postfix).map_err(|e| Error::IoError(e))
	}
}

impl<T> CustomDisplay for Option<T> where T: CustomDisplay
{
	fn write_value(&self, out: &mut Write, prefix: &str, postfix: &str) -> BuildResult
	{
		match *self
		{
			Some(ref value) => CustomDisplay::write_value(value, out, prefix, postfix),
			None => write!(out, "{}0{}, ", prefix, postfix).map_err(|e| Error::IoError(e)),
		}
	}
}

#[derive(Debug, Deserialize)]
enum SpeciesStatisticsValue<T> where T: Clone + CustomDisplay
{
	FormChange(Vec<SpeciesStatisticsGroup<T>>),
	NoChange(SpeciesStatisticsGroup<T>),
}

impl<T> SpeciesStatisticsValue<T> where T: Clone + CustomDisplay
{
	fn write(&self, out: &mut Write, who: &str, attribute: &str, prefix: &str, postfix: &str,
		forms: &IdNamePairSet<FormId>, value_list: &Vec<Option<&SpeciesStatisticsGroup<T>>>,
		stat: Statistic) -> BuildResult
	{
		return match *self
		{
			SpeciesStatisticsValue::FormChange(_) =>
			{
				match stat
				{
					Statistic::Health =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.health, out, prefix, postfix)
						})
					}
					Statistic::Attack =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.attack, out, prefix, postfix)
						})
					}
					Statistic::Defense =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.defense, out, prefix, postfix)
						})
					}
					Statistic::SpAttack =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.sp_attack, out, prefix, postfix)
						})
					}
					Statistic::SpDefense =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.sp_defense, out, prefix, postfix)
						})
					}
					Statistic::Speed =>
					{
						CustomDisplay::write_list(out, who, attribute, prefix, postfix, forms,
							&value_list, |value, out, prefix, postfix|
						{
							CustomDisplay::write_value(&value.speed, out, prefix, postfix)
						})
					}
				}
			}
			SpeciesStatisticsValue::NoChange(ref value) =>
			{
				match stat
				{
					Statistic::Health =>
					{
						CustomDisplay::write_value(&value.health, out, prefix, postfix)
					}
					Statistic::Attack =>
					{
						CustomDisplay::write_value(&value.attack, out, prefix, postfix)
					}
					Statistic::Defense =>
					{
						CustomDisplay::write_value(&value.defense, out, prefix, postfix)
					}
					Statistic::SpAttack =>
					{
						CustomDisplay::write_value(&value.sp_attack, out, prefix, postfix)
					}
					Statistic::SpDefense =>
					{
						CustomDisplay::write_value(&value.sp_defense, out, prefix, postfix)
					}
					Statistic::Speed =>
					{
						CustomDisplay::write_value(&value.speed, out, prefix, postfix)
					}
				}
			}
		};
	}
}

#[derive(Debug, Deserialize)]
struct SpeciesStatistics
{
	base: SpeciesStatisticsValue<StatBaseType>,
	#[serde(rename = "yield")]
	ev_yield: SpeciesStatisticsValue<Option<StatEvType>>,
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesFormAttack
{
	form: Option<String>,
	attacks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
enum SpeciesAttacks
{
	Attacks(Vec<String>),
	FormAttacks(Vec<SpeciesFormAttack>)
}

#[derive(Debug, Clone, Deserialize)]
struct SpeciesLearnableAttack
{
	level: LevelType,
	attacks: SpeciesAttacks,
}

#[derive(Debug, Deserialize)]
struct SpeciesAttacksList
{
	learnable: Vec<SpeciesLearnableAttack>,
	#[serde(default)]
	inheritable: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Species
{
	name: String,
	internal: Option<String>,
	id: Id,
	description: String,
	kind: String,
	#[serde(default)]
	forms: IdNamePairSet<FormId>,
	elements: SpeciesFormChange<Vec<String>>,
	experience: ExperienceGrowthType,
	height: SpeciesFormChange<MetricType>,
	weight: SpeciesFormChange<MetricType>,
	rareness: RarenessType,
	friendship: FriendshipType,
	hatch: HatchType,
	groups: Vec<String>,
	gender: String,
	growth: String,
	color: String,
	habitat: String,
	#[serde(default)]
	evolutions: Vec<SpeciesEvolution>,
	abilities: SpeciesAbilities,
	statistics: SpeciesStatistics,
	attacks: SpeciesAttacksList,
}

derive_for_id!(Species, Id);

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct SpeciesFile
{
	pub species: Species,
}

impl CodeGenerateGroup for Species
{
	fn is_valid(group: &HashSet<Species>) -> BuildResult
	{
		for species in group
		{
			let mut valid = false;
			for attack in &species.attacks.learnable
			{
				if attack.level == 1
				{
					valid = true;
					break;
				}
			}
			if !valid
			{
				return Err(Error::SyntaxError(format!("Invalid learnable attacks for species `{}`.
					Must have an attack learnable at level 1.", species.name).to_string()));
			}
		}
		IdResource::<Id>::sequential(group)
	}
	fn gen_rust_group(group: &HashSet<Species>, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "static species data"));
		try!(writeln!(out,
"use base::species::Species;
use base::types::species::{{Id, MetricType}};
use gen::element::Element;
use gen::gender::GenderRatio;
use gen::species::{{Growth, Color, Habitat, Group}};
use gen::attack_list::AttackType;
"));

		try!(IdResource::<Id>::gen_rust_enum(out, "SpeciesType", group));

		for species in group
		{
			if species.forms.len() != 0
			{
				try!(IdResource::<FormId>::gen_rust_enum(out,
					format!("{}Form", Identifiable::identifier(species)).as_str(), &species.forms));
			}
		}

		try!(writeln!(out,
"impl SpeciesType
{{
	pub fn species(&self) -> &'static Species
	{{
		&SPECIES_LIST[*self as usize]
	}}
	pub fn from_id(id: Id) -> &'static Species
	{{
		&SPECIES_LIST[id as usize]
	}}
	pub fn count() -> Id
	{{
		SPECIES_LIST.len() as Id
	}}
}}

const SPECIES_LIST: &'static [Species] = &["));

		for id in 0 as Id..group.len() as Id
		{
			let species = group.get::<Id>(&id).unwrap();
			try!(writeln!(out, "\tSpecies\n\t{{"));

			try!(write!(out, "\t\tname: "));
			try!(write_utf8_escaped(out, &species.name));
			try!(writeln!(out, ","));

			try!(write!(out, "\t\tdescription: "));
			try!(write_utf8_escaped(out, &species.description));
			try!(writeln!(out, ","));

			try!(write!(out, "\t\tkind: "));
			try!(write_utf8_escaped(out, &species.kind));
			try!(writeln!(out, ","));

			try!(writeln!(out, "\t\tgender: GenderRatio::{},", species.gender));
			try!(writeln!(out, "\t\tgrowth: Growth::{},", species.growth));
			try!(writeln!(out, "\t\tcolor: Color::{},", species.color));
			try!(writeln!(out, "\t\thabitat: Habitat::{},", species.habitat));
			try!(writeln!(out, "\t\trareness: {},", species.rareness));
			try!(writeln!(out, "\t\tfriendship: {},", species.friendship));
			try!(writeln!(out, "\t\thatch: {},", species.hatch));

			try!(write!(out, "\t\tforms: &["));
			let mut form_map: HashMap<&String, FormId> = HashMap::new();
			for form in &species.forms
			{
				try!(write_utf8_escaped(out, &form.name()));
				try!(write!(out, ", "));
				form_map.insert(Identifiable::identifier(form), form.id());
			}
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\telements: &["));
			try!(species.elements.write(out, &species.name, "elements", "Element::", "",
				&species.forms, &form_map));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tgroups: &["));
			for group in &species.groups
			{
				try!(write!(out, "Group::{}, ", group));
			}
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\theight: &["));
			try!(species.height.write(out, &species.name, "height", "", " as MetricType",
				&species.forms, &form_map));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tweight: &["));
			try!(species.weight.write(out, &species.name, "weight", "", " as MetricType",
				&species.forms, &form_map));
			try!(writeln!(out, "],"));

			let mut base_stat_list = Vec::new();
			if let &SpeciesStatisticsValue::FormChange(ref base) = &species.statistics.base
			{
				base_stat_list = try!(form_map_order(&base, "statistics.base", "", &form_map));
			}

			try!(write!(out, "\t\tbase_health: &["));
			try!(species.statistics.base.write(out, &species.name, "base_health", "", "",
				&species.forms, &base_stat_list, Statistic::Health));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tbase_attack: &["));
			try!(species.statistics.base.write(out, &species.name, "base_attack", "", "",
				&species.forms, &base_stat_list, Statistic::Attack));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tbase_defense: &["));
			try!(species.statistics.base.write(out, &species.name, "base_defense", "", "",
				&species.forms, &base_stat_list, Statistic::Defense));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tbase_spattack: &["));
			try!(species.statistics.base.write(out, &species.name, "base_sp_attack", "", "",
				&species.forms, &base_stat_list, Statistic::SpAttack));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tbase_spdefense: &["));
			try!(species.statistics.base.write(out, &species.name, "base_sp_defense", "", "",
				&species.forms, &base_stat_list, Statistic::SpDefense));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tbase_speed: &["));
			try!(species.statistics.base.write(out, &species.name, "base_speed", "", "",
				&species.forms, &base_stat_list, Statistic::Speed));
			try!(writeln!(out, "],"));

			let mut yield_stat_list = Vec::new();
			if let &SpeciesStatisticsValue::FormChange(ref base) = &species.statistics.ev_yield
			{
				yield_stat_list = try!(form_map_order(&base, "statistics.yield", "", &form_map));
			}

			try!(write!(out, "\t\tyield_health: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_health", "", "",
				&species.forms, &yield_stat_list, Statistic::Health));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tyield_attack: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_attack", "", "",
				&species.forms, &yield_stat_list, Statistic::Attack));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tyield_defense: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_defense", "", "",
				&species.forms, &yield_stat_list, Statistic::Defense));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tyield_spattack: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_sp_attack", "", "",
				&species.forms, &yield_stat_list, Statistic::SpAttack));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tyield_spdefense: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_sp_defense", "", "",
				&species.forms, &yield_stat_list, Statistic::SpDefense));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tyield_speed: &["));
			try!(species.statistics.ev_yield.write(out, &species.name, "yield_speed", "", "",
				&species.forms, &yield_stat_list, Statistic::Speed));
			try!(writeln!(out, "],"));

			try!(write!(out, "\t\tattacks_learnable: &["));
			let mut attacks_learnable = species.attacks.learnable.clone();
			attacks_learnable.sort_by_key(|attack_list| attack_list.level);
			for attack in &attacks_learnable
			{
				try!(write!(out, "({}, ", attack.level));
				match attack.attacks
				{
					SpeciesAttacks::Attacks(ref attacks) =>
					{
						try!(write!(out, "&[&["));
						for _ in attacks
						{
							// TODO: Enable attacks when have attacks.
							// try!(write!(out, "AttackType::{}, ", attack));
							try!(write!(out, "AttackType::Pound, "));
						}
						try!(write!(out, "]]"));
					}
					SpeciesAttacks::FormAttacks(ref form_attacks) =>
					{
						let mut form_index_attack: HashMap<u8, &Vec<String>> = HashMap::new();
						let mut default_attack: Option<&Vec<String>> = None;
						for form_attack in form_attacks
						{
							match form_attack.form
							{
								Some(ref form) =>
								{
									if let Some(index) = form_map.get(form)
									{
										form_index_attack.insert(*index, &form_attack.attacks);
									}
									else
									{
										return Err(Error::SyntaxError(format!(
											"Invalid form `{}` at level {} for `{}`.", form,
											attack.level, species.name)))
									}
								}
								None =>
								{
									if let Some(_) = default_attack
									{
										return Err(Error::SyntaxError(format!(
											"Only 1 default attack list is allowed at level {} \
											for `{}`.", attack.level, species.name)))
									}
									else
									{
										default_attack = Some(&form_attack.attacks);
									}
								}
							}
						}
						try!(write!(out, "&["));
						for i in 0..form_map.len() as u8
						{
							if let Some(attacks) = form_index_attack.get(&i)
							{
								try!(write!(out, "&["));
								for _ in *attacks
								{
									// TODO: Enable attacks when have attacks.
									// try!(write!(out, "AttackType::{}, ", attack));
									try!(write!(out, "AttackType::Pound, "));
								}
								try!(write!(out, "], "));
							}
							else
							{
								try!(write!(out, "&["));
								for _ in default_attack
								{
									// TODO: Enable attacks when have attacks.
									// try!(write!(out, "AttackType::{}, ", attack));
									try!(write!(out, "AttackType::Pound, "));
								}
								try!(write!(out, "], "));
							}
						}
						try!(write!(out, "], "));
					}
				}
				try!(write!(out, "), "));
			}
			try!(writeln!(out, "],"));

			try!(writeln!(out, "\t}},"));
		}

		try!(writeln!(out, "];"));
		Ok(())
	}
	fn gen_constants_group(group: &HashSet<Species>, out: &mut Write) -> BuildResult
	{
		for id in group
		{
			let prefix = "SPECIES";
			let ident_capital = Identifiable::identifier(id).to_uppercase();
			try!(writeln!(out, "MON_{}_{}={}", prefix, ident_capital, id.id()));
			for form_id in &id.forms
			{
				let form_ident_capital = Identifiable::identifier(form_id).to_uppercase();
				try!(writeln!(out, "MON_{}_{}_FORM_{}={}", prefix, ident_capital,
					form_ident_capital, form_id.id()));
			}
		}
		Ok(())
	}
}

#[derive(Debug, Deserialize)]
pub struct SpeciesClassifiers
{
	growth: IdNamePairSet<GrowthId>,
	groups: IdNamePairSet<GrowthId>,
	colors: IdNamePairSet<ColorId>,
	habitats: IdNamePairSet<HabitatId>,
}

impl CodeGenerate for SpeciesClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		try!(IdResource::sequential(&self.growth));
		try!(IdResource::sequential(&self.groups));
		try!(IdResource::sequential(&self.colors));
		IdResource::sequential(&self.habitats)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "species classifiers"));
		try!(IdResource::gen_rust_enum(out, "Growth", &self.growth));
		try!(IdResource::gen_rust_enum(out, "Group", &self.groups));
		try!(IdResource::gen_rust_enum(out, "Color", &self.colors));
		IdResource::gen_rust_enum(out, "Habitat", &self.habitats)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		try!(IdResource::gen_constants(out, "GROWTH", &self.growth));
		try!(IdResource::gen_constants(out, "GROUP", &self.groups));
		try!(IdResource::gen_constants(out, "COLOR", &self.colors));
		IdResource::gen_constants(out, "HABITAT", &self.habitats)
	}
}
