use base::battle::Battle;
use base::party::{Party, PartyMember};
use base::monster::MonsterAttack;
use base::effect::{Effect, Switch, NoneReason};

use rand::Rng;

use std::cmp::Ordering;

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
						attack_command_other.attack_index].attack().priority;
					let priority_self = monster_self.member.get_attacks()[
						attack_command_self.attack_index].attack().priority;
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
	pub fn effects<'a, R: Rng>(&self, parties: &[Party<'a>], command: &Command, rng: &mut R,
		effects: &mut Vec<Effect>)
	{
		match *self
		{
			CommandType::Attack(ref attack_command) =>
			{
				let offense = &parties[command.party].active_member(attack_command.member).unwrap();
				let attack = offense.member.get_attacks()[attack_command.attack_index].attack_type();
				attack.effects(attack_command, command.party, parties, effects, rng);
			}
			CommandType::Switch(ref switch_command) =>
			{
				let switch = Switch
				{
					member: switch_command.member,
					target: switch_command.target,
				};
				effects.push(Effect::Switch(switch));
			}
			CommandType::Escape =>
			{
				effects.push(Effect::None(NoneReason::Escape));
			}
		}
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
