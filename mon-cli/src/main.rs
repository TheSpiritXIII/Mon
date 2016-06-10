extern crate mon_gen;
extern crate rand;

mod display;
mod terminal;

use std::str;

use rand::distributions::{Range, IndependentSample};

use mon_gen::{SpeciesType, Monster};
use mon_gen::base::battle::{Battle, Party, CommandType, BattleExecution, Effect, BattleError};

use display::{display_attacks, display_party, display_active};

/// Prompts the user to switch party members and returns the selected member if possible.
///
/// If `back` is true, then the user will be able to select an input equal to the number of party
/// members indicating that the user does not want to switch anymore.
///
fn battle_prompt_switch(battle: &Battle, party: usize, back: bool) -> usize
{
	display_party(battle.party(party), back);
	println!("\nChoose a party member to switch to:");
	let member_count = battle.party(party).count() + match back
	{
		true  => 1,
		false => 0,
	};
	terminal::input_range(member_count) - 1
}

/// Returns a descriptive string of the given battle error.
fn battle_error_as_string(err: BattleError) -> &'static str
{
	match err
	{
		BattleError::None =>
		{
			unreachable!();
		}
		BattleError::Blocking =>
		{
			unreachable!();
		}
		BattleError::Ready =>
		{
			unreachable!();
		}
		BattleError::Limit =>
		{
			"Selected move has no PP left."
		}
		BattleError::Target =>
		{
			unreachable!();
		}
		BattleError::Active =>
		{
			"Selected party member is already active."
		}
		BattleError::Health =>
		{
			"Selected party member has no health."
		}
		BattleError::Queued =>
		{
			"Selected party member is already queued to switch in."
		}
		BattleError::Escape =>
		{
			unreachable!();
		}
	}
}

fn main()
{
	// For the AI randomness.
	let mut rng = rand::thread_rng();

	// Initialize parties.
	let mut party_enemy = [
		Monster::new(SpeciesType::Deoxys, 10),
		Monster::new(SpeciesType::Deoxys, 10),
	];
	let mut party_self = [
		Monster::new(SpeciesType::Shaymin, 10),
		Monster::new(SpeciesType::Bulbasaur, 5),
		Monster::new(SpeciesType::Bulbasaur, 20),
	];
	let battle_data = vec![
		Party::new(&mut party_self, 2),
		Party::new(&mut party_enemy, 2),
	];
	let mut battle = Battle::new(battle_data);

	// Stores the latest input. Used for when there are commands that need multiple user inputs.
	let mut last_input: Option<usize> = None;

	// Stores the active monster that the user is inputting commands for.
	let mut active = 0;

	// Stores the commands to publish for the player.
	// let mut command_queue: Vec<> = Vec::new();

	loop
	{
		terminal::clear();
		display_active(&battle, active);

		if let Some(input) = last_input
		{
			match input
			{
				1 =>
				{
					let attack_amount =
					{
						let attack_list = battle.monster_active(0, active).get_attacks();
						display_attacks(attack_list);
						attack_list.len()
					};
					println!("\nChoose an attack to use:");

					// Input range is greater than the number of attacks for an option to go back.
					let input = terminal::input_range(attack_amount + 1);
					if input == attack_amount
					{
						last_input = None;
						continue;
					}
					else
					{
						let err = battle.add_command_attack(0, active, 1, 0, input - 1);
						if err != BattleError::None
						{
							println!("Invalid selection: {}", battle_error_as_string(err));
							terminal::wait();
							continue;
						}
					}
				}
				2 =>
				{
					// TODO: Item use.
					println!("Items are unimplemented. Select another choice.");
					terminal::wait();
					last_input = None;
					continue;
				}
				3 =>
				{
					let target = battle_prompt_switch(&battle, 0, true);
					if target == battle.party(0).count()
					{
						last_input = None;
						continue;
					}

					let err = battle.add_command_switch(0, active, target);
					if err != BattleError::None
					{
						println!("Invalid selection: {}", battle_error_as_string(err));
						terminal::wait();
						continue;
					}
				}
				4 =>
				{
					// TODO: Escaping should be a command.
					println!("Ran away safely.");
					break;
				}
				_ =>
				{
					unreachable!();
				}
			}

			if active != battle.monster_active_count(0) - 1
			{
				// Select command for next monster.
				active += 1;
				last_input = None;
				continue;
			}
			else
			{
				let target_range = Range::new(0, battle.monster_active_count(0));

				// AI battle command.
				for opponent in 0..battle.monster_active_count(1)
				{
					let attack_amount = battle.monster_active(1, opponent).get_attacks().len();
					let attack_index = Range::new(0, attack_amount).ind_sample(&mut rng);
					let target_member = target_range.ind_sample(&mut rng);
					battle.add_command_attack(1, opponent, 0, target_member, attack_index);
				}

				active = 0;
			}

			loop
			{
				terminal::clear();
				display_active(&battle, usize::max_value());

				match battle.execute()
				{
					BattleExecution::Command =>
					{
						let command = battle.get_current_command().unwrap();
						match command.command_type
						{
							CommandType::Attack(ref attack_command) =>
							{
								let monster = &battle.party(
									command.party).active_member(attack_command.member);
								let nick = str::from_utf8(monster.get_nick()).unwrap();
								let attack_name = "an attack";
								println!("{} used {}.", nick, attack_name);
								terminal::wait();
							}
							CommandType::Switch(_) =>
							{
								// Ignore.
							}
							CommandType::Escape =>
							{
								// Ignore.
							}
						}
					}
					BattleExecution::Queue =>
					{
						let effect = battle.get_current_effect().unwrap();
						match *effect
						{
							Effect::Damage(ref damage) =>
							{
								let member = battle.monster_active(damage.party(), damage.member());
								if member.get_health() == 0
								{
									terminal::clear();
									display_active(&battle, usize::max_value());
									println!("{} fainted!",
										str::from_utf8(member.get_nick()).unwrap());
									terminal::wait();
								}
							}
							Effect::Switch(_) =>
							{
								println!("Come back!");
								println!("Go!");
								terminal::wait();
							}
							Effect::None(_) =>
							{
								// TODO: Don't ignore this.
							}
						}
						continue;
					}
					BattleExecution::Switch(_) =>
					{
						let target = battle_prompt_switch(&battle, 0, false);
						let err = battle.execute_switch(target);
						if err != BattleError::None
						{
							println!("Invalid selection: {}", battle_error_as_string(err));
							terminal::wait();
							continue;
						}
					}
					BattleExecution::Waiting =>
					{
						break;
					}
				}
			}

			last_input = None;
		}
		else
		{
			let exit_str = if active == 0
			{
				"4) Escape"
			}
			else
			{
				"4) Back"
			};
			println!("{:^20}{:^20}{:^20}{:^20}", "1) Move", "2) Item", "3) Switch", exit_str);
			println!("\nWhat will you do?");

			last_input = Some(terminal::input_range(4));
		}
	}
}
