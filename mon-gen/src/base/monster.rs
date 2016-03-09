//! An instance of a species.
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use rand::{random, thread_rng};
use rand::distributions::{IndependentSample, Range};

use base::types::monster::{LevelType, PersonalityType, StatType, StatIvType};
use base::types::species;
use base::types::species::{FormId, StatBaseType, StatEvType};
use gen::monster::{Nature};
use gen::species_list::SPECIES_LIST;
use gen::gender::{Gender};
use calculate::statistics;

#[derive(PartialEq, PartialOrd, Eq)]
pub struct Monster
{
	species: species::Id,
	nick: CString,
	form: FormId,
	level: LevelType,
	personality: PersonalityType,
	gender: Gender,
	nature: Nature,
	lost_health: StatType,
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
	// caught_method: CatchMethod,
	// caught_location: Location,
}

impl Monster
{
	pub fn new(species: species::Id, level: LevelType) -> Option<Self>
	{
		let nick = unsafe
		{
			CStr::from_ptr((SPECIES_LIST[species as usize]).name.as_ptr() as *const c_char)
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
			gender: Gender::rand(&mut rng, SPECIES_LIST[species as usize].gender),
			nature: random(),
			lost_health: 0,
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
		};
		monster.recalculate_stats();
		Some(monster)
	}

	pub fn get_species(&self) -> species::Id
	{
		self.species
	}

	pub fn get_form(&self) -> FormId
	{
		self.form
	}

	pub fn set_form(&mut self, form: FormId)
	{
		self.form = form;
	}

	pub fn get_nick(&self) -> *const c_char
	{
		self.nick.as_ptr()
	}

	pub fn set_nick(&mut self, nick: *const c_char) -> bool
	{
		let nick = unsafe
		{
			CStr::from_ptr(nick)
		}
		.to_owned();
		if nick.to_bytes().len() > 16
		{
			return false;
		}
		self.nick = nick;
		true
	}

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
// 	// fn experience_total() -> ExperienceType;
//
// 	// fn experience_add() -> ExperienceType;
//
//
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
		self.stat_health = statistics::calculate_health(&self);
		self.stat_attack = statistics::calculate_attack(&self);
		self.stat_defense = statistics::calculate_defense(&self);
		self.stat_spattack = statistics::calculate_spattack(&self);
		self.stat_spdefense = statistics::calculate_spdefense(&self);
		self.stat_speed = statistics::calculate_speed(&self);
	}

	pub fn get_base_health(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_health[self.form as usize]
	}

	pub fn get_base_attack(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_attack[self.form as usize]
	}

	pub fn get_base_defense(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_defense[self.form as usize]
	}

	pub fn get_base_spattack(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_spattack[self.form as usize]
	}

	pub fn get_base_spdefense(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_spdefense[self.form as usize]
	}

	pub fn get_base_speed(&self) -> StatBaseType
	{
		SPECIES_LIST[self.species as usize].base_speed[self.form as usize]
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

// 	// fn trainer() -> TrainerSize;
//
// 	// fn ailment() -> AilmentSize;
}
