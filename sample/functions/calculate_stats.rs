use super::{Monster, StatType, BaseStatType, IvType, EvType, LevelType, Nature};

pub fn calculate_health(monster: &Monster) -> StatType
{
	let base = monster.get_base_health() as f32;
	let iv = monster.get_iv_health() as f32;
	let level = monster.get_level() as f32;
	let ev = monster.get_ev_health() as f32;
	((((2_f32 * base + iv + (ev / 4_f32)) * level) / 100_f32) + level + 10_f32).trunc() as StatType
}

fn calculate_stat(base: BaseStatType, iv: IvType, ev: EvType, level: LevelType, bonus: f32)
	-> StatType
{
	let stat = ((((2_f32 * base as f32 + iv as f32 + (ev as f32 / 4_f32)) * level as f32) /
		100_f32) + 5_f32).trunc();
	(stat * bonus).trunc() as StatType
}

fn nature_bonus_attack(nature: Nature) -> f32
{
	match nature
	{
		Nature::Lonely | Nature::Brave | Nature::Adamant | Nature::Naughty => 1.1f32,
		Nature::Bold | Nature::Timid | Nature::Modest | Nature::Calm => 0.9f32, 
		_ => 1.0f32
	}
}

fn nature_bonus_defense(nature: Nature) -> f32
{
	match nature
	{
		Nature::Bold | Nature::Impish | Nature::Lax | Nature::Relaxed => 1.1f32,
		Nature::Lonely | Nature::Mild | Nature::Gentle | Nature::Hasty => 0.9f32, 
		_ => 1.0f32
	}
}

fn nature_bonus_spattack(nature: Nature) -> f32
{
	match nature
	{
		Nature::Modest | Nature::Mild | Nature::Rash | Nature::Quiet => 1.1f32,
		Nature::Adamant | Nature::Impish | Nature::Careful | Nature::Jolly => 0.9f32, 
		_ => 1.0f32
	}
}

fn nature_bonus_spdefense(nature: Nature) -> f32
{
	match nature
	{
		Nature::Calm | Nature::Gentle | Nature::Careful | Nature::Sassy => 1.1f32,
		Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Naive => 0.9f32, 
		_ => 1.0f32
	}
}

fn nature_bonus_speed(nature: Nature) -> f32
{
	match nature
	{
		Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => 1.1f32,
		Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => 0.9f32, 
		_ => 1.0f32
	}
}

pub fn calculate_attack(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_attack(), monster.get_iv_attack(),
		monster.get_ev_attack(), monster.get_level(), nature_bonus_attack(monster.get_nature()))
}

pub fn calculate_defense(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_defense(), monster.get_iv_defense(),
		monster.get_ev_defense(), monster.get_level(), nature_bonus_defense(monster.get_nature()))
}

pub fn calculate_spattack(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_spattack(), monster.get_iv_spattack(),
		monster.get_ev_spattack(), monster.get_level(), nature_bonus_spattack(monster.get_nature()))
}

pub fn calculate_spdefense(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_spdefense(), monster.get_iv_spdefense(),
		monster.get_ev_spdefense(), monster.get_level(), nature_bonus_spdefense(monster.get_nature()))
}

pub fn calculate_speed(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_speed(), monster.get_iv_speed(),
		monster.get_ev_speed(), monster.get_level(), nature_bonus_speed(monster.get_nature()))
}

