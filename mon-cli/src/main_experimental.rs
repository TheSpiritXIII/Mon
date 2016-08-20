use mon_gen::monster::Monster;
use mon_gen::species::{SpeciesType, FormId};
use mon_gen::species::form::DeoxysForm;
use mon_gen::battle::{Party};
use mon_gen::experimental::{Battle, BattleError};

use terminal;
use display::{display_active_experimental, display_error_experimental, display_party, display_attacks};

/// Prompts the user to switch party members and returns the selected member if possible.
///
/// If `back` is true, then the user will be able to select an input equal to the number of party
/// members indicating that the user does not want to switch anymore.
///
fn battle_prompt_switch(battle: &Battle, party: usize, back: bool) -> usize
{
	display_party(battle.runner().party(party), back);
	println!("\nChoose a party member to switch to:");
	let member_count = battle.runner().party(party).member_count() + if back
	{
		1
	}
	else
	{
		0
	};
	terminal::input_range(member_count) - 1
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
	let mut battle = Battle::new(battle_data).unwrap();

	// Stores the active monster that the user is inputting commands for.
	let mut active = 0;

	loop
	{
		terminal::clear();
		display_active_experimental(&battle, active);

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

		match terminal::input_range(4)
		{
			1 =>
			{
				let attack_amount =
				{
					let attack_list = battle.runner().party(0).active_member(active).member.attacks();
					display_attacks(attack_list);
					attack_list.len()
				};
				println!("\nChoose an attack to use:");

				// Input range is greater than the number of attacks for an option to go back.
				let input = terminal::input_range(attack_amount + 1);
				if input == attack_amount
				{
					continue;
				}
				else
				{
					let err = battle.command_add_attack(0, active, 1, 0, input - 1);
					if err != BattleError::None
					{
						display_error_experimental(err);
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
				continue;
			}
			3 =>
			{
				let target = battle_prompt_switch(&battle, 0, true);
				if target == battle.runner().party(0).member_count()
				{
					continue;
				}

				let err = battle.command_add_switch(0, active, target);
				if err != BattleError::None
				{
					display_error_experimental(err);
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

		active += 1;
	}
}
