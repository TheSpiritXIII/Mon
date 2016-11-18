use types::monster::StatType;
use base::statmod::StatModifiers;

use types::monster::ExperienceType;

use base::runner::BattleFlagsType;
use base::runner::{BattleEffects, BattleState};

use calculate::lingering::LingeringType;

#[derive(Debug, PartialEq)]
pub enum Effect
{
	Damage(Damage),
	Switch(Switch),
	Retreat(Retreat),
	Modifier(Modifier),
	ExperienceGain(ExperienceGain),
	FlagsChange(FlagsChange),
	LingeringAdd(LingeringAdd),
	LingeringChange(LingeringChange),
	// Status(StatusId),
	// Ability(AbilityId),
	// Miss,
	// ,
	None(NoneReason),
}

#[derive(Debug, PartialEq)]
pub struct Damage
{
	pub party: usize, //
	pub active: usize,
	pub member: usize, //
	pub meta: DamageMeta, //
}

impl Damage
{
	pub fn amount(&self) -> StatType
	{
		self.meta.amount
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn member(&self) -> usize
	{
		self.member
	}
	pub fn critical(&self) -> bool
	{
		self.meta.critical
	}
	pub fn type_bonus(&self) -> f32
	{
		self.meta.type_bonus
	}
}

#[derive(Debug, PartialEq)]
pub struct DamageMeta
{
	pub amount: StatType, //
	pub type_bonus: f32, //
	pub critical: bool, //
	// pub recoil: bool, //
}

#[derive(Debug, PartialEq)]
pub struct Switch
{
	pub party: usize,
	pub member: usize,
	pub target: usize,
}

#[derive(Debug, PartialEq)]
pub struct Retreat
{
	pub party: usize,
	pub active: usize,
}

#[derive(Debug, PartialEq)]
pub struct Modifier
{
	party: usize,
	active: usize,
	modifiers: StatModifiers,
}

impl Modifier
{
	pub fn new(party: usize, active: usize, modifiers: StatModifiers) -> Self
	{
		Modifier
		{
			party: party,
			active: active,
			modifiers: modifiers
		}
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn active(&self) -> usize
	{
		self.active
	}
	pub fn modifiers(&self) -> &StatModifiers
	{
		&self.modifiers
	}
}

#[derive(Debug, PartialEq)]
pub struct ExperienceGain
{
	pub party: usize,
	pub member: usize,
	pub amount: ExperienceType,
	// Original level of the party member.
	pub level: u8,
}

impl ExperienceGain
{
	pub fn new(party: usize, member: usize, amount: ExperienceType, level: u8) -> Self
	{
		ExperienceGain
		{
			party: party,
			member: member,
			amount: amount,
			level: level,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum NoneReason
{
	None,
	Miss,
	Escape,
	Turn,
}

#[derive(Debug, PartialEq)]
pub struct FlagsChange
{
	pub flags: BattleFlagsType,
}

pub trait Lingering
{
	/// Creates an effect. Returns `true` to terminate further effects.
	fn effect(&self, effects: &mut BattleEffects, state: &BattleState) -> bool;

	/// Updates the underlying object's state. Returns `true` to active an effect.
	///
	/// Returns `true` by default.
	///
	fn state_change(&mut self) -> bool
	{
		true
	}

	// TODO: Effects may be activated upon creation.
	/// Activates after the object was created.
	fn after_create(&mut self, state: &BattleState);

	/// Activates at the end of a turn. Returns `true` to change state.
	///
	/// Returns `false` by default.
	///
	fn after_turn(&self) -> bool
	{
		false
	}
}

#[derive(Debug, PartialEq)]
pub struct LingeringAdd
{
	pub lingering: LingeringType,
}

#[derive(Debug, PartialEq)]
pub struct LingeringChange
{
	pub index: usize,
}
