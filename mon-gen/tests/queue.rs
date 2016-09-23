extern crate mon_gen;

use mon_gen::battle::{BattleQueue, Party, CommandType, CommandAttack, CommandSwitch, CommandEscape};
use mon_gen::monster::Monster;
use mon_gen::species::SpeciesType;

// Make sure ready flag is working correctly when override commands.
#[test]
fn queue_ready()
{
	let mut party_data1 =
	[
		Monster::new(SpeciesType::Deoxys, 1),
		Monster::new(SpeciesType::Deoxys, 2),
		Monster::new(SpeciesType::Deoxys, 3),
	];

	let mut party_data2 =
	[
		Monster::new(SpeciesType::Deoxys, 4),
		Monster::new(SpeciesType::Deoxys, 5),
		Monster::new(SpeciesType::Deoxys, 6),
	];

	let mut party_data3 =
	[
		Monster::new(SpeciesType::Deoxys, 7),
		Monster::new(SpeciesType::Deoxys, 8),
		Monster::new(SpeciesType::Deoxys, 9),
	];

	let parties = vec!
	[
		Party::new(&mut party_data1, 0, 3, false),
		Party::new(&mut party_data2, 0, 3, false),
		Party::new(&mut party_data3, 0, 3, false),
	];

	let mut queue = BattleQueue::new(&parties);

	let mut escape: CommandEscape = CommandEscape
	{
		party: 0,
	};

	// Add a command to each party but not each party member.
	queue.command_add(CommandType::Escape(escape), 0, 0);
	escape.party = 1;
	queue.command_add(CommandType::Escape(escape), 1, 0);
	escape.party = 2;
	queue.command_add(CommandType::Escape(escape), 2, 0);
	assert!(!queue.ready());

	// Add a party command, overriding member commands.
	escape.party = 0;
	queue.command_add_party(CommandType::Escape(escape), 0);
	assert!(!queue.ready());

	escape.party = 1;
	queue.command_add_party(CommandType::Escape(escape), 1);
	assert!(!queue.ready());

	escape.party = 2;
	queue.command_add_party(CommandType::Escape(escape), 2);
	assert!(queue.ready());

	// Add a member command, override party commands.
	escape.party = 0;
	queue.command_add(CommandType::Escape(escape), 0, 2);
	assert!(!queue.ready());

	escape.party = 1;
	queue.command_add(CommandType::Escape(escape), 1, 2);
	assert!(!queue.ready());

	escape.party = 2;
	queue.command_add(CommandType::Escape(escape), 2, 2);
	assert!(!queue.ready());

	// Fill the rest of the members.
	escape.party = 0;
	queue.command_add(CommandType::Escape(escape), 0, 0);
	queue.command_add(CommandType::Escape(escape), 0, 1);
	assert!(!queue.ready());

	escape.party = 1;
	queue.command_add(CommandType::Escape(escape), 1, 0);
	queue.command_add(CommandType::Escape(escape), 1, 1);
	assert!(!queue.ready());

	escape.party = 2;
	queue.command_add(CommandType::Escape(escape), 2, 0);
	queue.command_add(CommandType::Escape(escape), 2, 1);
	assert!(queue.ready());
}

#[test]
fn queue_command()
{
	// TODO: Add items.
	let mut party_data1 =
	[
		Monster::new(SpeciesType::Deoxys, 7),
		Monster::new(SpeciesType::Deoxys, 8),
	];

	let mut party_data2 =
	[
		Monster::new(SpeciesType::Deoxys, 11),
		Monster::new(SpeciesType::Deoxys, 12),
	];

	let mut party_data3 =
	[
		Monster::new(SpeciesType::Deoxys, 1),
		Monster::new(SpeciesType::Deoxys, 2),
	];

	let mut party_data4 =
	[
		Monster::new(SpeciesType::Deoxys, 3),
		Monster::new(SpeciesType::Deoxys, 4),
	];

	let parties = vec!
	[
		Party::new(&mut party_data1, 0, 1, false),
		Party::new(&mut party_data2, 0, 1, false),
		Party::new(&mut party_data3, 0, 1, false),
		Party::new(&mut party_data4, 0, 1, false),
	];

	let mut queue = BattleQueue::new(&parties);
	let mut attack: CommandAttack = CommandAttack
	{
		party: 2,
		member: 0,
		attack_index: 0,
		target_party: 0,
		target_member: 0,
	};
	let mut switch: CommandSwitch = CommandSwitch
	{
		party: 0,
		member: 0,
		target: 1,
	};
	let mut escape: CommandEscape = CommandEscape
	{
		party: 0,
	};

	// Ensure priority command order.
	queue.command_add(CommandType::Switch(switch), 0, 0);
	escape.party = 1;
	queue.command_add(CommandType::Escape(escape), 1, 0);
	queue.command_add(CommandType::Attack(attack), 2, 0);
	escape.party = 3;
	queue.command_add(CommandType::Escape(escape), 3, 0);
	assert!(queue.ready());

	escape.party = 1;
	assert_eq!(queue.command_consume(&parties), CommandType::Escape(escape));
	assert!(queue.ready());
	escape.party = 3;
	assert_eq!(queue.command_consume(&parties), CommandType::Escape(escape));
	assert!(queue.ready());
	assert_eq!(queue.command_consume(&parties), CommandType::Switch(switch));
	assert!(queue.ready());
	assert_eq!(queue.command_consume(&parties), CommandType::Attack(attack));
	assert!(!queue.ready());

	// Ensure higher speed monsters go first.
	attack.party = 0;
	queue.command_add(CommandType::Attack(attack), 0, 0);
	attack.party = 1;
	queue.command_add(CommandType::Attack(attack), 1, 0);
	attack.party = 2;
	queue.command_add(CommandType::Attack(attack), 2, 0);
	attack.party = 3;
	queue.command_add(CommandType::Attack(attack), 3, 0);
	assert!(queue.ready());

	// TODO: Fix.
	// assert_eq!(queue.command_consume(&parties).party(), 1);
	// assert!(queue.ready());
	// assert_eq!(queue.command_consume(&parties).party(), 0);
	// assert!(queue.ready());
	// assert_eq!(queue.command_consume(&parties).party(), 3);
	// assert!(queue.ready());
	// assert_eq!(queue.command_consume(&parties).party(), 2);
	// assert!(!queue.ready());

	// TODO: Same attack, same monsters, different attack priorities.
}
