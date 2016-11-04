use std::cmp::max;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use base::command::CommandAttack;
use base::effect::{Damage, DamageMeta, Effect, NoneReason};
use base::party::PartyMember;
use base::runner::{BattleEffects, BattleState};
use calculate::common::for_targets;
use gen::attack::Category;
use types::attack::AccuracyType;
use types::battle::StatModifierType;
use types::monster::StatType;

#[cfg(feature = "test")]
pub fn calculate_miss<R: Rng>(_: &PartyMember, _: usize, _: &mut R) -> bool
{
	false
}

#[cfg(not(feature = "test"))]
pub fn calculate_miss<R: Rng>(offending: &PartyMember, attack_index: usize, rng: &mut R) -> bool
{
	let attack = offending.member.attacks()[attack_index].attack();
	let range = Range::new(0.0 as AccuracyType, 1.0 as AccuracyType);
	let chance = offending.modifiers.accuracy_value() / offending.modifiers.evasion_value();
	range.ind_sample(rng) > attack.accuracy / chance
}

#[cfg(feature = "test")]
pub fn calculate_damage<R: Rng>(offending: &PartyMember, attack_index: usize,
	defending: &PartyMember, critical: bool, bonus: f32, _: &mut R) -> StatType
{
	calculate_damage_randomness(offending, attack_index, defending, critical, bonus, 1.0f32)
}

#[cfg(not(feature = "test"))]
pub fn calculate_damage<R: Rng>(offending: &PartyMember, attack_index: usize,
	defending: &PartyMember, critical: bool, bonus: f32, rng: &mut R) -> StatType
{
	let range = Range::new(0.85f32, 1f32);
	let randomness = range.ind_sample(rng);

	calculate_damage_randomness(offending, attack_index, defending, critical, bonus, randomness)
}

pub fn calculate_damage_randomness(offending: &PartyMember, attack_index: usize,
	defending: &PartyMember, critical: bool, bonus: f32, randomness: f32) -> StatType
{
	let attack = offending.member.attacks()[attack_index].attack();
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

	bonus *= randomness;

	max(1, ((((2 * offending.member.level() + 10) as f32 / 250f32) *
		(stat_attack as f32 / stat_defense as f32) * attack.power as f32 * 2f32) *
		bonus).floor() as StatType)
}

pub fn miss_or<R: Rng, F>(effects: &mut BattleEffects, command: &CommandAttack,
	party: usize, state: &BattleState, rng: &mut R, func: F)
		where F: Fn(&mut BattleEffects, &CommandAttack, usize, &BattleState, &mut R)
{
	let attacking_member = &state.parties()[party].active_member(command.member);
	if calculate_miss(attacking_member, command.attack_index, rng)
	{
		effects.effect_add(Effect::None(NoneReason::Miss));
	}
	else
	{
		func(effects, command, party, state, rng);
	}
}

fn is_critical<R: Rng>(stage: StatModifierType, high_chance: bool, rng: &mut R) -> bool
{
	let rate = match stage
	{
		0 => 32,
		1 => 16,
		2 => 8,
		_ => 4,
	};
	let odds = if high_chance
	{
		2
	}
	else
	{
		1
	};
	rng.gen::<u8>() % rate <= odds
}

pub fn damage<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, party: usize,
	state: &BattleState, rng: &mut R)
{
	for_targets(command, party, state, |target_party, target_member|
	{
		let attacking_party = &state.parties()[party];
		let defending_party = &state.parties()[target_party];
		let attacking_member = &attacking_party.active_member(command.member);
		let defending_member = &defending_party.active_member(target_member);

		// Element defense bonus.
		let mut type_bonus = 1f32;
		let attack = attacking_member.member.attacks()[command.attack_index].attack();
		for element in defending_member.member.get_elements()
		{
			type_bonus *= attack.element.effectiveness(*element);
		}

		let is_critical = is_critical(attacking_member.modifiers.critical_stage(), false, rng);

		let amount = calculate_damage(attacking_member, command.attack_index, defending_member,
			is_critical, type_bonus, rng);

		let damage = Damage
		{
			party: command.target_party,
			active: command.target_member,
			member: defending_party.active_member_index(command.target_member),
			meta: DamageMeta
			{
				amount: amount,
				type_bonus: type_bonus,
				critical: is_critical,
			}
		};
		effects.effect_add(Effect::Damage(damage));
	});
}
