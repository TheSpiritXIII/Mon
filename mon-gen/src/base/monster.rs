//! An instance of a species.
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use rand::{random, thread_rng};
use rand::distributions::{IndependentSample, Range};

use types::monster::{LevelType, PersonalityType, StatType, StatIndividualType, ExperienceType};
use types::species::{FormId, StatBaseType, StatYieldType};
use base::util::as_rust_str_from;
use gen::attack_list::AttackType;
use gen::monster::{Nature, RecruitMethod};
use gen::species_list::SpeciesType;
use gen::gender::Gender;
use gen::element::Element;
use calculate::statistics;

/// The limit on the number of attacks a Monster can have.
pub const ATTACK_LIMIT: usize = 4;

pub type LimitUpgradeType = u8;

pub const LIMIT_BOOST: f32 = 0.2;

use types::attack::LimitType;
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
	ev_health: StatYieldType,
	ev_attack: StatYieldType,
	ev_defense: StatYieldType,
	ev_spattack: StatYieldType,
	ev_spdefense: StatYieldType,
	ev_speed: StatYieldType,
	iv_health: StatIndividualType,
	iv_attack: StatIndividualType,
	iv_defense: StatIndividualType,
	iv_spattack: StatIndividualType,
	iv_spdefense: StatIndividualType,
	iv_speed: StatIndividualType,
	attacks: Vec<MonsterAttack>,
	recruited: Option<RecruitMethod>,
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
			experience: species.species().growth.experience_with_level(level),
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
			recruited: None,
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

		monster.stats_recalculate();
		monster.health_restore();
		monster
	}

	pub fn species(&self) -> SpeciesType
	{
		self.species
	}

	pub fn form(&self) -> FormId
	{
		self.form
	}

	pub fn form_set(&mut self, form: FormId)
	{
		assert!(form < self.species().species().forms.len() as FormId);
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

	pub fn nick_raw(&self) -> &[u8]
	{
		self.nick.as_bytes_with_nul()
	}

	pub fn nick_raw_set(&mut self, nick: CString)
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

	pub fn level(&self) -> LevelType
	{
		self.level
	}

	pub fn personality(&self) -> PersonalityType
	{
		self.personality
	}

	pub fn gender(&self) -> Gender
	{
		self.gender
	}

	pub fn nature(&self) -> Nature
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
		if self.level != 100
		{
			let growth = self.species().species().growth;
			self.experience += amount;
			while self.level != 100 &&
				self.experience >= growth.experience_with_level(self.level + 1) 
			{
				self.level += 1;
				if self.level == 100
				{
					self.experience = growth.experience_with_level(100);
				}
			} 
		}
	}

	pub fn stat_health(&self) -> StatType
	{
		self.stat_health
	}

	pub fn stat_attack(&self) -> StatType
	{
		self.stat_attack
	}

	pub fn stat_defense(&self) -> StatType
	{
		self.stat_defense
	}

	pub fn stat_spattack(&self) -> StatType
	{
		self.stat_spattack
	}

	pub fn stat_spdefense(&self) -> StatType
	{
		self.stat_spdefense
	}

	pub fn stat_speed(&self) -> StatType
	{
		self.stat_speed
	}

	pub fn stats_recalculate(&mut self)
	{
		self.stat_health = statistics::calculate_health(self);
		self.stat_attack = statistics::calculate_attack(self);
		self.stat_defense = statistics::calculate_defense(self);
		self.stat_spattack = statistics::calculate_spattack(self);
		self.stat_spdefense = statistics::calculate_spdefense(self);
		self.stat_speed = statistics::calculate_speed(self);
	}

	pub fn base_health(&self) -> StatBaseType
	{
		self.species.species().base_health[self.form as usize]
	}

	pub fn base_attack(&self) -> StatBaseType
	{
		self.species.species().base_attack[self.form as usize]
	}

	pub fn base_defense(&self) -> StatBaseType
	{
		self.species.species().base_defense[self.form as usize]
	}

	pub fn base_spattack(&self) -> StatBaseType
	{
		self.species.species().base_spattack[self.form as usize]
	}

	pub fn base_spdefense(&self) -> StatBaseType
	{
		self.species.species().base_spdefense[self.form as usize]
	}

	pub fn base_speed(&self) -> StatBaseType
	{
		self.species.species().base_speed[self.form as usize]
	}

	pub fn yield_health(&self) -> StatYieldType
	{
		self.ev_health
	}

	pub fn yield_attack(&self) -> StatYieldType
	{
		self.ev_attack
	}

	pub fn yield_defense(&self) -> StatYieldType
	{
		self.ev_defense
	}

	pub fn yield_spattack(&self) -> StatYieldType
	{
		self.ev_spattack
	}

	pub fn yield_spdefense(&self) -> StatYieldType
	{
		self.ev_spdefense
	}

	pub fn yield_speed(&self) -> StatYieldType
	{
		self.ev_speed
	}

	pub fn individual_health(&self) -> StatIndividualType
	{
		self.iv_health
	}

	pub fn individual_attack(&self) -> StatIndividualType
	{
		self.iv_attack
	}

	pub fn individual_defense(&self) -> StatIndividualType
	{
		self.iv_defense
	}

	pub fn individual_spattack(&self) -> StatIndividualType
	{
		self.iv_spattack
	}

	pub fn individual_spdefense(&self) -> StatIndividualType
	{
		self.iv_spdefense
	}

	pub fn individual_speed(&self) -> StatIndividualType
	{
		self.iv_speed
	}

	pub fn health(&self) -> StatType
	{
		self.health
	}

	pub fn health_lose(&mut self, damage: StatType)
	{
		self.health = self.health.saturating_sub(damage);
	}

	pub fn health_gain(&mut self, gain: StatType)
	{
		self.health += gain;
		if self.health > self.stat_health
		{
			self.health = self.stat_health;
		}
	}

	pub fn health_restore(&mut self)
	{
		self.health = self.stat_health;
	}

	pub fn attacks(&self) -> &[MonsterAttack]
	{
		self.attacks.as_slice()
	}

	pub fn attacks_mut(&mut self) -> &mut [MonsterAttack]
	{
		self.attacks.as_mut_slice()
	}

	pub fn attack_remove(&mut self, index: usize)
	{
		self.attacks.remove(index);
	}

	pub fn recruited(&self) -> bool
	{
		self.recruited.is_some()
	}

	pub fn recruit_method(&self) -> RecruitMethod
	{
		self.recruited.unwrap()
	}

	// pub fn recruit_location()

	// pub fn recruit_set(method, location);

// 	// fn trainer() -> TrainerType;
//
// 	// fn ailment() -> AilmentType;
}
