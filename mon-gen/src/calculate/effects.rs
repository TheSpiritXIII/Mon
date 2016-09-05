use base::command::CommandAttack;
use base::battle::StatModifierType;
use base::party::Party;
use base::statmod::StatModifiers;
use base::effect::{Effect, NoneReason, Damage, DamageMeta, Modifier};

use types::monster::{LevelType, ExperienceType};

use calculate::damage::{calculate_miss, calculate_damage};

use rand::Rng;

use gen::species::Growth;

fn effect_if_not_miss<'a, R: Rng, F>(command: &CommandAttack, party: usize,
	parties: &[Party<'a>], effects: &mut Vec<Effect>, rng: &mut R, func: F)
	where F: Fn(&CommandAttack, usize, &[Party<'a>], &mut Vec<Effect>, &mut R)
{
	let attacking_party = &parties[party];
	let attacking_member = &attacking_party.active_member(command.member);
	if calculate_miss(attacking_member, command.attack_index, rng)
	{
		effects.push(Effect::None(NoneReason::Miss));
	}
	else
	{
		func(command, party, parties, effects, rng);
	}
}

fn is_critical<R: Rng>(stage: StatModifierType, high_chance: bool, rng: &mut R) -> bool
{
	let rate = match stage
	{
		0 => 32,
		1 => 16,
		2 => 8,
		_ => 4,
	};
	let odds = if high_chance
	{
		2
	}
	else
	{
		1
	};
	rng.gen::<u8>() % rate <= odds
}

fn damage_effect<'a, R: Rng>(command: &CommandAttack, party: usize,
	parties: &[Party<'a>], effects: &mut Vec<Effect>, rng: &mut R, high_critical: bool)
{
	let attacking_party = &parties[party];
	let defending_party = &parties[command.target_party];
	let attacking_member = &attacking_party.active_member(command.member);
	let defending_member = &defending_party.active_member(command.target_member);

	// Element defense bonus.
	let mut type_bonus = 1f32;
	let attack = attacking_member.member.attacks()[command.attack_index].attack();
	for element in defending_member.member.get_elements()
	{
		type_bonus *= attack.element.effectiveness(*element);
	}

	let is_critical = is_critical(attacking_member.modifiers.critical_stage(), high_critical, rng);

	let amount = calculate_damage(attacking_member, command.attack_index, defending_member,
		is_critical, type_bonus, rng);

	let damage = Damage
	{
		party: command.target_party,
		active: command.target_member,
		member: defending_party.active_member_index(command.target_member),
		meta: DamageMeta
		{
			amount: amount,
			type_bonus: type_bonus,
			critical: is_critical,
		}
	};
	effects.push(Effect::Damage(damage));
}

pub fn default_effect<'a, R: Rng>(command: &CommandAttack, party: usize, parties: &[Party<'a>],
	effects: &mut Vec<Effect>, rng: &mut R)
{
	effect_if_not_miss(command, party, parties, effects, rng, |command, party, parties, effects, rng|
	{
		damage_effect(command, party, parties, effects, rng, false);
	});
}

// pub fn high_critical_effect<'a, R: Rng>(command: &CommandAttack, party: usize,
// 	parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R)
// {
// 	effect_if_not_miss(command, party, parties, effects, rng, |command, party, parties, effects, rng|
// 	{
// 		damage_effect(command, party, parties, effects, rng, true);
// 	});
// }

fn stat_modifier_effect<'a, R: Rng, F>(command: &CommandAttack, party: usize,
	parties: &[Party<'a>], effects: &mut Vec<Effect>, rng: &mut R, modifier_func: F)
		where F: Fn(&mut StatModifiers)
{
	effect_if_not_miss(command, party, parties, effects, rng, |command, _, _, effects, _|
	{
		let mut stats = Default::default();
		modifier_func(&mut stats);
		let modifier = Modifier::new(command.target_party, command.target_member, stats);
		effects.push(Effect::Modifier(modifier));
	});
}

pub fn decrease_attack_stage_1<'a, R: Rng>(command: &CommandAttack, party: usize,
	parties: &[Party<'a>], effects: &mut Vec<Effect>, rng: &mut R)
{
	stat_modifier_effect(command, party, parties, effects, rng, move |modifier: &mut StatModifiers|
	{
		 modifier.attack_delta(-1);
	});
}

// TODO: This doesn't belong here but I'm very sleepy.
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

// pub fn damage_retreat<'a, R: Rng>(command: &CommandAttack, party: usize, parties: &[Party<'a>],
// 	effects: &mut Vec<Effect>, rng: &mut R)
// {
// 	effect_if_not_miss(command, party, parties, effects, rng, |command, party, parties, effects, rng|
// 	{
// 		damage_effect(command, party, parties, effects, rng, false);
// 		effects.push(Effect::Retreat(Retreat
// 		{
// 			member: command.member
// 		}));
// 	});
// }
