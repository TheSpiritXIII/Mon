// TODO: Move these functions elsewhere.
use rand::Rng;

use base::command::CommandAttack;
use base::effect::{Damage, DamageMeta, Effect, FlagsChange, NoneReason, Retreat};
use base::runner::{BattleFlagsType, BattleEffects, BattleState};
use types::monster::StatType;

pub fn damage_fixed<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, _: usize,
	state: &BattleState, _: &mut R, amount: StatType)
{
	let defending_party = &state.parties()[command.target_party];
	let damage = Damage
	{
		party: command.target_party,
		active: command.target_member,
		member: defending_party.active_member_index(command.target_member),
		meta: DamageMeta
		{
			amount: amount,
			type_bonus: 1.0,
			critical: false,
		}
	};
	effects.effect_add(Effect::Damage(damage));
}

pub fn retreat<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, party: usize,
	_: &BattleState, _: &mut R)
{
	effects.effect_add(Effect::Retreat(Retreat
	{
		party: party,
		active: command.member,
	}));
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

pub mod effects;
pub mod experience;
pub mod modifier;
pub mod statistics;
