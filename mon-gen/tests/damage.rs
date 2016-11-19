extern crate mon_gen;

use mon_gen::attack::AttackType;
use mon_gen::battle::
{
	Battle,
	BattleExecution,
	BattlePartyMember,
	CommandAttack,
	CommandType,
	Damage,
	DamageMeta,
	Effect,
	NoneReason,
	Party
};
use mon_gen::monster::Monster;
use mon_gen::species::SpeciesType;

// Validate move effectiveness changes battle damage and fixed damage attacks.
#[test]
fn battle_damage()
{
	// Bulbasaur vs Charmander
	// Assert super effective move.
	// Assert not very effective move.
	// Assert fixed damage.
	// Assert not effective move.
}

// Fixed damage always does promised amount except for unaffected elements.
#[test]
fn battle_damage_fixed()
{
	let mut monster_fixed_attack = Monster::new(SpeciesType::Charmander, 100);
	assert_eq!(monster_fixed_attack.attack_set(AttackType::DragonRage, 0), true);
	let mut party_data0 =
	[
		monster_fixed_attack,
	];

	let mut monster_skip = Monster::new(SpeciesType::Mew, 100);
	assert_eq!(monster_skip.attack_set(AttackType::Splash, 0), true);
	let mut party_data1 =
	[
		monster_skip,
	];

	let parties = vec!
	[
		Party::new(&mut party_data0, 0, 1, false),
		Party::new(&mut party_data1, 1, 1, false),
	];

	let mut battle = Battle::new(parties).unwrap();
	battle.command_add_attack(0, 0, 0, 1, 0);
	battle.command_add_attack(1, 0, 0, 0, 0);

	// The attack does the promised amount.

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(*battle.current_command(), CommandType::Attack(CommandAttack
	{
		party: 1,
		member: 0,
		attack_index: 0,
		target_party: 0,
		target_member: 0,
	}));
	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(*battle.current_effect(), Effect::None(NoneReason::None));

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(*battle.current_command(), CommandType::Attack(CommandAttack
	{
		party: 0,
		member: 0,
		attack_index: 0,
		target_party: 1,
		target_member: 0,
	}));

	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(*battle.current_effect(), Effect::Damage(Damage
	{
		party: 1,
		member: 0,
		active: 0,
		meta: DamageMeta
		{
			amount: 40,
			type_bonus: 1.0,
			critical: false,
		}
	}));

	// The attack does not do more damage even with a type advantage.

	// TODO: Add a Dragon type species.

	// The attack does no damage if the elemental type is unaffected.

	// TODO: Add a Fairy type species.
}

// Knock out moves have guaranteed kill except for unaffected elements.
#[test]
fn battle_damage_knock_out()
{
	let mut monster_knock_out_attack = Monster::new(SpeciesType::Mew, 1);
	assert_eq!(monster_knock_out_attack.attack_set(AttackType::Fissure, 0), true);
	let mut party_data0 =
	[
		monster_knock_out_attack,
	];

	let mut monster_skip = Monster::new(SpeciesType::Mew, 100);
	assert_eq!(monster_skip.attack_set(AttackType::Splash, 0), true);
	let mut party_data1 =
	[
		monster_skip,
	];

	let parties = vec!
	[
		Party::new(&mut party_data0, 0, 1, false),
		Party::new(&mut party_data1, 1, 1, false),
	];

	let mut battle = Battle::new(parties).unwrap();
	battle.command_add_attack(0, 0, 0, 1, 0);
	battle.command_add_attack(1, 0, 0, 0, 0);

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(*battle.current_command(), CommandType::Attack(CommandAttack
	{
		party: 1,
		member: 0,
		attack_index: 0,
		target_party: 0,
		target_member: 0,
	}));
	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(*battle.current_effect(), Effect::None(NoneReason::None));

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(*battle.current_command(), CommandType::Attack(CommandAttack
	{
		party: 0,
		member: 0,
		attack_index: 0,
		target_party: 1,
		target_member: 0,
	}));

	assert_eq!(battle.execute(), BattleExecution::Death(BattlePartyMember
	{
		party: 1,
		member: 0,
	}));

	assert_eq!(battle.execute(), BattleExecution::Finished(0));

	// TODO: Check still works against type disadvantage (Fire species).

	// TODO: Check does not work against types that are unaffective (Flying species).
}
