//! An instance of a species.
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use rand::{random, thread_rng};
use rand::distributions::{IndependentSample, Range};

use base::types::monster::{LevelType, PersonalityType, StatType, StatIvType, ExperienceType};
use base::types::species::{FormId, StatBaseType, StatEvType};
use base::util::as_rust_str_from;
use gen::attack_list::AttackType;
use gen::monster::Nature;
use gen::species_list::SpeciesType;
use gen::gender::Gender;
use gen::element::Element;
use calculate::statistics;

/// The limit on the number of attacks a Monster can have.
pub const ATTACK_LIMIT: usize = 4;

pub type LimitUpgradeType = u8;

pub const LIMIT_BOOST: f32 = 0.2;

use base::types::attack::LimitType;
use base::attack::AttackMeta;

#[derive(Debug)]
pub struct MonsterAttack
{
	attack_type: AttackType,
	limit_left: LimitType,
	limit_upgraded: LimitUpgradeType,
}

impl MonsterAttack
{
	fn new(attack_type: AttackType) -> Self
	{
		MonsterAttack
		{
			attack_type: attack_type,
			limit_left: attack_type.attack().limit,
			limit_upgraded: 0,
		}
	}
	pub fn attack(&self) -> &'static AttackMeta
	{
		self.attack_type.attack()
	}
	pub fn attack_type(&self) -> AttackType
	{
		self.attack_type
	}
	pub fn limit_left_take(&mut self, amount: LimitType)
	{
		self.limit_left = self.limit_left.saturating_sub(amount);
	}
	pub fn limit_left(&self) -> LimitType
	{
		self.limit_left
	}
	pub fn limit_max(&self) -> LimitType
	{
		let limit_bonus = self.limit_upgraded as f32 * LIMIT_BOOST;
		((self.attack_type.attack().limit as f32) + limit_bonus).floor() as LimitType
	}
}

/// An instance of a species.
#[derive(Debug)]
pub struct Monster
{
	species: SpeciesType,
	nick: CString,
	form: FormId,
	level: LevelType,
	personality: PersonalityType,
	gender: Gender,
	nature: Nature,
	experience: ExperienceType,
	health: StatType,
	stat_health: StatType,
	stat_attack: StatType,
	stat_defense: StatType,
	stat_spattack: StatType,
	stat_spdefense: StatType,
	stat_speed: StatType,
	ev_health: StatEvType,
	ev_attack: StatEvType,
	ev_defense: StatEvType,
	ev_spattack: StatEvType,
	ev_spdefense: StatEvType,
	ev_speed: StatEvType,
	iv_health: StatIvType,
	iv_attack: StatIvType,
	iv_defense: StatIvType,
	iv_spattack: StatIvType,
	iv_spdefense: StatIvType,
	iv_speed: StatIvType,
	attacks: Vec<MonsterAttack>,
	// caught_method: CatchMethod,
	// caught_location: Location,
}

