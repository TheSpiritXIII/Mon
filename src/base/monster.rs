use base::species::Species;
use base::types::{LevelType, FormId, PersonalityType, BaseStatType, StatType, EvType, IvType, Gender};
use std::ffi::{CString, CStr};
use libc::c_char;
use rand;
use rand::distributions::{IndependentSample, Range};
use gen::classifiers_gen::Nature;
use gen::calculate_gen::calculate_stats;
use base::gender_ratio::GenderRatio;

pub struct Monster
{
	species: *const Species,
	nick: CString,
	form: FormId,
	level: LevelType,
	personality: PersonalityType,
	gender: Gender,
	nature: Nature,
	//lost_health: StatType,
	stat_health: StatType,
	stat_attack: StatType,
	stat_defense: StatType,
	stat_spattack: StatType,
	stat_spdefense: StatType,
	stat_speed: StatType,
	ev_health: EvType,
	ev_attack: EvType,
	ev_defense: EvType,
	ev_spattack: EvType,
	ev_spdefense: EvType,
	ev_speed: EvType,
	iv_health: IvType,
	iv_attack: IvType,
	iv_defense: IvType,
	iv_spattack: IvType,
	iv_spdefense: IvType,
	iv_speed: IvType,
}

impl Monster
{
	pub fn new(species: *const Species, level: LevelType) -> Option<Self>
	{
		let nick = unsafe
		{
			CStr::from_ptr((*species).name().as_ptr() as *const c_char)
		}
		.to_owned();
		
		let iv_stat = Range::new(0, 32);
		let mut rng = rand::thread_rng();
		
		let mut monster = Monster
		{
			species: species,
			nick: nick,
			form: 0,
			level: level,
			personality: rand::random::<PersonalityType>(),
			gender: GenderRatio::gender(unsafe { (*species).gender_ratio() }, &mut rng),
			nature: rand::random(),
			//lost_health: 0,
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

	pub fn get_species(&self) -> *const Species
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
	
	// fn ability() -> u8;
	
	// fn nature() -> NatureSize;
	
	// fn gender() -> Gender;
	
	// fn experience_total() -> ExperienceType;
	
	// fn experience_add() -> ExperienceType;
	
	
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
		self.stat_health = calculate_stats::calculate_health(&self);
		self.stat_attack = calculate_stats::calculate_attack(&self);
		self.stat_defense = calculate_stats::calculate_defense(&self);
		self.stat_spattack = calculate_stats::calculate_spattack(&self);
		self.stat_spdefense = calculate_stats::calculate_spdefense(&self);
		self.stat_speed = calculate_stats::calculate_speed(&self);
	}
	
	pub fn get_base_health(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_health(self.form)
		}
	}
	
	pub fn get_base_attack(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_attack(self.form)
		}
	}
	
	pub fn get_base_defense(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_defense(self.form)
		}
	}
	
	pub fn get_base_spattack(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_spattack(self.form)
		}
	}
	
	pub fn get_base_spdefense(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_spdefense(self.form)
		}
	}
	
	pub fn get_base_speed(&self) -> BaseStatType
	{
		unsafe
		{
			(*self.species).base_speed(self.form)
		}
	}
	
	pub fn get_ev_health(&self) -> EvType
	{
		self.ev_health
	}
	
	pub fn get_ev_attack(&self) -> EvType
	{
		self.ev_attack
	}
	
	pub fn get_ev_defense(&self) -> EvType
	{
		self.ev_defense
	}
	
	pub fn get_ev_spattack(&self) -> EvType
	{
		self.ev_spattack
	}
	
	pub fn get_ev_spdefense(&self) -> EvType
	{
		self.ev_spdefense
	}
	
	pub fn get_ev_speed(&self) -> EvType
	{
		self.ev_speed
	}
	
	pub fn get_iv_health(&self) -> IvType
	{
		self.iv_health
	}
	
	pub fn get_iv_attack(&self) -> IvType
	{
		self.iv_attack
	}
	
	pub fn get_iv_defense(&self) -> IvType
	{
		self.iv_defense
	}
	
	pub fn get_iv_spattack(&self) -> IvType
	{
		self.iv_spattack
	}
	
	pub fn get_iv_spdefense(&self) -> IvType
	{
		self.iv_spdefense
	}
	
	pub fn get_iv_speed(&self) -> IvType
	{
		self.iv_speed
	}
	
	// fn trainer() -> TrainerSize;
	
	// fn ailment() -> AilmentSize;
}
