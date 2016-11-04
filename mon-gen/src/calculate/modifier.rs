// TODO: Don't publically use here.
pub use rand::Rng;

pub use base::command::CommandAttack;
pub use base::effect::{Damage, DamageMeta, Effect, FlagsChange, Modifier, NoneReason};
pub use base::runner::{BattleFlagsType, BattleEffects, BattleState};
pub use base::statmod::StatModifiers;
pub use types::monster::StatType;
pub use types::battle::StatModifierType;

pub fn modifier_delta<F>(effects: &mut BattleEffects, command: &CommandAttack, modifier_func: F)
		where F: Fn(&mut StatModifiers)
{
	let mut stats = Default::default();
	modifier_func(&mut stats);
	let modifier = Modifier::new(command.target_party, command.target_member, stats);
	effects.effect_add(Effect::Modifier(modifier));
}

pub mod attack
{

use super::*;

pub fn delta<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, _: usize,
	_: &BattleState, _: &mut R, amount: StatModifierType)
{
	modifier_delta(effects, command, |modifier|
	{
		modifier.attack_delta(amount);
	});
}

}

pub mod defense
{

use super::*;

pub fn delta<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, _: usize,
	_: &BattleState, _: &mut R, amount: StatModifierType)
{
	modifier_delta(effects, command, |modifier|
	{
		modifier.defense_delta(amount);
	});
}

}

pub mod accuracy
{

use super::*;

pub fn delta<R: Rng>(effects: &mut BattleEffects, command: &CommandAttack, _: usize,
	_: &BattleState, _: &mut R, amount: StatModifierType)
{
	modifier_delta(effects, command, |modifier|
	{
		modifier.accuracy_delta(amount);
	});
}

}