impl Monster
{
	pub fn new(species: SpeciesType, level: LevelType) -> Self
	{
		let nick = unsafe
		{
			CStr::from_ptr(species.species().name.as_ptr() as *const c_char)
		}
		.to_owned();

		let iv_stat = Range::new(0, 32);
		let mut rng = thread_rng();

		let mut monster = Monster
		{
			species: species,
			nick: nick,
			form: 0,
			level: level,
			personality: random(),
			gender: Gender::rand(&mut rng, species.species().gender),
			nature: random(),
			experience: 0,
			health: 0,
			stat_health: 0,
			stat_attack: 0,
			stat_defense: 0,
			stat_spattack: 0,
			stat_spdefense: 0,
			stat_speed: 0,
			ev_health: 0,
			ev_attack: 0,
			ev_defense: 0,
			ev_spattack: 0,
			ev_spdefense: 0,
			ev_speed: 0,
			iv_health: iv_stat.ind_sample(&mut rng),
			iv_attack: iv_stat.ind_sample(&mut rng),
			iv_defense: iv_stat.ind_sample(&mut rng),
			iv_spattack: iv_stat.ind_sample(&mut rng),
			iv_spdefense: iv_stat.ind_sample(&mut rng),
			iv_speed: iv_stat.ind_sample(&mut rng),
			attacks: Vec::new(),
		};

		// The index at which the level is closest to.
		let mut attack_level_index = species.species().attacks_learnable.binary_search_by(
			|&(level, _)|
		{
			level.cmp(&monster.level)
		})
		.unwrap_or_else(|index| index - 1);

		// The index at which the monster's attack list is filled.
		let mut attack_filled_index = 0;
		'outer: loop
		{
			let (_, ref attacks_forms) = species.species().attacks_learnable[attack_level_index];
			let attack_list = attacks_forms[monster.form as usize];
			for attack in attack_list
			{
				monster.attacks.insert(0, MonsterAttack::new(*attack));
				attack_filled_index += 1;
				if attack_filled_index == ATTACK_LIMIT
				{
					break 'outer;
				}
			}

			if attack_level_index == 0
			{
				break;
			}

			attack_level_index -= 1;
		}

		monster.recalculate_stats();
		monster.restore_health();
		monster
	}

	pub fn get_species(&self) -> SpeciesType
	{
		self.species
	}

	pub fn get_form(&self) -> FormId
	{
		self.form
	}

	pub fn set_form(&mut self, form: FormId)
	{
		assert!(form < self.get_species().species().forms.len() as FormId);
		self.form = form;
	}

	pub fn get_elements(&self) -> &'static [Element]
	{
		self.species.species().elements[self.form as usize]
	}

	pub fn nick(&self) -> &str
	{
		let string = self.nick.as_bytes_with_nul();
		as_rust_str_from(string)
	}

	pub fn get_nick_raw(&self) -> &[u8]
	{
		self.nick.as_bytes_with_nul()
	}

	pub fn set_nick_raw(&mut self, nick: CString)
	{
		self.nick = nick;
	}
	// pub fn set_nick_raw(&mut self, nick: *const c_char) -> bool
	// {
	// 	// TODO: Validate safe utf8.
	// 	let nick = unsafe
	// 	{
	// 		CStr::from_ptr(nick)
	// 	}
	// 	.to_owned();
	// 	if nick.to_bytes().len() > 16
	// 	{
	// 		return false;
	// 	}
	// 	self.nick = nick;
	// 	true
	// }

	pub fn get_level(&self) -> LevelType
	{
		self.level
	}

	pub fn get_personality(&self) -> PersonalityType
	{
		self.personality
	}

	pub fn get_gender(&self) -> Gender
	{
		self.gender
	}

	pub fn get_nature(&self) -> Nature
	{
		self.nature
	}
//
// 	// fn ability() -> u8;
//
// 	// fn nature() -> NatureSize;
//
// 	// fn gender() -> Gender;
// 
	pub fn experience_total(&self) -> ExperienceType
	{
		self.experience
	}

	pub fn experience_add(&mut self, amount: ExperienceType)
	{
		self.experience += amount;
		
		// TODO: Actually level up.
	}

	pub fn get_stat_health(&self) -> StatType
	{
		self.stat_health
	}

	pub fn get_stat_attack(&self) -> StatType
	{
		self.stat_attack
	}

	pub fn get_stat_defense(&self) -> StatType
	{
		self.stat_defense
	}

	pub fn get_stat_spattack(&self) -> StatType
	{
		self.stat_spattack
	}

	pub fn get_stat_spdefense(&self) -> StatType
	{
		self.stat_spdefense
	}

	pub fn get_stat_speed(&self) -> StatType
	{
		self.stat_speed
	}

