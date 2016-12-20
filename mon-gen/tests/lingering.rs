extern crate mon_gen;

use mon_gen::attack::AttackType;
use mon_gen::battle::
{
	Battle,
	BattleError,
	BattleExecution,
	BattlePartyMember,
	CommandAttack,
	CommandType,
	Damage,
	DamageMeta,
	Effect,
	LingeringChange,
	NoneReason,
	Party
};
use mon_gen::monster::Monster;
use mon_gen::species::SpeciesType;

fn command_attack_none(battle: &mut Battle)
{
	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(*battle.current_effect(), Effect::None(NoneReason::None));
}

// Validate death all in 5 turns.
#[test]
fn lingering_death_turns()
{
	let mut monster_lingering = Monster::new(SpeciesType::Mew, 100);
	assert_eq!(monster_lingering.attack_add(AttackType::Splash), true); // TODO: Remove.
	assert_eq!(monster_lingering.attack_set(AttackType::Splash, 0), true);
	assert_eq!(monster_lingering.attack_set(AttackType::PerishSong, 1), true);
	let mut party_data0 =
	[
		monster_lingering,
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
	battle.command_add_attack(0, 0, 1, 1, 0);
	battle.command_add_attack(1, 0, 0, 0, 0);

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(battle.execute(), BattleExecution::Effect);

	command_attack_none(&mut battle);

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(battle.execute(), BattleExecution::Effect);

	assert_eq!(battle.execute(), BattleExecution::Waiting);

	// Nothing happens for 3 turns.
	for _ in 0..3
	{
		assert_eq!(battle.command_add_attack(0, 0, 0, 1, 0), BattleError::None);
		assert_eq!(battle.command_add_attack(1, 0, 0, 0, 0), BattleError::None);

		command_attack_none(&mut battle);
		command_attack_none(&mut battle);

		assert_eq!(battle.execute(), BattleExecution::Command);
		assert_eq!(battle.execute(), BattleExecution::Effect);

		assert_eq!(battle.execute(), BattleExecution::Waiting);
	}
	
	// On 5th turn, all targets die.
	assert_eq!(battle.command_add_attack(0, 0, 0, 1, 0), BattleError::None);
	assert_eq!(battle.command_add_attack(1, 0, 0, 0, 0), BattleError::None);

	command_attack_none(&mut battle);
	command_attack_none(&mut battle);

	assert_eq!(battle.execute(), BattleExecution::Command);
	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(battle.execute(), BattleExecution::Effect);
	assert_eq!(*battle.current_effect(), Effect::LingeringChange(LingeringChange
	{
		index: 0,
	}));

	assert_eq!(battle.execute(), BattleExecution::Death(BattlePartyMember {
		party: 0,
		member: 0,
	}));

	assert_eq!(battle.execute(), BattleExecution::Death(BattlePartyMember {
		party: 1,
		member: 0,
	}));
}
