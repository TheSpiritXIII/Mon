use rand::Rng;

use base::attack::Target;
use base::command::CommandAttack;
use base::effect::{Damage, DamageMeta, Effect, FlagsChange, NoneReason, Retreat};
use base::runner::{BattleFlagsType, BattleEffects, BattleState};
use types::monster::StatType;


pub fn for_targets<F>(command: &CommandAttack, party: usize, state: &BattleState, mut closure: F)
	where F: FnMut(usize, usize)
{
	let target =
	{
		let member = &state.parties()[party].active_member(command.member).member;
		member.attacks()[command.attack_index].attack().target
	};
	if target & Target::MULTI == 0
	{
		closure(command.target_party, command.target_member);
	}
	else
	{
		for party_index in 0..state.parties().len()
		{
			// TODO: This should be side check, not party check.
			let same_side = party == party_index;
			let hit_enemies = target & Target::SIDE_ENEMY != 0 && !same_side;
			let hit_allies = target & Target::SIDE_ALLY != 0 && same_side;
			
			if hit_enemies || hit_allies
			{
				for active_index in 0..state.parties()[party_index].active_count()
				{
					let is_adjacent = Target::is_adjacent_with(command.member, active_index);
					let hit_adjacent = target & Target::RANGE_ADJACENT != 0 && is_adjacent;
					let hit_opposite = target & Target::RANGE_OPPOSITE != 0 && !is_adjacent;

					if hit_adjacent || hit_opposite
					{
						let not_self = target & Target::TARGET_SELF == 0;
						let is_self = party_index == party && active_index == command.member;
						if not_self || !is_self
						{
							closure(party_index, active_index);
						}
					}
				}
			}
		}
	}
}

pub fn damage_fixed<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, party: usize,
	state: &BattleState, _: &mut R, amount: StatType)
{
	for_targets(command, party, state, |target_party, target_member|
	{
		let defending_party = &state.parties()[command.target_party];
		let damage = Damage
		{
			party: target_party,
			active: target_member,
			member: defending_party.active_member_index(command.target_member),
			meta: DamageMeta
			{
				amount: amount,
				type_bonus: 1.0,
				critical: false,
			}
		};
		effects.effect_add(Effect::Damage(damage));
	});
}

pub fn retreat<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, party: usize,
	state: &BattleState, _: &mut R)
{
	for_targets(command, party, state, |target_party, target_member|
	{
		effects.effect_add(Effect::Retreat(Retreat
		{
			party: target_party,
			active: target_member,
		}));
	});
}

pub fn nothing<R: Rng>(effects: &mut BattleEffects, _: &CommandAttack, _: usize, _: &BattleState,
	_: &mut R)
{
	effects.effect_add(Effect::None(NoneReason::None));
}

pub fn battle_flags_toggle<R: Rng>(effects: &mut BattleEffects, _: &CommandAttack, _: usize,
	state: &BattleState, _: &mut R, flags: BattleFlagsType)
{
	effects.effect_add(Effect::FlagsChange(FlagsChange
	{
		flags: state.flags() ^ flags
	}));
}