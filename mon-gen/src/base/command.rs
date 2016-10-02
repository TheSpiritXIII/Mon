// use base::battle::Battle;
use base::party::Party;
use base::monster::MonsterAttack;
use base::effect::{Effect, Switch, NoneReason};
use base::battle::Battle;

use rand::Rng;

use std::cmp::Ordering;
use std::collections::VecDeque;

// The battle flags value type for `BattleFlags`.
pub type BattleFlagsType = u8;

// Constants for battle setting bitflags.
pub struct BattleFlags;

impl BattleFlags
{
	// Reverses the priority order so that attacks with lower priority go first.
	pub const PRIORITY_REVERSE: BattleFlagsType = 0b01;

	// Reverses the speed order so that monsters with a slower speed go first.
	pub const SPEED_REVERSE: BattleFlagsType = 0b10;
}

impl CommandType
{
	pub fn cmp(&self, other: &CommandType, parties: &[Party], flags: BattleFlagsType) -> Ordering
	{
		match *self
		{
			CommandType::Attack(ref attack_command_self) =>
			{
				if let CommandType::Attack(ref attack_command_other) = *other
				{
					attack_command_self.cmp(attack_command_other, parties, flags)
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
	pub fn effects<'a, R: Rng>(&self, parties: &[Party<'a>], rng: &mut R,
		effects: &mut VecDeque<Effect>)
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
				effects.push_back(Effect::Switch(switch));
			}
			CommandType::Escape(_) =>
			{
				effects.push_back(Effect::None(NoneReason::Escape));
			}
			CommandType::Turn =>
			{
				effects.push_back(Effect::None(NoneReason::Turn));
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
	pub fn cmp(&self, other: &CommandAttack, parties: &[Party], flags: BattleFlagsType) -> Ordering
	{
		let monster_other = parties[other.party].active_member(other.member);
		let monster_self = parties[self.party].active_member(self.member);
		let monster_priority_cmp = monster_other.priority().cmp(&monster_self.priority());
		if monster_priority_cmp == Ordering::Equal
		{
			let attack_priority_other = monster_other.member.attacks()[
				other.attack_index].attack().priority;
			let attack_priority_self = monster_self.member.attacks()[
				self.attack_index].attack().priority;

			let attack_priority_cmp = if flags & BattleFlags::PRIORITY_REVERSE == 0
			{
				attack_priority_other.cmp(&attack_priority_self)
			}
			else
			{
				attack_priority_self.cmp(&attack_priority_other)
			};

			if attack_priority_cmp == Ordering::Equal
			{
				if flags & BattleFlags::SPEED_REVERSE == 0
				{
					monster_other.speed().cmp(&monster_self.speed())
				}
				else
				{
					monster_self.speed().cmp(&monster_other.speed())
				}
			}
			else
			{
				attack_priority_cmp
			}
		}
		else
		{
			monster_priority_cmp
		}
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
