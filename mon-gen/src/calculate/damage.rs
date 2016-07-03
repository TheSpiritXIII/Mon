use base::types::monster::StatType;
use base::types::attack::AccuracyType;
use base::party::PartyMember;

use gen::battle::Category;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::cmp::max;

pub fn calculate_miss<R: Rng>(offending: &PartyMember, attack_index: usize, rng: &mut R) -> bool
{
	let attack = offending.member.get_attacks()[attack_index].attack();
	let range = Range::new(0.0 as AccuracyType, 1.0 as AccuracyType);
	let chance = offending.modifiers.accuracy_value() / offending.modifiers.evasion_value();
	range.ind_sample(rng) > attack.accuracy / chance
}

pub fn calculate_damage<R: Rng>(offending: &PartyMember, attack_index: usize, defending: &PartyMember,
	critical: bool, bonus: f32, rng: &mut R) -> StatType
{
	let attack = offending.member.get_attacks()[attack_index].attack();
	let mut bonus = bonus;
	let (stat_attack, stat_defense) = match attack.category
	{
		Category::Physical => (offending.attack(), defending.defense()),
		Category::Special => (offending.sp_attack(), defending.sp_defense()),
		_ => (1, 1),
	};

	// Element attack bonus.
	for element in offending.member.get_elements()
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

	max(1, ((((2 * offending.member.get_level() + 10) as f32 / 250f32) *
		(stat_attack as f32 / stat_defense as f32) * attack.power as f32 * 2f32) *
		bonus).floor() as StatType)
}
