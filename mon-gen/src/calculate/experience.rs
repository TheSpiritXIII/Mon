use std::collections::HashMap;

use base::party::Party;
use gen::species::Growth;
use types::monster::{ExperienceType, LevelType};

pub struct MemberIndex
{
	pub party: usize,
	pub member: usize,
}

impl Growth
{
	pub fn experience_with_level(&self, level: LevelType) -> ExperienceType
	{
		let n = level as f32;
		let exp = match *self
		{
			Growth::Erratic =>
			{
				(n * n * n) * match level
				{
					 1 ... 50 => (100f32 - n) / 50f32,
					51 ... 68 => (150f32 - n) / 100f32,
					69 ... 98 => ((1911f32 - 10f32 * n) / 3f32) / 500f32,
					99 ... 100 => (160f32 - n) / 100f32,
					_ => 0f32,
				}
			}
			Growth::Fast =>
			{
				n * n * n * 0.8f32
			}
			Growth::MediumFast =>
			{
				n * n * n
			}
			Growth::MediumSlow =>
			{
				match level
				{
					1 => 8f32,
					2 => 19f32,
					3 => 37f32,
					4 ... 100 =>
					{
						let n_squared = n * n;
						1.2f32 * n_squared * n - 15f32 * n_squared + 100f32 * n - 140f32
					}
					_ => 0f32,
				}
			}
			Growth::Slow =>
			{
				match level
				{
					1 ... 100 => n * n * n * 1.25f32,
					_ => 0f32,
				}
			}
			Growth::Fluctuating =>
			{
				n * n * n * match level
				{
					 1 ... 15 => (((n + 1f32) / 3f32) + 25f32) / 50f32,
					16 ... 36 => (n + 14f32) / 14f32,
					37 ... 100 => ((n / 2f32) + 32f32) / 50f32,
					_ => 0f32,
				}
			}
		};
		exp.floor() as ExperienceType
	}
}

pub fn calculate_experience(parties: &[Party], _: Option<MemberIndex>,
	defense: MemberIndex) -> HashMap<usize, HashMap<usize, ExperienceType>>
{
	// TODO: Bonus is 1.5 if battling a non-wild trainer.
	let bonus = 1.0f32;

	let defense_member = parties[defense.party].active_member(defense.member).member;
	let base_yield = defense_member.species().species().experience_yield as f32;
	let level = defense_member.level() as f32;

	// TODO: Bigger bonus if monster is traded.
	let trade_bonus = 1.0f32;

	let gain = ((bonus * trade_bonus * base_yield * level) / 7f32).round() as ExperienceType;

	let mut party_map = HashMap::new();

	let exposed = parties[defense.party].expose_get_member(defense.member);
	for exposed_party in exposed
	{
		let member_map = party_map.entry(*exposed_party.0).or_insert_with(HashMap::new);
		for exposed_member in exposed_party.1
		{
			member_map.insert(*exposed_member, gain);
		}
	}

	party_map
}