	pub fn recalculate_stats(&mut self)
	{
		self.stat_health = statistics::calculate_health(self);
		self.stat_attack = statistics::calculate_attack(self);
		self.stat_defense = statistics::calculate_defense(self);
		self.stat_spattack = statistics::calculate_spattack(self);
		self.stat_spdefense = statistics::calculate_spdefense(self);
		self.stat_speed = statistics::calculate_speed(self);
	}

	pub fn get_base_health(&self) -> StatBaseType
	{
		self.species.species().base_health[self.form as usize]
	}

	pub fn get_base_attack(&self) -> StatBaseType
	{
		self.species.species().base_attack[self.form as usize]
	}

	pub fn get_base_defense(&self) -> StatBaseType
	{
		self.species.species().base_defense[self.form as usize]
	}

	pub fn get_base_spattack(&self) -> StatBaseType
	{
		self.species.species().base_spattack[self.form as usize]
	}

	pub fn get_base_spdefense(&self) -> StatBaseType
	{
		self.species.species().base_spdefense[self.form as usize]
	}

	pub fn get_base_speed(&self) -> StatBaseType
	{
		self.species.species().base_speed[self.form as usize]
	}

	pub fn get_ev_health(&self) -> StatEvType
	{
		self.ev_health
	}

	pub fn get_ev_attack(&self) -> StatEvType
	{
		self.ev_attack
	}

	pub fn get_ev_defense(&self) -> StatEvType
	{
		self.ev_defense
	}

	pub fn get_ev_spattack(&self) -> StatEvType
	{
		self.ev_spattack
	}

	pub fn get_ev_spdefense(&self) -> StatEvType
	{
		self.ev_spdefense
	}

	pub fn get_ev_speed(&self) -> StatEvType
	{
		self.ev_speed
	}

	pub fn get_iv_health(&self) -> StatIvType
	{
		self.iv_health
	}

	pub fn get_iv_attack(&self) -> StatIvType
	{
		self.iv_attack
	}

	pub fn get_iv_defense(&self) -> StatIvType
	{
		self.iv_defense
	}

	pub fn get_iv_spattack(&self) -> StatIvType
	{
		self.iv_spattack
	}

	pub fn get_iv_spdefense(&self) -> StatIvType
	{
		self.iv_spdefense
	}

	pub fn get_iv_speed(&self) -> StatIvType
	{
		self.iv_speed
	}

	pub fn get_health(&self) -> StatType
	{
		self.health
	}

	pub fn lose_health(&mut self, damage: StatType)
	{
		self.health = self.health.saturating_sub(damage);
	}

	pub fn gain_health(&mut self, gain: StatType)
	{
		self.health += gain;
		if self.health > self.stat_health
		{
			self.health = self.stat_health;
		}
	}

	pub fn restore_health(&mut self)
	{
		self.health = self.stat_health;
	}

	pub fn get_attacks(&self) -> &[MonsterAttack]
	{
		self.attacks.as_slice()
	}

	pub fn get_attacks_mut(&mut self) -> &mut [MonsterAttack]
	{
		self.attacks.as_mut_slice()
	}

	pub fn remove_attack(&mut self, index: usize)
	{
		self.attacks.remove(index);
	}

// 	// fn trainer() -> TrainerType;
//
// 	// fn ailment() -> AilmentType;
}

// Validate that every species has properties for each of their forms.
#[test]
fn validate_species_forms()
{
	for species in 0..SpeciesType::count()
	{
		let forms = SpeciesType::from_id(species).forms.len();
		assert_eq!(forms > 0, true);
		assert_eq!(SpeciesType::from_id(species).elements.len(), forms);
		assert_eq!(SpeciesType::from_id(species).height.len(), forms);
		assert_eq!(SpeciesType::from_id(species).weight.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_health.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_attack.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_defense.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_spattack.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_spdefense.len(), forms);
		assert_eq!(SpeciesType::from_id(species).base_speed.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_health.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_attack.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_defense.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_spattack.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_spdefense.len(), forms);
		assert_eq!(SpeciesType::from_id(species).yield_speed.len(), forms);
	}
}
