extern crate mon_gen;
extern crate rand;

use std::collections::HashMap;

mod display;
mod terminal;

use mon_gen::attack::AttackType;
use mon_gen::battle::CommandType;
use mon_gen::monster::Monster;
use mon_gen::species::{SpeciesType, FormId};
use mon_gen::species::form::DeoxysForm;
use mon_gen::battle;
use mon_gen::battle::{Party, Effect, NoneReason, StatModifiers, StatModifierType};
use mon_gen::battle::{Battle, BattleError, BattleExecution, LingeringType};
use rand::distributions::{IndependentSample, Range};

use display::{display, display_member, display_active, display_error, display_party, display_attacks};

/// Prompts the user to switch party members and returns the selected member if possible.
///
/// If `back` is true, then the user will be able to select an input equal to the number of party
/// members indicating that the user does not want to switch anymore.
///
fn battle_prompt_switch(battle: &Battle, active: usize, party: usize, back: bool) -> usize
{
	loop
	{
		terminal::clear();
		display_active(battle, active);
		display_party(&battle.state().parties()[party], back);
		println!("\nChoose a party member to switch to:");
		let member_count = battle.state().parties()[party].member_count();
		let input_count = member_count + if back
		{
			1
		}
		else
		{
			0
		};

		let input = terminal::input_range(input_count) - 1;
		if (!back || input != input_count) && battle.state().parties()[0].member_is_active(input)
		{
			println!("Cannot switch to a member who is already active.");
			terminal::wait();
			continue;
		}
		return input;
	}
}

fn battle_prompt_target(battle: &Battle) -> Option<(usize, usize)>
{
		let mut target_map = HashMap::new();
		for party_index in (0..battle.state().parties().len()).rev()
		{
			for index in 0..battle.state().parties()[party_index].active_count()
			{
				let target_index = target_map.len();
				target_map.insert(target_index, (party_index, index));
				let opponent = party_index & 1 == 1;
				display(format!("{})", target_index + 1), opponent);
				display_member(battle.state().parties()[party_index].active_member_alive(index),
					opponent, false)
			}
		}

		println!("");
		println!("{:>80}", format!("{}) {}", target_map.len() + 1, "Back"));

		let input = terminal::input_range(target_map.len() + 1) - 1;
		if input == target_map.len()
		{
			None
		}
		else
		{
			Some(target_map[&input])
		}
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

/// Randomly chooses a move and target.
fn battle_random_ai(battle: &mut Battle, party: usize)
{
	let party_side = battle.state().parties()[party].side();
	let party_active = battle.state().parties()[party].active_count();

	let mut party_targets = Vec::new();
	for party_index in 0..battle.state().parties().len()
	{
		if battle.state().parties()[party_index].side() != party_side
		{
			party_targets.push(party_index);
		}
	}
	let party_range = Range::new(0, party_targets.len());
	let mut rng = rand::thread_rng();

	for active_index in 0..party_active
	{
		let attack =
		{
			let active_member = battle.state().parties()[party].active_member(active_index).member;
			let attack_range = Range::new(0, active_member.attacks().len());
			attack_range.ind_sample(&mut rng)
		};
		let party_target = party_range.ind_sample(&mut rng);
		battle.command_add_attack(party, active_index, 0, party_target, attack);
	}
}

trait LingeringDisplay
{
	fn display_add(&self);
	fn display_change(&self);
}

impl LingeringDisplay for battle::Lingering
{
	fn display_add(&self)
	{
		unimplemented!();
	}
	fn display_change(&self)
	{
		unimplemented!();
	}
}

fn battle_execute_effect(battle: &Battle)
{
	match *battle.current_effect()
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

			let member = battle.state().parties()[damage.party()].member(damage.member());
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
			println!("(Switch) Come back!");
			println!("Go!");
			terminal::wait();
		}
		Effect::Retreat(_) =>
		{
			println!("(Retreat) Come back!");
			println!("Go!");
			terminal::wait();
		}
		Effect::Modifier(ref modifiers) =>
		{
			let member = &battle.state().parties()[modifiers.party()].active_member(
				modifiers.active());
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
			let member = battle.state().parties()[gain.party].member(gain.member);
			println!("{} gained {} exp.", member.nick(), gain.amount);
			terminal::wait();
			if gain.level != member.level()
			{
				println!("{} leveled up!", member.nick());
				terminal::wait();
			}
		}
		Effect::FlagsChange(_) =>
		{
			println!("Twisted the dimensions!");
			terminal::wait();
		}
		Effect::LingeringAdd(_) =>
		{
			match *battle.state().lingering().last().unwrap()
			{
				LingeringType::PerishSong(_) =>
				{
					println!("All monsters that hear the song will faint in three turns!");
				}
			}
			terminal::wait();
		}
		Effect::LingeringChange(ref lingering_change) =>
		{
			match battle.state().lingering()[lingering_change.index]
			{
				LingeringType::PerishSong(ref perish_song) =>
				{
					println!("X's perish count fell to {}!", perish_song.turns());
				}
			}
			terminal::wait();
		}
		Effect::None(ref reason) =>
		{
			match *reason
			{
				NoneReason::Turn =>
				{
					// Ignore.
				}
				NoneReason::None =>
				{
					println!("But nothing happened!");
					terminal::wait();
				}
				NoneReason::Miss =>
				{
					println!("It missed!");
					terminal::wait();
				}
				NoneReason::Escape =>
				{
					println!("You escaped!");
					terminal::wait();
				}
			}
		}
	}
}

