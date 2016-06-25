use base::monster::Monster;
use base::types::monster::StatType;
use base::types::attack::AccuracyType;
use base::types::battle::StatModifier;
use base::party::MemberStatModifiers;

use gen::battle::Category;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::cmp::max;

pub fn calculate_critical<R: Rng>(stage: StatModifier, rng: &mut R) -> bool
{
	let rate = match stage
	{
		0 => 16,
		1 => 8,
		2 => 2,
		_ => 1,
	};
	rng.gen::<u8>() % rate == 0
}

pub fn accuracy_stage_probability(stage: StatModifier) -> AccuracyType
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

pub fn evasion_stage_probability(stage: StatModifier) -> AccuracyType
{
	accuracy_stage_probability(-stage)
}

pub fn calculate_miss<R: Rng>(offending: &Monster, attack_index: usize,
	modifiers: &MemberStatModifiers, rng: &mut R) -> bool
{
	let attack = offending.get_attacks()[attack_index].attack();
	let range = Range::new(0.0 as AccuracyType, 1.0 as AccuracyType);
	let accuracy = accuracy_stage_probability(modifiers.accuracy);
	let evasion = evasion_stage_probability(modifiers.evasion);
	range.ind_sample(rng) > attack.accuracy / (accuracy / evasion)
}

pub fn calculate_damage<R: Rng>(offending: &Monster, attack_index: usize, defending: &Monster,
	critical: bool, bonus: f32, rng: &mut R) -> StatType
{
	let attack = offending.get_attacks()[attack_index].attack();
	let mut bonus = bonus;
	let (stat_attack, stat_defense) = match attack.category
	{
		Category::Physical => (offending.get_stat_attack(), defending.get_stat_defense()),
		Category::Special => (offending.get_stat_attack(), defending.get_stat_defense()),
		_ => (1, 1),
	};

	// Element attack bonus.
	for element in offending.get_elements()
	{
		if *element == attack.element
		{
			bonus *= 1.5f32;
			break;
		}
	}

	// Critical attack bonus.
	bonus *= match critical
	{
		true => 1.5f32,
		false => 1f32,
	};

	// Randomness bonus.
	let range = Range::new(0.85f32, 1f32);
	bonus *= range.ind_sample(rng);

	max(1, ((((2 * offending.get_level() + 10) as f32 / 250f32) *
		(stat_attack as f32 / stat_defense as f32) * attack.power as f32 * 2f32) *
		bonus).floor() as StatType)
}
