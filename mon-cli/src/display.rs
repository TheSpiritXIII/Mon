use mon_gen::{Monster, Battle};
use mon_gen::base::monster::MonsterAttack;
use mon_gen::base::battle::Party;

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
	for index in 0..battle.monster_active_count(1)
	{
		display_stats(battle.monster_active(1, index), true, false);
	}
	for index in 0..battle.monster_active_count(0)
	{
		display_stats(battle.monster_active(0, index), false, active == index);
	}
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
	println!("Attacks: {}", attacks.len());
	println!("");

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

pub fn display_party(party: &Party)
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
	println!("{:>80}", format!("{}) {}", party.count() + 1, "Back"));
}
