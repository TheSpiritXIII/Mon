use types::battle::StatModifierType;

use types::attack::AccuracyType;

use std::cmp;

#[derive(Debug, Default, Clone, PartialEq)]
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
	pub const ATTACK_MIN: StatModifierType = -6;
	pub const ATTACK_MAX: StatModifierType = 6;
	pub const DEFENSE_MIN: StatModifierType = -6;
	pub const DEFENSE_MAX: StatModifierType = 6;
	pub const SP_ATTACK_MIN: StatModifierType = -6;
	pub const SP_ATTACK_MAX: StatModifierType = 6;
	pub const SP_DEFENSE_MIN: StatModifierType = -6;
	pub const SP_DEFENSE_MAX: StatModifierType = 6;
	pub const SPEED_MIN: StatModifierType = -6;
	pub const SPEED_MAX: StatModifierType = 6;
	pub const ACCURACY_MIN: StatModifierType = -6;
	pub const ACCURACY_MAX: StatModifierType = 6;
	pub const EVASION_MIN: StatModifierType = -6;
	pub const EVASION_MAX: StatModifierType = 6;
	pub const CRITICAL_MIN: StatModifierType = 0;
	pub const CRITICAL_MAX: StatModifierType = StatModifierType::max_value();

	pub fn apply(&mut self, modifiers: &StatModifiers)
	{
		self.attack_delta(modifiers.attack);
		self.defense_delta(modifiers.attack);
		self.sp_attack_delta(modifiers.attack);
		self.sp_defense_delta(modifiers.attack);
		self.speed_delta(modifiers.attack);
		self.accuracy_delta(modifiers.attack);
		self.evasion_delta(modifiers.attack);
		self.critical_delta(modifiers.attack);
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
			0 =>  1.0,
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
	pub fn attack_stage(&self) -> StatModifierType
	{
		self.attack
	}
	pub fn attack_delta(&mut self, delta: StatModifierType)
	{
		self.attack = clamp::<StatModifierType>(self.attack + delta, StatModifiers::ATTACK_MIN,
			StatModifiers::ATTACK_MAX);
	}
	pub fn defense_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.defense)
	}
	pub fn defense_stage(&self) -> StatModifierType
	{
		self.defense
	}
	pub fn defense_delta(&mut self, delta: StatModifierType)
	{
		self.defense = clamp::<StatModifierType>(self.defense + delta, StatModifiers::DEFENSE_MIN,
			StatModifiers::DEFENSE_MAX);
	}
	pub fn sp_attack_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.attack)
	}
	pub fn sp_attack_stage(&self) -> StatModifierType
	{
		self.sp_attack
	}
	pub fn sp_attack_delta(&mut self, delta: StatModifierType)
	{
		self.sp_attack = clamp::<StatModifierType>(self.sp_attack + delta,
			StatModifiers::SP_ATTACK_MIN, StatModifiers::SP_ATTACK_MAX);
	}
	pub fn sp_defense_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.defense)
	}
	pub fn sp_defense_stage(&self) -> StatModifierType
	{
		self.sp_defense
	}
	pub fn sp_defense_delta(&mut self, delta: StatModifierType)
	{
		self.sp_defense = clamp::<StatModifierType>(self.sp_defense + delta,
			StatModifiers::SP_DEFENSE_MIN, StatModifiers::SP_DEFENSE_MAX);
	}
	pub fn speed_value(&self) -> AccuracyType
	{
		StatModifiers::base_value(self.speed)
	}
	pub fn speed_stage(&self) -> StatModifierType
	{
		self.speed
	}
	pub fn speed_delta(&mut self, delta: StatModifierType)
	{
		self.speed = clamp::<StatModifierType>(self.speed + delta,
			StatModifiers::SPEED_MIN, StatModifiers::SPEED_MAX);
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
			0 =>  1.0,
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
	pub fn accuracy_value(&self) -> AccuracyType
	{
		StatModifiers::evasion_accuracy_value(self.accuracy)
	}
	pub fn accuracy_stage(&self) -> StatModifierType
	{
		self.accuracy
	}
	pub fn accuracy_delta(&mut self, delta: StatModifierType)
	{
		self.accuracy = clamp::<StatModifierType>(self.speed + delta, StatModifiers::ACCURACY_MIN,
			StatModifiers::ACCURACY_MAX);
	}
	pub fn evasion_value(&self) -> AccuracyType
	{
		StatModifiers::evasion_accuracy_value(self.evasion)
	}
	pub fn evasion_stage(&self) -> StatModifierType
	{
		self.evasion
	}
	pub fn evasion_delta(&mut self, delta: StatModifierType)
	{
		self.evasion = clamp::<StatModifierType>(self.speed + delta, StatModifiers::EVASION_MIN,
			StatModifiers::EVASION_MAX);
	}
	pub fn critical_stage(&self) -> StatModifierType
	{
		self.attack
	}
	pub fn critical_delta(&mut self, delta: StatModifierType)
	{
		self.critical = cmp::min(0, self.critical.wrapping_add(delta));
	}
}
