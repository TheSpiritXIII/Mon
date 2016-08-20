extern crate mon_gen;

use mon_gen::experimental::{Battle, BattleError};
use mon_gen::battle::Party;
use mon_gen::monster::{Monster, StatType};
use mon_gen::species::SpeciesType;

// Validate that correct errors are being returned.
#[test]
fn battle_command_error()
{
	let mut party_data0 =
	[
		Monster::new(SpeciesType::Deoxys, 1),
		Monster::new(SpeciesType::Deoxys, 2),
		Monster::new(SpeciesType::Deoxys, 3),
	];

	let mut party_data1 =
	[
		Monster::new(SpeciesType::Deoxys, 4),
		Monster::new(SpeciesType::Deoxys, 5),
		Monster::new(SpeciesType::Deoxys, 6),
		Monster::new(SpeciesType::Deoxys, 7),
		Monster::new(SpeciesType::Deoxys, 8),
		Monster::new(SpeciesType::Deoxys, 9),
	];

	// {
	// 	let limit_total = party_data0[2].attacks()[1].limit_left();
	// 	party_data0[2].attacks_mut()[1].limit_left_take(limit_total);
	// }
	party_data1[4].health_lose(StatType::max_value());

	let parties = vec!
	[
		Party::new(&mut party_data0, 0, 3, false),
		Party::new(&mut party_data1, 0, 3, false),
	];

	let mut battle = Battle::new(parties).unwrap();
	// assert_eq!(battle.run(), BattleExecution::Waiting);

	// TODO: Add attacks that prevent escape.
	assert_eq!(battle.command_add_escape(0), BattleError::None);
	assert_eq!(battle.command_add_escape(1), BattleError::None);

	// Changing who to switch to.
	assert_eq!(battle.command_add_switch(1, 0, 3), BattleError::None);
	assert_eq!(battle.command_add_switch(1, 0, 5), BattleError::None);
	assert_eq!(battle.command_add_switch(1, 0, 4), BattleError::SwitchHealth);

	// Switching to a member already queued to switch in.
	assert_eq!(battle.command_add_switch(1, 1, 3), BattleError::None);
	assert_eq!(battle.command_add_switch(1, 1, 5), BattleError::SwitchQueued);

	// Attempt to switch to the same member.
	assert_eq!(battle.command_add_switch(1, 2, 1), BattleError::None);
	assert_eq!(battle.command_add_switch(1, 2, 2), BattleError::SwitchActive);

	// Target an opposing party.
	assert_eq!(battle.command_add_attack(0, 0, 0, 1, 0), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 0, 0, 1, 2), BattleError::AttackTarget);
	assert_eq!(battle.command_add_attack(0, 0, 0, 1, 1), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 1, 0, 1, 0), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 1, 0, 1, 2), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 1, 0, 1, 1), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 2, 0, 1, 0), BattleError::AttackTarget);
	assert_eq!(battle.command_add_attack(0, 2, 0, 1, 2), BattleError::None);
	assert_eq!(battle.command_add_attack(0, 2, 0, 1, 1), BattleError::None);
	// assert_eq!(battle.command_add_attack(0, 2, 1, 1, 2), BattleError::Limit);

	// TODO: Non-normal attack target.

	// Start the battle turn. All commands are rejected.
	// assert_eq!(battle.run(), BattleExecution::Waiting);

	// assert_eq!(battle.command_add_escape(0), BattleError::Rejected);
	// assert_eq!(battle.command_add_switch(0, 0, 0), BattleError::Rejected);
	// assert_eq!(battle.command_add_attack(0, 0, 0, 1, 0), BattleError::Rejected);
}
