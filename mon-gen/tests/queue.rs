extern crate mon_gen;

use mon_gen::experimental::BattleQueue;
use mon_gen::battle::{Party, Command, CommandType};
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

	// Add a command to each party but not each party member.
	queue.command_add(Command::new(CommandType::Escape, 0), 0, 0);
	queue.command_add(Command::new(CommandType::Escape, 1), 1, 0);
	queue.command_add(Command::new(CommandType::Escape, 2), 2, 0);
	assert!(!queue.ready());

	// Add a party command, overriding member commands.
	queue.command_add_party(Command::new(CommandType::Escape, 0), 0);
	assert!(!queue.ready());

	queue.command_add_party(Command::new(CommandType::Escape, 1), 1);
	assert!(!queue.ready());

	queue.command_add_party(Command::new(CommandType::Escape, 2), 2);
	assert!(queue.ready());

	// Add a member command, override party commands.
	queue.command_add(Command::new(CommandType::Escape, 0), 0, 2);
	assert!(!queue.ready());

	queue.command_add(Command::new(CommandType::Escape, 1), 1, 2);
	assert!(!queue.ready());

	queue.command_add(Command::new(CommandType::Escape, 2), 2, 2);
	assert!(!queue.ready());

	// Fill the rest of the members.
	queue.command_add(Command::new(CommandType::Escape, 0), 0, 0);
	queue.command_add(Command::new(CommandType::Escape, 0), 0, 1);
	assert!(!queue.ready());

	queue.command_add(Command::new(CommandType::Escape, 1), 1, 0);
	queue.command_add(Command::new(CommandType::Escape, 1), 1, 1);
	assert!(!queue.ready());

	queue.command_add(Command::new(CommandType::Escape, 2), 2, 0);
	queue.command_add(Command::new(CommandType::Escape, 2), 2, 1);
	assert!(queue.ready());
}
