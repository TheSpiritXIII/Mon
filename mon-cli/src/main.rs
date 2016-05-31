extern crate mon_gen;
extern crate rand;

mod display;
mod terminal;

use std::str;

use mon_gen::{SpeciesType, Monster};
use mon_gen::base::battle::{Battle, Party, CommandType, BattleExecution, CommandAttack, Effect};

use display::{display_attacks, display_party, display_active};

use rand::distributions::{Range, IndependentSample};

/// Prompts the user to switch party members and returns the selected member if possible.
///
/// If `back` is true, then the user will be able to select an input equal to the number of party
/// members indicating that the user does not want to switch anymore.
///
fn battle_prompt_switch(battle: &Battle, party: usize, back: bool) -> Result<usize, &'static str>
{
	// TODO: Move some of this logic to Battle itself?
	display_party(battle.party(party), back);
	println!("\nChoose a party member to switch to:");
	let member_count = battle.party(party).count();
	let input = terminal::input_range(battle.party(party).count() + 1) - 1;
	if input == member_count
	{
		if back && input == member_count
		{
			Ok(input)
		}
		else
		{
			Err("Value out of range.")
		}
	}
	else if battle.monster_is_active(party, input)
	{
		Err("Selected party member is already active.")
	}
	else if battle.monster(party, input).get_health() == 0
	{
		Err("Selected party member has no health.")
	}
	else
	{
		Ok(input)
	}
}

fn main()
{
	let mut rng = rand::thread_rng();

	let mut party_enemy = vec![
		Monster::new(SpeciesType::Deoxys, 10),
		Monster::new(SpeciesType::Deoxys, 10),
	];
	let mut party_self = vec![
		Monster::new(SpeciesType::Shaymin, 10),
		Monster::new(SpeciesType::Bulbasaur, 5),
		Monster::new(SpeciesType::Bulbasaur, 20),
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
						let attack_command = CommandAttack
						{
							party: 1,
							member: 0,
							attack_index: input - 1,
						};
						battle.add_command(CommandType::Attack(attack_command), 0, active);
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
					match battle_prompt_switch(&battle, 0, true)
					{
						Ok(target) =>
						{
							println!("Going with... {}", target);
							if battle.party(0).count() == target
							{
								last_input = None;
								continue;
							}
							else
							{
								battle.add_command(CommandType::Switch(target), 0, active);
							}
						}
						Err(e) =>
						{
							println!("Invalid selection: {}", e);
							terminal::wait();
							continue;
						}
					}
				}
				4 =>
				{
					// TODO: Escape calculation.
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
				for opponent_index in 0..battle.monster_active_count(1)
				{
					let attack_range = Range::new(0,
						battle.monster_active(1, active).get_attacks().len());
					let attack_command = CommandAttack
					{
						party: 0,
						member: target_range.ind_sample(&mut rng),
						attack_index: attack_range.ind_sample(&mut rng),
					};
					battle.add_command(CommandType::Attack(attack_command), 1, opponent_index);
				}

				active = 0;
			}

			loop
			{
				terminal::clear();
				println!("");
				display_active(&battle, usize::max_value());
				println!("");

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
									command.party).active_member(command.member);
								let nick = str::from_utf8(monster.get_nick()).unwrap();
								println!("{} used an attack.", nick);
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
									println!("");
									display_active(&battle, usize::max_value());
									println!("");
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
						match battle_prompt_switch(&battle, 0, false)
						{
							Ok(member) =>
							{
								battle.execute_switch(member)
							}
							Err(e) =>
							{
								println!("Invalid selection: {}", e);
								terminal::wait();
								continue;
							}
						}
					}
					BattleExecution::Waiting =>
					{
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
