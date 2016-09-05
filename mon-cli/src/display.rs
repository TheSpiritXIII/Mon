use mon_gen::monster::{Monster, MonsterAttack};
use mon_gen::battle::{Party, Battle, BattleError};
use mon_gen::experimental;

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
		if let Some(monster) = battle.monster_active_alive(1, index)
		{
			display_stats(monster.member, true, false);
		}
		else
		{
			display("---".to_string(), true);
			display("---".to_string(), true);
			println!("");
		}
	}
	for index in 0..battle.monster_active_count(0)
	{
		if let Some(monster) = battle.monster_active_alive(0, index)
		{
			display_stats(monster.member, false, active == index);
		}
		else
		{
			display("---".to_string(), false);
			display("---".to_string(), false);
			println!("");
		}
	}
	println!("");
}

pub fn display_active_experimental(battle: &experimental::Battle, active: usize)
{
	println!("");
	for index in 0..battle.runner().parties()[1].active_count()
	{
		if let Some(monster) = battle.runner().parties()[1].active_member_alive(index)
		{
			display_stats(monster.member, true, false);
		}
		else
		{
			display("---".to_string(), true);
			display("---".to_string(), true);
			println!("");
		}
	}
	for index in 0..battle.runner().parties()[0].active_count()
	{
		if let Some(monster) = battle.runner().parties()[0].active_member_alive(index)
		{
			display_stats(monster.member, false, active == index);
		}
		else
		{
			display("---".to_string(), false);
			display("---".to_string(), false);
			println!("");
		}
	}
	println!("");
}

pub fn display_stats(monster: &Monster, opponent: bool, active: bool)
{
	let active_arrow = if active
	{
		"-> "
	}
	else
	{
		""
	};
	let form_name = if monster.species().species().forms.len() > 1
	{
		format!(" ({})", monster.species().species().form(monster.form() as usize))
	}
	else
	{
		String::new()
	};
	display(format!("{}{}{} Lv. {}", active_arrow, monster.nick(), form_name, monster.level()),
		opponent);
	display(format!("{}HP: {}/{}", active_arrow, monster.health(), monster.stat_health()),
		opponent);
	println!("");
}

pub fn display_attacks(attacks: &[MonsterAttack])
{
	let mut alternate = true;
	for (index, attack) in attacks.iter().enumerate()
	{
		alternate = !alternate;
		display(format!("{}) {}", index + 1, attack.attack().name()), alternate);
		display(format!("   {}", attack.attack().element.name()), alternate);
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
		display(format!("{}) {} Lv. {}", index + 1, monster.nick(), monster.level()),
			alternate);
		display(format!("   HP: {}/{}", monster.health(), monster.stat_health()),
			alternate);
		display(format!("   ATK: {}, DEF: {}", monster.stat_attack(),
			monster.stat_defense()), alternate);
		display(format!("   SPATK: {}, SPDEF: {}", monster.stat_spattack(),
			monster.stat_spdefense()), alternate);
		display(format!("   SPD: {}", monster.stat_speed()), alternate);
	}
	println!("");
	if back
	{
		println!("{:>80}", format!("{}) {}", party.member_count() + 1, "Back"));
	}
}

/// Returns a descriptive string of the given battle error.
pub fn display_error(err: BattleError)
{
	let error_str = match err
	{
		BattleError::None =>
		{
			unreachable!();
		}
		BattleError::Blocking =>
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
	};
	println!("Invalid selection: {}", error_str);
}

/// Returns a descriptive string of the given battle error.
pub fn display_error_experimental(err: experimental::BattleError)
{
	let error_str = match err
	{
		experimental::BattleError::None =>
		{
			unreachable!();
		}
		experimental::BattleError::Rejected =>
		{
			unreachable!();
		}
		experimental::BattleError::AttackLimit =>
		{
			"Selected move has no PP left."
		}
		experimental::BattleError::AttackTarget =>
		{
			"Invalid target."
		}
		experimental::BattleError::SwitchActive =>
		{
			"Selected party member is already active."
		}
		experimental::BattleError::SwitchHealth =>
		{
			"Selected party member has no health."
		}
		experimental::BattleError::SwitchQueued =>
		{
			"Selected party member is already queued to switch in."
		}
	};
	println!("Invalid selection: {}", error_str);
}
