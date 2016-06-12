use base::monster::Monster;
use base::types::monster::StatType;
use base::types::attack::AccuracyType;

use gen::battle::Category;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::cmp::max;

pub fn calculate_miss<R: Rng>(offending: &Monster, attack_index: usize, rng: &mut R) -> bool
{
	let attack = offending.get_attacks()[attack_index].attack();
	let range = Range::new(0 as AccuracyType, 1 as AccuracyType);
	range.ind_sample(rng) > attack.accuracy
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

	// Element defense bonus.
	for element in defending.get_elements()
	{
		bonus *= attack.element.effectiveness(*element);
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
