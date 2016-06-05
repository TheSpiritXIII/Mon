extern crate mon_gen;
extern crate rand;

mod display;
mod terminal;

use std::str;

use mon_gen::{SpeciesType, Monster};
use mon_gen::base::battle::{Battle, Party, CommandType, BattleExecution, Effect, BattleError, BattleSwitchError};

use display::{display_attacks, display_party, display_active};

use rand::distributions::{Range, IndependentSample};

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
	println!("Yes {}", member_count);
	terminal::input_range(member_count) - 1
}

fn battle_swtich_error_as_string(err: BattleSwitchError) -> &'static str
{
	match err
	{
		BattleSwitchError::Active =>
		{
			"Selected party member is already active."
		}
		BattleSwitchError::Health =>
		{
			"Selected party member has no health."
		}
		BattleSwitchError::Queued =>
		{
			"Selected party member is already queued to switch in."
		}
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
						battle.add_command_attack(0, active, 1, 0, input - 1);
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
					match battle.add_command_switch(0, active, target)
					{
						BattleError::Switch(switch_err) =>
						{
							println!("Invalid selection: {}", battle_swtich_error_as_string(switch_err));
							terminal::wait();
							continue;
						}
						BattleError::None =>
						{
							// Ignore
						}
						_ =>
						{
							unreachable!();
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
					let attack_member = target_range.ind_sample(&mut rng);
					let attack_index = attack_range.ind_sample(&mut rng);
					battle.add_command_attack(1, opponent_index, 0, attack_member, attack_index);
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
						match err
						{
							BattleError::Switch(switch_err) =>
							{
								println!("Invalid selection: {}", battle_swtich_error_as_string(switch_err));
								terminal::wait();
								continue;
							}
							BattleError::None =>
							{
								// Ignore
							}
							_ =>
							{
								println!("Unknown error: {:?}", err);
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
			println!("{:^20}{:^20}{:^20}{:^20}", "1) Attack", "2) Item", "3) Switch", "4) Escape");
			println!("\nWhat will you do?");
		}


		last_input = Some(terminal::input_range(4));
	}
}
