use base::battle::Battle;
use base::party::{Party, PartyMember};
use base::monster::MonsterAttack;
use base::types::monster::StatType;

use calculate::damage::{calculate_damage, calculate_miss, calculate_critical};

use rand::Rng;

use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Command
{
	pub command_type: CommandType,
	party: usize,
}

impl Command
{
	pub fn new(command_type: CommandType, party: usize) -> Self
	{
		Command
		{
			command_type: command_type,
			party: party,
		}
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn cmp(command_self: &Command, command_other: &Command, battle: &Battle) -> Ordering
	{
		match command_self.command_type
		{
			CommandType::Attack(ref attack_command_self) =>
			{
				if let CommandType::Attack(ref attack_command_other) = command_other.command_type
				{
					let monster_other = attack_command_other.active_member(command_other, battle);
					let monster_self = attack_command_self.active_member(command_self, battle);
					let priority_other = monster_other.member.get_attacks()[
						attack_command_self.attack_index].attack().priority;
					let priority_self = monster_self.member.get_attacks()[
						attack_command_other.attack_index].attack().priority;
					let priority_cmp = priority_other.cmp(&priority_self);
					if priority_cmp == Ordering::Equal
					{
						monster_other.speed().cmp(&monster_self.speed())
					}
					else
					{
						priority_cmp
					}
				}
				else
				{
					Ordering::Greater
				}
			}
			CommandType::Switch(_) =>
			{
				if let CommandType::Switch(ref switch_command) = command_other.command_type
				{
					let group = command_self.party.cmp(&command_other.party);
					if group == Ordering::Equal
					{
						switch_command.member.cmp(&switch_command.member)
					}
					else
					{
						group
					}
				}
				else if let CommandType::Escape = command_other.command_type
				{
					Ordering::Greater
				}
				else
				{
					Ordering::Less
				}
			}
			CommandType::Escape =>
			{
				if let CommandType::Escape = command_other.command_type
				{
					command_self.party.cmp(&command_other.party)
				}
				else
				{
					Ordering::Less
				}
			}
		}
	}
}

#[derive(Debug)]
pub enum CommandType
{
	Attack(CommandAttack),
	// Item(CommandItem),
	Switch(CommandSwitch),
	Escape,
}

impl CommandType
{
	pub fn effect_if_not_miss<'a, R: Rng, F>(command: CommandAttack, party: usize,
		parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R, func: F)
		where F: Fn(CommandAttack, &Vec<Party<'a>>, &mut Vec<Effect>, &mut Rng)
	{
		let attacking_party = &parties[party];
		let attacking_member = &attacking_party.active_member(command.member).unwrap();
		if calculate_miss(attacking_member, command.attack_index, rng)
		{
			effects.push(Effect::None(Reason::Miss));
		}
		else
		{
			func(command, parties, effects, rng);
		}
	}
	pub fn damage_effect<'a, R: Rng>(command: CommandAttack, party: usize,
		parties: &Vec<Party<'a>>, effects: &mut Vec<Effect>, rng: &mut R)
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

		let is_critical = calculate_critical(attacking_member.modifiers.critical_stage(), rng);

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
	pub fn effects<'a, R: Rng>(&self, parties: &Vec<Party<'a>>, command: &Command, rng: &mut R) -> VecDeque<Effect>
	{
		let mut v = VecDeque::new();
		match *self
		{
			CommandType::Attack(ref attack_command) =>
			{
				let offense = &parties[command.party].active_member(attack_command.member).unwrap();
				if calculate_miss(offense, attack_command.attack_index, rng)
				{
					v.push_back(Effect::None(Reason::Miss));
				}
				else
				{
					// TODO: Cleanup.
					let defense = &parties[attack_command.target_party].active_member(attack_command.target_member).unwrap();

					// Element defense bonus.
					let mut type_bonus = 1f32;
					let attack = offense.member.get_attacks()[attack_command.attack_index].attack();
					for element in defense.member.get_elements()
					{
						type_bonus *= attack.element.effectiveness(*element);
					}

					// TODO: Move this into dedicated function.
					// TODO: Critical hit modifiers.
					let is_critical = calculate_critical(offense.modifiers.critical_stage(), rng);

					let amount = calculate_damage(offense, attack_command.attack_index, defense, is_critical, 1f32, rng);

					let damage = Damage
					{
						party: attack_command.target_party,
						active: attack_command.target_member,
						member: parties[attack_command.target_party].active_member_index(attack_command.target_member).unwrap(),
						meta: DamageMeta
						{
							amount: amount,
							type_bonus: type_bonus,
							critical: is_critical,
						}
					};
					v.push_back(Effect::Damage(damage));
				}

			}
			CommandType::Switch(ref switch_command) =>
			{
				let switch = Switch
				{
					member: switch_command.member,
					target: switch_command.target,
				};
				v.push_back(Effect::Switch(switch));
			}
			CommandType::Escape =>
			{
				v.push_back(Effect::None(Reason::Escape));
			},
		}
		v
	}
}

#[derive(Debug)]
pub struct CommandAttack
{
	pub member: usize,
	pub attack_index: usize,
	pub target_party: usize,
	pub target_member: usize,
}

impl CommandAttack
{
	fn active_member<'a>(&'a self, command: &Command, battle: &'a Battle) -> PartyMember
	{
		battle.monster_active(command.party, self.member).unwrap()
	}
	pub fn attack<'a>(&'a self, party: usize, battle: &'a Battle) -> &MonsterAttack
	{
		&battle.monster_active(party, self.member).unwrap().member.get_attacks()[self.attack_index]
	}
}

#[derive(Debug)]
pub struct CommandSwitch
{
	pub member: usize,
	pub target: usize,
}

#[derive(Debug)]
pub enum Effect
{
	Damage(Damage),
	Switch(Switch),
	// Status(StatusId),
	// Ability(AbilityId),
	// Miss,
	// ,
	None(Reason),
}

#[derive(Debug)]
pub struct Damage
{
	party: usize,
	pub active: usize,
	member: usize,
	meta: DamageMeta,
}

impl Damage
{
	pub fn amount(&self) -> StatType
	{
		self.meta.amount
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn member(&self) -> usize
	{
		self.member
	}
	pub fn critical(&self) -> bool
	{
		self.meta.critical
	}
	pub fn type_bonus(&self) -> f32
	{
		self.meta.type_bonus
	}
}

#[derive(Debug)]
pub struct DamageMeta
{
	amount: StatType,
	type_bonus: f32,
	critical: bool,
}

#[derive(Debug)]
pub struct Switch
{
	pub member: usize,
	pub target: usize,
}

#[derive(Debug)]
pub enum Reason
{
	Miss,
	Escape,
}
