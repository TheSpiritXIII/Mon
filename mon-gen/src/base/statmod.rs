use base::types::battle::StatModifierType;
use base::types::attack::AccuracyType;

use std::cmp;

#[derive(Debug)]
pub struct StatModifiers
{
	attack: StatModifierType,
	defense: StatModifierType,
	sp_attack: StatModifierType,
	sp_defense: StatModifierType,
	speed: StatModifierType,
	accuracy: StatModifierType,
	evasion: StatModifierType,
	critical: StatModifierType,
}

fn clamp<T>(value: T, min: T, max: T) -> T where T: Ord
{
	cmp::min(max, cmp::max(value, min))
}

impl StatModifiers
{
	pub fn new() -> StatModifiers
	{
		StatModifiers
		{
			attack: 0,
			defense: 0,
			sp_attack: 0,
			sp_defense: 0,
			speed: 0,
			accuracy: 0,
			evasion: 0,
			critical: 0,
		}
	}
	pub fn apply(&mut self, modifiers: &StatModifiers)
	{
		self.attack_delta(modifiers.attack);
		self.defense = clamp::<StatModifierType>(self.defense + modifiers.defense, -6, 6);
		self.sp_attack = clamp::<StatModifierType>(self.sp_attack + modifiers.sp_attack, -6, 6);
		self.sp_defense = clamp::<StatModifierType>(self.sp_defense + modifiers.sp_defense, -6, 6);
		self.speed = clamp::<StatModifierType>(self.speed + modifiers.speed, -6, 6);
		self.accuracy = clamp::<StatModifierType>(self.accuracy + modifiers.accuracy, -6, 6);
		self.evasion = clamp::<StatModifierType>(self.evasion + modifiers.evasion, -6, 6);
		self.critical = cmp::min(0, self.critical + modifiers.critical);
	}
	fn base_value(stage: StatModifierType) -> AccuracyType
	{
		match stage
		{
			-6 => 2.0 / 8.0,
			-5 => 2.0 / 7.0,
			-4 => 2.0 / 6.0,
			-3 => 2.0 / 5.0,
			-2 => 2.0 / 4.0,
			-1 => 2.0 / 3.0,
			0 =>  2.0 / 2.0,
			1 =>  3.0 / 2.0,
			2 =>  4.0 / 2.0,
			3 =>  5.0 / 2.0,
			4 =>  6.0 / 2.0,
			5 =>  7.0 / 2.0,
			6 =>  8.0 / 2.0,
			_ =>
			{
				unreachable!();
			}
		}
	}
	pub fn attack_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.attack)
	}
	// fn attack_stage(&self) -> StatModifier
	// {
	// 	self.attack
	// }
	pub fn attack_delta(&mut self, delta: StatModifierType)
	{
		self.attack = clamp::<StatModifierType>(self.attack + delta, -6, 6);
	}
	pub fn defense_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.defense)
	}
	pub fn sp_attack_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.attack)
	}
	pub fn sp_defense_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.defense)
	}
	pub fn speed_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.speed)
	}
	fn evasion_accuracy_value(stage: StatModifierType) -> AccuracyType
	{
		match stage
		{
			-6 => 3.0 / 9.0,
			-5 => 3.0 / 8.0,
			-4 => 3.0 / 7.0,
			-3 => 3.0 / 6.0,
			-2 => 3.0 / 5.0,
			-1 => 3.0 / 4.0,
			0 =>  3.0 / 3.0,
			1 =>  4.0 / 3.0,
			2 =>  5.0 / 3.0,
			3 =>  6.0 / 3.0,
			4 =>  7.0 / 3.0,
			5 =>  8.0 / 3.0,
			6 =>  9.0 / 3.0,
			_ =>
			{
				unreachable!();
			}
		}
	}
	pub fn evasion_stage(&self) -> StatModifierType
	{
		self.evasion
	}
	pub fn evasion_value(&self) -> AccuracyType
	{
		StatModifiers::evasion_accuracy_value(self.evasion)
	}
	pub fn accuracy_stage(&self) -> StatModifierType
	{
		self.accuracy
	}
	pub fn accuracy_value(&self) -> AccuracyType
	{
		StatModifiers::evasion_accuracy_value(self.accuracy)
	}
	pub fn critical_stage(&self) -> StatModifierType
	{
		self.attack
	}
}
