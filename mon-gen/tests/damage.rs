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

// Validate knock out moves do for sure kill in a single hit.
#[test]
fn battle_knock_out()
{
	let mut monster_knock_out = Monster::new(SpeciesType::Mew, 1);
	assert_eq!(monster_knock_out.attack_set(AttackType::Fissure, 0), true);
	let mut party_data0 =
	[
		monster_knock_out,
	];

	let mut monster_skip = Monster::new(SpeciesType::Mew, 100);
	let health = monster_skip.health();
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
}
