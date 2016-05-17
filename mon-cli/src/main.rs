extern crate mon_gen;

mod display;
mod terminal;

use std::str;

use mon_gen::{SpeciesType, Monster};
use mon_gen::base::battle::{Battle, Party, CommandType, BattleExecution, CommandAttack};

use display::{display_attacks, display_party, display_active};

fn main()
{
	let mut party_enemy = vec![
		Monster::new(SpeciesType::Deoxys, 10),
		Monster::new(SpeciesType::Deoxys, 10),
	];
	let mut party_self = vec![
		Monster::new(SpeciesType::Shaymin, 10),
		Monster::new(SpeciesType::Bulbasaur, 5),
	];
	let battle_self = Party::new(&mut party_self, 2);
	let battle_enemy = Party::new(&mut party_enemy, 2);
	let battle_data = vec![
		battle_self,
		battle_enemy,
	];

	let mut battle = Battle::new(battle_data);

	let mut last_input: Option<usize> = None;
	let mut active = 0;
	loop
	{
		terminal::clear();
		println!("");
		display_active(&battle, active);
		println!("Inputting...");

		if let Some(input) = last_input
		{
			match input
			{
				1 =>
				{
					let attack_len =
					{
						let attack_list = battle.monster_active(0, active).get_attacks();
						display_attacks(attack_list);
						attack_list.len() + 1
					};
					println!("\nChoose an attack to use:");
					let input = terminal::input_range(attack_len);
					if input == attack_len
					{
						last_input = None;
						continue;
					}
					else
					{
						println!("Adding..");
						println!("{:?}", battle.add_command(CommandType::Attack(
							CommandAttack { party: 1, monster: 0, attack_index: input - 1 }), 0, active));
						println!("{:?}", battle.add_command(CommandType::Attack(
							CommandAttack { party: 0, monster: 0, attack_index: 0 }), 1, 0));
					}
				}
				3 =>
				{
					display_party(battle.party(0).members);
					println!("\nChoose a party member to switch to:");
					let input = terminal::input_range(battle.party(0).members.len() + 1);
					if input == battle.party(0).members.len() + 1
					{
						last_input = None;
						continue;
					}
					else
					{
						println!("Unimplemented Switch");
					}
				}
				4 =>
				{
					// TODO: Escape calculation.
					println!("Ran away safely.");
					break;
				}
				_ => println!("Unimplemented action"),
			}
			if active != battle.monster_active_count(0) - 1
			{
				active += 1;
				last_input = None;
				continue;
			}
			loop
			{
				terminal::clear();
				println!("");
				display_active(&battle, usize::max_value());
				println!("");
				// stdin_input_wait();

				match battle.execute()
				{
					BattleExecution::Command =>
					{
						let command = battle.get_current_command().unwrap();
						match command.command_type
						{
							CommandType::Attack(_) =>
							{
								let monster = &battle.party(
									command.party).members[command.monster];
								let nick = str::from_utf8(monster.get_nick()).unwrap();
								println!("{} used an attack.", nick);
							}
							_ =>
							{
								// println!("Unknown command : {:?}", command);
							}
						}
						terminal::wait();
					}
					BattleExecution::Queue =>
					{
						// println!("Queue.");
						// stdin_input_wait();
						continue;
					}
					BattleExecution::Waiting =>
					{
						// println!("Waiting.");
						// stdin_input_wait();
						break;
					}
				}
			}
			last_input = None;
			continue;
		}
		else
		{
			println!("{:^20}{:^20}{:^20}{:^20}", "1) Attack", "2) Item", "3) Switch", "4) Exit");
			println!("\nWhat will you do?");
		}


		last_input = Some(terminal::input_range(4));
	}
}
