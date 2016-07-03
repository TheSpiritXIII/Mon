use base::command::{CommandAttack, Effect, Reason, Damage, DamageMeta, Modifier};
use base::types::battle::StatModifierType;
use base::party::Party;
use base::statmod::StatModifiers;

use calculate::damage::{calculate_miss, calculate_damage};

use rand::Rng;

fn effect_if_not_miss<'a, R: Rng, F>(command: &CommandAttack, party: usize,
	parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R, func: F)
	where F: Fn(&CommandAttack, usize, &Vec<Party<'a>>, &mut Vec<Effect>, &mut R)
{
	let attacking_party = &parties[party];
	let attacking_member = &attacking_party.active_member(command.member).unwrap();
	if calculate_miss(attacking_member, command.attack_index, rng)
	{
		effects.push(Effect::None(Reason::Miss));
	}
	else
	{
		func(&command, party, parties, effects, rng);
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
	let odds = match high_chance
	{
		true =>  2,
		false => 1,
	};
	rng.gen::<u8>() % rate <= odds
}

fn damage_effect<'a, R: Rng>(command: &CommandAttack, party: usize,
	parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R, high_critical: bool)
{
	let attacking_party = &parties[party];
	let defending_party = &parties[command.target_party];
	let attacking_member = &attacking_party.active_member(command.member).unwrap();
	let defending_member = &defending_party.active_member(command.target_member).unwrap();

	// Element defense bonus.
	let mut type_bonus = 1f32;
	let attack = attacking_member.member.get_attacks()[command.attack_index].attack();
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
		member: defending_party.active_member_index(command.target_member).unwrap(),
		meta: DamageMeta
		{
			amount: amount,
			type_bonus: type_bonus,
			critical: is_critical,
		}
	};
	effects.push(Effect::Damage(damage));
}

pub fn default_effect<'a, R: Rng>(command: &CommandAttack, party: usize,
	parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R)
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

pub fn decrease_attack_stage_1<'a, R: Rng>(command: &CommandAttack, party: usize,
	parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R)
{
	effect_if_not_miss(command, party, parties, effects, rng, |command, _, _, effects, _|
	{
		let mut stats = StatModifiers::new();
		stats.attack_delta(1);
		let modifier = Modifier::new(command.target_party, command.target_member, stats);
		effects.push(Effect::Modifier(modifier));
	});
}
