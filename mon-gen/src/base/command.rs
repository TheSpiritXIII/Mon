// use base::battle::Battle;
use base::party::Party;
use base::monster::MonsterAttack;
use base::effect::{Effect, Switch, NoneReason};
use base::battle::Battle;

use rand::Rng;

use std::cmp::Ordering;

impl CommandType
{
	pub fn cmp(&self, other: &CommandType, parties: &[Party]) -> Ordering
	{
		match *self
		{
			CommandType::Attack(ref attack_command_self) =>
			{
				if let CommandType::Attack(ref attack_command_other) = *other
				{
					let monster_other = parties[attack_command_other.party].active_member(attack_command_other.member);//attack_command_other.active_member(command_other, parties);
					let monster_self = parties[attack_command_self.party].active_member(attack_command_self.member);//attack_command_self.active_member(command_self, parties);
					let priority_other = monster_other.member.attacks()[
						attack_command_other.attack_index].attack().priority;
					let priority_self = monster_self.member.attacks()[
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
			CommandType::Switch(ref switch_command_self) =>
			{
				if let CommandType::Switch(ref switch_command_other) = *other
				{
					let group = switch_command_self.party.cmp(&switch_command_other.party);
					if group == Ordering::Equal
					{
						switch_command_other.member.cmp(&switch_command_other.member)
					}
					else
					{
						group
					}
				}
				else if let CommandType::Escape(_) = *other
				{
					Ordering::Greater
				}
				else
				{
					Ordering::Less
				}
			}
			CommandType::Escape(ref escape_self) =>
			{
				if let CommandType::Escape(ref escape_other) = *other
				{
					escape_self.party.cmp(&escape_other.party)
				}
				else
				{
					Ordering::Less
				}
			}
			CommandType::Turn =>
			{
				// TODO: This comparison function is actually non-deterministics if you switch sort functions.
				// Retreat and turn should be moved out of here eventually.
				Ordering::Less
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandType
{
	Attack(CommandAttack),
	// Item(CommandItem),
	Switch(CommandSwitch),
	Escape(CommandEscape),
	Turn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommandEscape
{
	pub party: usize,
}

impl CommandType
{
	pub fn effects<'a, R: Rng>(&self, parties: &[Party<'a>], rng: &mut R, effects: &mut Vec<Effect>)
	{
		match *self
		{
			CommandType::Attack(ref attack_command) =>
			{
				let offense = &parties[attack_command.party].active_member(attack_command.member);
				let attack = offense.member.attacks()[attack_command.attack_index].attack_type();
				attack.effects(attack_command, attack_command.party, parties, effects, rng);
			}
			CommandType::Switch(ref switch_command) =>
			{
				let switch = Switch
				{
					party: switch_command.party,
					member: switch_command.member,
					target: switch_command.target,
				};
				effects.push(Effect::Switch(switch));
			}
			CommandType::Escape(_) =>
			{
				effects.push(Effect::None(NoneReason::Escape));
			}
			CommandType::Turn =>
			{
				effects.push(Effect::None(NoneReason::Turn));
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommandAttack
{
	pub party: usize,
	pub member: usize,
	pub attack_index: usize,
	pub target_party: usize,
	pub target_member: usize,
}

impl CommandAttack
{
	pub fn attack<'a>(&'a self, battle: &'a Battle) -> &MonsterAttack
	{
		&battle.runner().parties()[self.party].active_member(self.member).member.attacks()[self.attack_index]
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommandSwitch
{
	pub party: usize,
	pub member: usize,
	pub target: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommandRetreat
{
	pub target: usize,
}
