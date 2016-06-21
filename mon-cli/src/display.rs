use mon_gen::{Monster, Battle, Party};
use mon_gen::base::monster::MonsterAttack;

use std::str;

fn display(text: String, left: bool)
{
	if left
	{
		println!("{:>80}", text);
	}
	else
	{
		println!("{}", text);
	}
}

pub fn display_active(battle: &Battle, active: usize)
{
	println!("");
	for index in 0..battle.monster_active_count(1)
	{
		if let Some(monster) = battle.monster_active(1, index)
		{
			display_stats(monster, true, false);
		}
		else
		{
			display("---".to_string(), true);
			display("---\n".to_string(), true);
		}
	}
	for index in 0..battle.monster_active_count(0)
	{
		if let Some(monster) = battle.monster_active(0, index)
		{
			display_stats(monster, false, active == index);
		}
		else
		{
			display("---".to_string(), false);
			display("---\n".to_string(), false);
		}
	}
	println!("");
}

pub fn display_stats(monster: &Monster, opponent: bool, active: bool)
{
	let active_arrow = match active
	{
		true => "-> ",
		false => "",
	};
	let form_name = match monster.get_species().species().forms.len() != 0
	{
		true => format!(" ({})", str::from_utf8(monster.get_species().species().forms[monster.get_form() as usize]).unwrap()),
		false => "".to_string(),
	};
	display(format!("{}{}{} Lv. {}", active_arrow, str::from_utf8(monster.get_nick()).unwrap(),
		form_name, monster.get_level()), opponent);
	display(format!("{}HP: {}/{}\n", active_arrow, monster.get_health(),
		monster.get_stat_health()), opponent);
}

pub fn display_attacks(attacks: &[MonsterAttack])
{
	let mut alternate = true;
	for (index, attack) in attacks.iter().enumerate()
	{
		alternate = !alternate;
		display(format!("{}), {}", index + 1, str::from_utf8(attack.attack().name).unwrap()),
			alternate);
		display(format!("   {}", str::from_utf8(attack.attack().element.name()).unwrap()),
			alternate);
		display(format!("   Limit: {}/{}", attack.limit_left(), attack.limit_max()), alternate);
	}
	println!("");
	println!("{:>80}", format!("{}) {}", attacks.len() + 1, "Back"));
}

pub fn display_party(party: &Party, back: bool)
{
	println!("Party members:");
	println!("");

	let mut alternate = true;
	for (index, monster) in party.iter().enumerate()
	{
		alternate = !alternate;
		display(format!("{}) {} Lv. {}", index + 1, str::from_utf8(monster.get_nick()).unwrap(),
			monster.get_level()), alternate);
		display(format!("   HP: {}/{}", monster.get_health(), monster.get_stat_health()),
			alternate);
		display(format!("   ATK: {}, DEF: {}", monster.get_stat_attack(),
			monster.get_stat_defense()), alternate);
		display(format!("   SPATK: {}, SPDEF: {}", monster.get_stat_spattack(),
			monster.get_stat_spdefense()), alternate);
		display(format!("   SPD: {}", monster.get_stat_speed()), alternate);
	}
	println!("");
	if back
	{
		println!("{:>80}", format!("{}) {}", party.count() + 1, "Back"));
	}
}
