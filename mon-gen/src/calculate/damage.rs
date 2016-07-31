use types::monster::StatType;
use types::attack::AccuracyType;
use base::party::PartyMember;

use gen::battle::Category;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use types::monster::ExperienceType;

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
	bonus *= if critical
	{
		1.5f32
	}
	else
	{
		1f32
	};

	// Randomness bonus.
	let range = Range::new(0.85f32, 1f32);
	bonus *= range.ind_sample(rng);

	max(1, ((((2 * offending.member.get_level() + 10) as f32 / 250f32) *
		(stat_attack as f32 / stat_defense as f32) * attack.power as f32 * 2f32) *
		bonus).floor() as StatType)
}

use base::party::Party;

use std::collections::HashMap;

pub struct MemberIndex
{
	pub party: usize,
	pub member: usize,
}

pub fn calculate_experience(parties: &[Party], _: Option<MemberIndex>,
	defense: MemberIndex) -> HashMap<usize, HashMap<usize, ExperienceType>>
{
	// TODO: Bonus is 1.5 if battling a non-wild trainer.
	let bonus = 1.0f32;

	let defense_member = parties[defense.party].active_member(defense.member).member;
	let base_yield = defense_member.get_species().species().experience_yield as f32;
	let level = defense_member.get_level() as f32;

	// TODO: Bigger bonus if monster is traded.
	let trade_bonus = 1.0f32;

	let gain = ((bonus * trade_bonus * base_yield * level) / 7f32).round() as ExperienceType;

	let mut party_map = HashMap::new();

	let exposed = parties[defense.party].expose_get_member(defense.member);
	for exposed_party in exposed
	{
		let member_map = party_map.entry(*exposed_party.0).or_insert_with(HashMap::new);
		for exposed_member in exposed_party.1
		{
			member_map.insert(*exposed_member, gain);
		}
	}

	party_map
}
