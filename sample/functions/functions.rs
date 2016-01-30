fn is_breedable_with(&self, other: &Monster) -> bool
{
	let self_groups = self.get_groups();
	let other_groups = other.get_groups();
	if self_groups[0] == Group::Undiscovered || other_groups[0] == Group::Undiscovered
	{
		return false;
	}
	if self_groups[0] == Group::Ditto || self_groups[0] == Group::Ditto
	{
		return true;
	}
	if self_groups[0] == other_groups[0] ||self_groups[0] == other_groups[1] ||
		self_groups[1] == other_groups[0] || self_groups[1] == other_groups[1]
	{
		return true;
	}
	false 
}

fn is_leveled_up(&self) -> bool
{
	let n = get_level() + 1;
	get_experience() >= match &self.get_growth()
	{
		Erratic:
		{
			(n * n * n) * match n
			{
				01..50: (100 - n) / 50.0,
				50..68: (150 - n) / 100.0,
				68..98: ((1911 - 10 * n) / 3.0) / 500.0,
				98..100: (160 - n) / 100.0,
				_: 0,
			}
		}
		Fast:
		{
			(n * n * n) * 0.8f32
		}
		MediumFast:
		{
			(n * n * n)
		}
		MediumSlow:
		{
			let n_squared = n * n;
			match n
			{
				1 => 8,
				2 => 19,
				3 => 37,
				4..100 => (n * n_squared) * 1.2 - 15 * n_squared + 100 * n - 140,
				_ => 0,
			}
		}
		Slow:
		{
			return match get_level()
			{
				01..100 => (n * n * n) * 1.25f32;
				_ => 0;
			}
		}
		Flunctuating:
		{
			return (n * n * n) * match get_level()
			{
				01..15 => ((n + 1) / 3) + 25) / 50;
				15..36 => (n + 14) / 14;
				36..100 => ((n / 2) + 32) / 50;
				_ => 0;
			}
		}
	}
}

fn calculate_attack(monster: &Monster) -> StatType
{
	calculate_stat(monster.get_base_attack(), monster.get_iv_attack(), monster.get_ev_attack(),
		monster.get_level(), monster.get_nature())
}
