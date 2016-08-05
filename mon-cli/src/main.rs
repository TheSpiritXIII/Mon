extern crate mon_gen;
extern crate rand;

mod display;
mod terminal;

use rand::distributions::{Range, IndependentSample};

use mon_gen::monster::Monster;
use mon_gen::species::{SpeciesType, FormId};
use mon_gen::species::form::DeoxysForm;
use mon_gen::battle::{Party, Battle, BattleExecution, BattleError, CommandType, Effect, NoneReason, StatModifiers, StatModifierType};

use display::{display_attacks, display_party, display_active, display_error};

/// Prompts the user to switch party members and returns the selected member if possible.
///
/// If `back` is true, then the user will be able to select an input equal to the number of party
/// members indicating that the user does not want to switch anymore.
///
fn battle_prompt_switch(battle: &Battle, party: usize, back: bool) -> usize
{
	display_party(battle.party(party), back);
	println!("\nChoose a party member to switch to:");
	let member_count = battle.party(party).member_count() + if back
	{
		1
	}
	else
	{
		0
	};
	terminal::input_range(member_count) - 1
}

/// Displays a message for the given stat modifier.
fn battle_modifier_message(who: &str, stat: &'static str, amount: StatModifierType,
	current: StatModifierType, min: StatModifierType, max: StatModifierType)
{
	let difference =
	{
		if current == min && amount < 0
		{
			"won't go any lower"
		}
		else if current == max && amount > 0
		{
			"won't go any higher"
		}
		else
		{
			match amount
			{
				-3 => "severely fell",
				-2 => "harshly fell",
				-1 => "fell",
				1  => "rose",
				2  => "rose sharply",
				3  => "rose drastically",
				_  => unreachable!(),
			}
		}
	};
	println!("{}'s {} {}!", who, stat, difference);
}

fn execute_battle(battle: &mut Battle) -> bool
{
	loop
	{
		terminal::clear();
		display_active(battle, usize::max_value());

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
							command.party()).active_member(attack_command.member).member;
						let nick = monster.nick();
						let attack = attack_command.attack(command.party(), battle).attack();
						let attack_name = attack.name();
						println!("{} used {}.", nick, attack_name);
						terminal::wait();
					}
					CommandType::Switch(_) | CommandType::Escape =>
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
						if damage.critical()
						{
							terminal::clear();
							display_active(battle, usize::max_value());
							println!("It's a critical hit!");
							terminal::wait();
						}

						if damage.type_bonus() == 0f32
						{
							terminal::clear();
							display_active(battle, usize::max_value());
							println!("It's unaffective!");
							terminal::wait();
						}
						else if damage.type_bonus() < 1f32
						{
							terminal::clear();
							display_active(battle, usize::max_value());
							println!("It's not very effective...");
							terminal::wait();
						}
						else if damage.type_bonus() > 1f32
						{
							terminal::clear();
							display_active(battle, usize::max_value());
							println!("It's super effective!");
							terminal::wait();
						}

						let member = battle.monster(damage.party(), damage.member());
						if member.health() == 0
						{
							terminal::clear();
							display_active(battle, usize::max_value());
							println!("{} fainted!", member.nick());
							terminal::wait();
						}
					}
					Effect::Switch(_) =>
					{
						println!("Come back!");
						println!("Go!");
						terminal::wait();
					}
					Effect::Modifier(ref modifiers) =>
					{
						let member = &battle.party(
							modifiers.party()).active_member(modifiers.active());
						let nick = member.member.nick();
						let modifiers = modifiers.modifiers();
						if modifiers.attack_stage() != 0
						{
							battle_modifier_message(nick, "attack",
								modifiers.attack_stage(),
								member.modifiers().attack_stage(),
								StatModifiers::ATTACK_MIN, StatModifiers::ATTACK_MAX);
						}
						if modifiers.defense_stage() != 0
						{
							battle_modifier_message(nick, "defense",
								modifiers.defense_stage(),
								member.modifiers().defense_stage(),
								StatModifiers::DEFENSE_MIN, StatModifiers::DEFENSE_MAX);
						}
						if modifiers.sp_attack_stage() != 0
						{
							battle_modifier_message(nick, "sp. attack",
								modifiers.sp_attack_stage(),
								member.modifiers().sp_attack_stage(),
								StatModifiers::SP_ATTACK_MIN,
								StatModifiers::SP_ATTACK_MAX);
						}
						if modifiers.sp_defense_stage() != 0
						{
							battle_modifier_message(nick, "sp. defense",
								modifiers.sp_defense_stage(),
								member.modifiers().sp_defense_stage(),
								StatModifiers::SP_DEFENSE_MIN,
								StatModifiers::SP_DEFENSE_MAX);
						}
						if modifiers.speed_stage() != 0
						{
							battle_modifier_message(nick, "speed",
								modifiers.speed_stage(),
								member.modifiers().speed_stage(),
								StatModifiers::SPEED_MIN, StatModifiers::SPEED_MAX);
						}
						if modifiers.accuracy_stage() != 0
						{
							battle_modifier_message(nick, "accuracy",
								modifiers.accuracy_stage(),
								member.modifiers().accuracy_stage(),
								StatModifiers::ACCURACY_MIN, StatModifiers::ACCURACY_MAX);
						}
						if modifiers.evasion_stage() != 0
						{
							battle_modifier_message(nick, "evasion",
								modifiers.evasion_stage(),
								member.modifiers().evasion_stage(),
								StatModifiers::EVASION_MIN, StatModifiers::EVASION_MAX);
						}
						terminal::wait();
					}
					Effect::ExperienceGain(ref gain) =>
					{
						let member = battle.monster(gain.party, gain.member);
						println!("{} gained {} exp.", member.nick(), gain.amount);
						terminal::wait();
						if gain.level != member.level()
						{
							println!("{} leveled up!", member.nick());
							terminal::wait();
						}
					}
					Effect::None(ref reason) =>
					{
						match *reason
						{
							NoneReason::Miss =>
							{
								println!("It missed!");
								terminal::wait();
							}
							NoneReason::Escape =>
							{
								// TODO
							}
						}
					}
				}
			}
			BattleExecution::Switch(party) =>
			{
				if party == 0
				{
					let target = battle_prompt_switch(battle, 0, false);
					let err = battle.execute_switch(target);
					if err != BattleError::None
					{
						display_error(err);
						terminal::wait();
					}
				}
			}
			BattleExecution::Waiting =>
			{
				break;
			}
			BattleExecution::SwitchWaiting =>
			{
				if let Some(member) = battle.is_party_post_switch_waiting(0)
				{
					let target = battle_prompt_switch(battle, 0, false);
					let err = battle.execute_post_switch(0, member, target);
					if err != BattleError::None
					{
						display_error(err);
						terminal::wait();
					}
				}
			}
			BattleExecution::Finished(side) =>
			{
				if side == 0
				{
					println!("You won!");
				}
				else
				{
					println!("You lost...");
				}
				return true;
			}
		}
	}
	false
}