fn battle_execute(battle: &mut Battle) -> bool
{
	loop
	{
		terminal::clear();
		display_active(battle, usize::max_value());

		let execute = battle.execute();
		match execute
		{
			BattleExecution::SwitchWaiting =>
			{
				for party_index in 0..battle.state().parties().len()
				{
					for active in 0..battle.state().parties()[party_index].active_count()
					{
						if battle.state().parties()[party_index].active_member(active).member.health() != 0
						{
							continue;
						}
						if party_index == 0
						{
							loop
							{
								let target = battle_prompt_switch(battle, active, 0, false);

								let err = battle.command_add_post_switch(0, active, target);
								if err != BattleError::None
								{
									display_error(err);
									terminal::wait();
									continue;
								}
								break;
							}
						}
						else
						{
							// TODO: Implement AI switching.
						}
					}
					
				}
			}
			BattleExecution::RetreatWaiting(ref retreat) =>
			{
				loop
				{
					let target = battle_prompt_switch(battle, retreat.member, 0, false);

					let err = battle.command_add_retreat(target);
					if err != BattleError::None
					{
						display_error(err);
						terminal::wait();
						continue;
					}
					break;
				}
			}
			BattleExecution::Command =>
			{
				match *battle.current_command()
				{
					CommandType::Attack(ref attack_command) =>
					{
						// TODO: Update these methods.
						let monster = &battle.state().parties()[
							attack_command.party].active_member(attack_command.member).member;
						let nick = monster.nick();
						let attack = attack_command.attack(battle).attack();
						let attack_name = attack.name();
						println!("{} used {}.", nick, attack_name);
						terminal::wait();
					}
					CommandType::Switch(_) | CommandType::Escape(_) | CommandType::Turn =>
					{
						// Ignore.
					}
				}
			}
			BattleExecution::Effect | BattleExecution::Death(_) =>
			{
				battle_execute_effect(battle);
			}
			BattleExecution::Waiting =>
			{
				return true;
			}
			BattleExecution::Ready =>
			{
				unreachable!();
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
				return false;
			}
		}
	}
}

pub fn main()
{
	// Initialize parties.
	let mut party_enemy =
	[
		Monster::new(SpeciesType::Deoxys, 50),
		Monster::new(SpeciesType::Deoxys, 9),
	];
	party_enemy[0].form_set(DeoxysForm::Defense as FormId);
	party_enemy[1].form_set(DeoxysForm::Defense as FormId);


	let mut pidgey = Monster::new(SpeciesType::Pidgey, 20);
	pidgey.attack_add(AttackType::Uturn);

	let mut party_self =
	[
		pidgey,
		Monster::new(SpeciesType::Bulbasaur, 55),
		Monster::new(SpeciesType::Charmander, 7),
		Monster::new(SpeciesType::Bulbasaur, 8),
		Monster::new(SpeciesType::Shaymin, 10),
		Monster::new(SpeciesType::Bulbasaur, 5),
	];
	
	let battle_data = vec!
	[
		Party::new(&mut party_self, 0, 2, true),
		Party::new(&mut party_enemy, 1, 2, false),
	];
	let mut battle = Battle::new(battle_data).unwrap();

	// Stores the active monster that the user is inputting commands for.
	let mut active = 0;

	'main: loop
	{
		terminal::clear();
		display_active(&battle, active);

		let exit_str = if active == 0
		{
			"4) Escape"
		}
		else
		{
			"4) Back"
		};
		println!("{:^20}{:^20}{:^20}{:^20}", "1) Move", "2) Item", "3) Switch", exit_str);
		println!("");
		println!("What will you do?");

		match terminal::input_range(4)
		{
			1 =>
			{
				terminal::clear();
				display_active(&battle, active);

				// Input range is greater than the number of attacks for an option to go back.
				let attack_amount =
				{
					let active_member = battle.state().parties()[0].active_member(active).member;
					let attack_list = active_member.attacks();
					display_attacks(attack_list);
					attack_list.len()
				} + 1;
				println!("\nChoose an attack to use:");

				let input = terminal::input_range(attack_amount);
				if input == attack_amount
				{
					continue;
				}
				else
				{
					loop
					{
						terminal::clear();
						display_active(&battle, active);

						println!("Choose a target");

						let target = battle_prompt_target(&battle);
						if let Some((target_party, target_member)) = target
						{
							let err = battle.command_add_attack(0, active, input - 1, target_party, target_member);
							if err != BattleError::None
							{
								display_error(err);
								terminal::wait();
								continue;
							}
							break;
						}
						else
						{
							continue 'main;
						}
					}
				}
			}
			2 =>
			{
				// TODO: Item use.
				println!("Items are unimplemented. Select another choice.");
				terminal::wait();
				continue;
			}
			3 =>
			{
				let target = battle_prompt_switch(&battle, active, 0, true);
				if target == battle.state().parties()[0].member_count()
				{
					continue;
				}

				let err = battle.command_add_switch(0, active, target);
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

		if active != battle.state().parties()[0].active_count() - 1
		{
			active += 1;
			continue;
		}

		active = 0;
		battle_random_ai(&mut battle, 1);

		if !battle_execute(&mut battle)
		{
			break;
		}
	}
}