fn main()
{
	// For the AI randomness.
	let mut rng = rand::thread_rng();

	// Initialize parties.
	let mut party_enemy =
	[
		Monster::new(SpeciesType::Deoxys, 50),
		Monster::new(SpeciesType::Deoxys, 9),
	];
	party_enemy[0].form_set(DeoxysForm::Defense as FormId);
	party_enemy[1].form_set(DeoxysForm::Defense as FormId);
	let mut party_self =
	[
		Monster::new(SpeciesType::Bulbasaur, 60),
		Monster::new(SpeciesType::Bulbasaur, 2),
		Monster::new(SpeciesType::Bulbasaur, 7),
		Monster::new(SpeciesType::Bulbasaur, 8),
		Monster::new(SpeciesType::Shaymin, 10),
		Monster::new(SpeciesType::Bulbasaur, 5),
	];
	let battle_data = vec!
	[
		Party::new(&mut party_self, 0, 2, true),
		Party::new(&mut party_enemy, 1, 2, false),
	];
	let mut battle = Battle::new(battle_data);

	// Stores the latest input. Used for when there are commands that need multiple user inputs.
	let mut last_input: Option<usize> = None;

	// Stores the active monster that the user is inputting commands for.
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
					// Input range is greater than the number of attacks for an option to go back.
					let attack_amount =
					{
						let attack_list = battle.monster_active(0, active).member.attacks();
						display_attacks(attack_list);
						attack_list.len()
					} + 1;
					println!("\nChoose an attack to use:");

					let input = terminal::input_range(attack_amount);
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
							display_error(err);
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
					if target == battle.party(0).member_count()
					{
						last_input = None;
						continue;
					}

					let err = battle.add_command_switch(0, active, target);
					if err != BattleError::None
					{
						display_error(err);
						terminal::wait();
						continue;
					}
				}
				4 =>
				{
					if active != 0
					{
						last_input = None;
						active -= 1;
						continue;
					}
					// TODO: Escaping should be a command after the command checks are fixed.
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
					let attack_amount = battle.monster_active(1, opponent).member.attacks().len();
					let attack_index = Range::new(0, attack_amount).ind_sample(&mut rng);
					let target_member = target_range.ind_sample(&mut rng);
					battle.add_command_attack(1, opponent, 0, target_member, attack_index);
				}

				active = 0;
			}

			if execute_battle(&mut battle)
			{
				break;
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
