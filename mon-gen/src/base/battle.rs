use std::slice;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::num::Wrapping;

use rand::{Rng, StdRng};

use base::types::monster::StatType;
use base::monster::{Monster, MonsterAttack};
use base::attack::target;

use calculate::damage::calculate_damage;

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],
	side: u8,
	active: Vec<usize>,
}

impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], out: usize) -> Self
	{
		let mut party = Party
		{
			members: members,
			side: 1,
			active: Vec::with_capacity(out),
		};

		let mut current = Wrapping(usize::max_value());
		for _ in 0..party.active.capacity()
		{
			current = Wrapping(party.next_alive((current + Wrapping(1usize)).0));
			if current.0 == party.members.len()
			{
				break;
			}
			party.active.push(current.0);
			if party.active.len() == party.active.capacity()
			{
				break;
			}
		}
		party
	}
	fn next_alive(&self, party: usize) -> usize
	{
		for member_index in party..self.members.len()
		{
			if self.members[member_index].get_health() != 0 &&
				!self.active.contains(&member_index)
			{
				return member_index;
			}
		}
		self.members.len()
	}
	pub fn active_member(&self, index: usize) -> &Monster
	{
		&self.members[self.active[index]]
	}
	pub fn active_count(&self) -> usize
	{
		self.active.len()
	}
	pub fn iter(&self) -> slice::Iter<Monster>
	{
		self.members.iter()
	}
	pub fn count(&self) -> usize
	{
		self.members.len()
	}
}

#[derive(Debug)]
pub struct CommandAttack
{
	pub member: usize,
	attack_index: usize,
	target_party: usize,
	target_member: usize,
}

impl CommandAttack
{
	fn active_member<'a>(&'a self, command: &Command, battle: &'a Battle) -> &Monster
	{
		battle.monster_active(command.party, self.member)
	}
	fn attack<'a>(&'a self, party: usize, battle: &'a Battle) -> &MonsterAttack
	{
		&battle.monster_active(party, self.member).get_attacks()[self.attack_index]
	}
}

#[derive(Debug)]
pub struct CommandSwitch
{
	member: usize,
	target: usize,
}

#[derive(Debug)]
pub enum CommandType
{
	Attack(CommandAttack),
	//Item(ItemId),
	Switch(CommandSwitch),
	Escape,
}

#[derive(Debug)]
pub struct Command
{
	pub command_type: CommandType,
	pub party: usize,
}

impl Command
{
	fn affects_member(&self, member: usize) -> bool
	{
		match self.command_type
		{
			CommandType::Attack(ref attack_command) =>
			{
				attack_command.member == member
			}
			CommandType::Switch(ref switch_command) =>
			{
				switch_command.member == member
			}
			CommandType::Escape =>
			{
				false
			}
		}
	}
}

impl Command
{
	fn new(command_type: CommandType, party: usize) -> Self
	{
		Command
		{
			command_type: command_type,
			party: party,
		}
	}
}

impl Command
{
	fn cmp(command_self: &Command, command_other: &Command, battle: &Battle) -> Ordering
	{
		match command_self.command_type
		{
			CommandType::Attack(ref attack_command_self) =>
			{
				if let CommandType::Attack(ref attack_command_other) = command_other.command_type
				{
					let monster_other = attack_command_other.active_member(command_other, battle);
					let monster_self = attack_command_self.active_member(command_self, battle);
					monster_other.get_stat_speed().cmp(&monster_self.get_stat_speed())
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

impl CommandType
{
	fn effects<'a, R: Rng>(&self, parties: &Vec<Party<'a>>, command: &Command, rng: &mut R) -> VecDeque<Effect>
	{
		let mut v = VecDeque::new();
		match *self
		{
			CommandType::Attack(ref attack_command) =>
			{
				let offense = &parties[command.party].members[attack_command.member];
				// TODO: Cleanup.
				let defense = &parties[attack_command.target_party].members[attack_command.target_member];
				let damage = Damage
				{
					amount: calculate_damage(offense, 0, defense, false, 1f32, rng),
					party: attack_command.target_party,
					member: attack_command.target_member
				};
				v.push_back(Effect::Damage(damage));
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
pub struct Damage
{
	amount: StatType,
	party: usize,
	member: usize,
}

impl Damage
{
	pub fn amount(&self) -> StatType
	{
		self.amount
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn member(&self) -> usize
	{
		self.member
	}
}

#[derive(Debug)]
pub struct Switch
{
	member: usize,
	target: usize,
}

#[derive(Debug)]
pub enum Reason
{
	Critical,
	Escape,
}

#[derive(Debug)]
pub enum Effect
{
	Damage(Damage),
	Switch(Switch),
	// Status(StatusId),
	// Bonus(BonusType),
	// Ability(AbilityId),
	// Miss,
	// ,
	// Uneffective,
	// LessEffective,
	// SuperEffecitve,
	None(Reason),
}

#[derive(Debug)]
struct BattleCommand
{
	command: Command,
	effects: VecDeque<Effect>,
}

impl BattleCommand
{
	fn new<'a, R: Rng>(command: Command, parties: &Vec<Party<'a>>, rng: &mut R) -> Self
	{
		let effects = command.command_type.effects(parties, &command, rng);
		BattleCommand
		{
			effects: effects,
			command: command,
		}
	}
}

pub struct Battle<'a>
{
	/// Maps to the groups that have already added a command.
	ready: Vec<Vec<Option<usize>>>,

	/// True if the turn has started, false otherwise.
	started: bool,

	/// The total number of available participants.
	total: usize,

	/// The number of groups still waiting for a command.
	waiting: usize,

	/// The parties in this battle.
	parties: Vec<Party<'a>>,

	/// The list of executed commands, for playback.
	commands: Vec<BattleCommand>,

	/// The queue of upcoming commands in the next turn.
	queue: Vec<Command>,

	/// The current effect being executed.
	current: usize,

	rng: StdRng,

	switch_queue: Option<(usize, usize)>,

	// TODO: lingering effects.
}

pub enum BattleExecution
{
	Command,
	Queue,
	Waiting,
	Switch(usize),
}

/// Indicates an error adding a command to a battle.
#[derive(Debug, PartialEq)]
pub enum BattleError
{
	/// There was no error.
	None,
	/// Occurs when the battle turn is in progress.
	Blocking,
	/// Occurs when the chosen attack is unable to be used due to the limit.
	Limit,
	/// Occurs when the chosen attack is unable to target the chosen party and respective member.
	Target,
	/// Occurs when a switch cannot occur.because the target is already active.
	Active,
	/// Occurs when a switch cannot occur because the target has no health.
	Health,
	/// Occurs when a switch cannot occur because the target has already been queued to switch.
	Queued,
	/// Occurs when an escape cannot occur because the party has already added commands.
	Escape,
}

fn is_adjacent_with(to: usize, from: usize) -> bool
{
	to == from || (to > 0 && to - 1 == from) || (to < usize::max_value() && to + 1 == from)
}

impl<'a> Battle<'a>
{
	pub fn new(parties: Vec<Party<'a>>) -> Self
	{
		let mut total = 0;
		let mut ready = Vec::with_capacity(parties.len());
		for group in parties.iter()
		{
			total += group.active_count();
			ready.push(vec![None; group.active_count()]);
		}
		Battle
		{
			ready: ready,
			started: false,
			total: total,
			waiting: total,
			parties: parties,
			commands: Vec::new(),
			queue: Vec::new(),
			current: 0,
			rng: StdRng::new().unwrap(),
			switch_queue: None,
		}
	}
	pub fn party(&self, index: usize) -> &Party<'a>
	{
		&self.parties[index]
	}
	pub fn monster(&self, party: usize, monster: usize) -> &Monster
	{
		&self.parties[party].members[monster]
	}
	pub fn monster_active(&self, party: usize, monster: usize) -> &Monster
	{
		self.parties[party].active_member(monster)
	}
	pub fn monster_is_active(&self, party: usize, monster: usize) -> bool
	{
		self.parties[party].active.contains(&monster)
	}
	pub fn monster_active_count(&self, party: usize) -> usize
	{
		self.parties[party].active_count()
	}
	fn is_member_valid(&mut self, party: usize, member: usize) -> BattleError
	{
		assert!(party < self.parties.len());
		assert!(member < self.parties[party].members.len());
		BattleError::None
	}
	fn is_switch_valid(&mut self, party: usize, member: usize) -> BattleError
	{
		let err = self.is_member_valid(party, member);
		if err != BattleError::None
		{
			err
		}
		else if self.monster_is_active(party, member)
		{
			BattleError::Active
		}
		else if self.monster(party, member).get_health() == 0
		{
			BattleError::Health
		}
		else
		{
			BattleError::None
		}
	}
	fn is_command_valid(&mut self, party: usize, member: usize) -> BattleError
	{
		if self.started
		{
			BattleError::Blocking
		}
		else
		{
			self.is_member_valid(party, member)
		}
	}
	pub fn add_command_attack(&mut self, party: usize, member: usize, target_party: usize,
		target_member: usize, attack_index: usize) -> BattleError
	{
		// TODO: Check if move itself can be usable (has any power left).
		let err = self.is_command_valid(party, member);
		if err != BattleError::None
		{
			return err;
		}

		let attack_command = CommandAttack
		{
			member: member,
			target_party: target_party,
			target_member: target_member,
			attack_index: attack_index,
		};

		{
			let monster_attack = attack_command.attack(party, &self);
			if monster_attack.limit_left() == 0
			{
				return BattleError::Limit;
			}

			let attack = monster_attack.attack();

			let same_party = party == attack_command.target_party;
			if (attack.target & target::SIDE_ENEMY) == 0 && !same_party
			{
				return BattleError::Target;
			}
			if (attack.target & target::SIDE_ALLY) == 0 && same_party
			{
				return BattleError::Target;
			}

			let is_adjacent = is_adjacent_with(attack_command.member, attack_command.target_member);
			if (attack.target & target::RANGE_ADJACENT) == 0 && is_adjacent
			{
				return BattleError::Target;
			}
			if (attack.target & target::RANGE_OPPOSITE) == 0 && !is_adjacent
			{
				return BattleError::Target;
			}

			let same_member = attack_command.member == attack_command.target_member;
			if (attack.target & target::TARGET_SELF) == 0 && same_party && same_member
			{
				return BattleError::Target;
			}
		}

		self.add_command_to_queue(party, member, CommandType::Attack(attack_command));
		BattleError::None
	}

	pub fn add_command_switch(&mut self, party: usize, member: usize, target: usize) -> BattleError
	{
		let err = self.is_command_valid(party, member);
		if err != BattleError::None
		{
			return err;
		}

		let switch_err = self.is_switch_valid(party, target);
		if switch_err != BattleError::None
		{
			switch_err
		}
		else
		{
			// TODO: Optimize queue switch check?
			let queued = self.queue.iter().any(|command|
			{
				if command.party == party
				{
					if let CommandType::Switch(ref switch_command_other) = command.command_type
					{
						if switch_command_other.target == target
						{
							return true;
						}
					}
				}
				false
			});
			if !queued
			{
				let switch_command = CommandSwitch
				{
					member: member,
					target: target,
				};
				self.add_command_to_queue(party, member, CommandType::Switch(switch_command));
				BattleError::None
			}
			else
			{
				BattleError::Queued
			}
		}
	}
	pub fn add_command_escape(&mut self, party: usize) -> BattleError
	{
		assert!(party < self.parties.len());

		let member_queued = self.ready[party].iter().any(|member|
		{
			member.is_some()
		});
		if member_queued
		{
			BattleError::Escape
		}
		else
		{
			for member in self.ready[party].iter_mut()
			{
				// Delete any existing commands if they exist.
				if let Some(queue_index) = *member
				{
					// TODO: NOTE: This invalidates all other indices! Fix ASAP!
					self.queue.remove(queue_index);
				}
				*member = Some(self.queue.len());
			}
			self.waiting -= self.ready[party].len();

			self.queue.push(Command::new(CommandType::Escape, party));
			BattleError::None
		}
	}
	fn add_command_to_queue(&mut self, party: usize, member: usize, command: CommandType)
	{
		if let Some(queue_index) = self.ready[party][member]
		{
			debug_assert!(self.queue[queue_index].party == party);
			self.queue[queue_index].command_type = command;
		}
		else
		{
			self.ready[party][member] = Some(self.queue.len());
			self.waiting -= 1;

			self.queue.push(Command::new(command, party));
		}
	}

	pub fn execute_switch(&mut self, member: usize) -> BattleError
	{
		let (party, active) = self.switch_queue.unwrap();
		let err = self.is_switch_valid(party, member);
		if err != BattleError::None
		{
			err
		}
		else
		{
			self.switch(party, active, member);
			self.switch_queue = None;
			BattleError::None
		}
	}

	fn switch(&mut self, party: usize, member: usize, with: usize)
	{
		self.parties[party].active[member] = with;
	}

	fn execute_command(&mut self) -> BattleExecution
	{
		let mut min_index = 0;
		for index in 1..self.queue.len()
		{
			if Command::cmp(&self.queue[index], &self.queue[min_index], &self) == Ordering::Less
			{
				min_index = index;
			}
		}
		let command = self.queue.swap_remove(min_index);
		if let CommandType::Attack(ref attack_command) = command.command_type
		{
			let target = self.parties[command.party].members.get_mut(attack_command.member).unwrap();
			target.get_attacks_mut()[attack_command.attack_index].limit_left_take(1);
		}

		let battle_command = BattleCommand::new(command, &self.parties, &mut self.rng);
		self.commands.push(battle_command);
		self.current = 0;
		BattleExecution::Command
	}

	/// Executes the next battle action.
	pub fn execute(&mut self) -> BattleExecution
	{
		if self.started
		{
			if let Some(switch_party) = self.switch_queue
			{
				BattleExecution::Switch(switch_party.0)
			}
			else if self.current != self.commands.last().unwrap().effects.len()
			{
				self.apply_effect();

				self.current += 1;
				BattleExecution::Queue
			}
			else if !self.queue.is_empty()
			{
				self.execute_command()
			}
			else
			{
				// Reset the waiting for new commands.
				self.waiting = self.total;
				self.started = false;
				for ready_party in self.ready.iter_mut()
				{
					for ready in ready_party.iter_mut()
					{
						*ready = None;
					}
				}
				BattleExecution::Waiting
			}
		}
		else
		{
			if self.waiting != 0
			{
				BattleExecution::Waiting
			}
			else
			{
				// TODO: Insert lingering effects into priority queue.

				self.started = true;
				self.execute_command()
			}
		}
	}

	/// The current executing command.
	pub fn get_current_command(&self) -> Option<&Command>
	{
		self.commands.last().map(|battle_command| &battle_command.command)
	}

	/// The current executing result of the current executing command.
	pub fn get_current_effect(&self) -> Option<&Effect>
	{
		self.commands.last().map(|battle_command| &battle_command.effects[self.current - 1])
	}

	fn apply_effect(&mut self)
	{
		let battle_command = self.commands.last().unwrap();
		match battle_command.effects[self.current]
		{
			Effect::Damage(ref effect) =>
			{
				let member = self.parties[effect.party].active[effect.member];

				let dead =
				{
					let target = self.parties[effect.party].members.get_mut(member).unwrap();
					target.lose_health(effect.amount);
					target.get_health() == 0
				};

				if dead
				{
					// In case the dead monster's command exists in queue, remove it.
					for i in 0..self.queue.len()
					{
						if self.queue[i].party == effect.party &&
							self.queue[i].affects_member(effect.member)
						{
							self.queue.remove(i);
							// TODO: Replace the attack with a miss.
							break;
						}
					}

					for i in 0..self.parties[effect.party].members.len()
					{
						let party = &self.parties[effect.party];
						if party.members[i].get_health() != 0 && !party.active.contains(&i)
						{
							self.switch_queue = Some((effect.party, member));
							return;
						}
					}

					// At this point, it doesn't matter which we remove because they're all false.
					self.ready[effect.party].pop();
					self.total -= 1;

					self.parties[effect.party].active.remove(effect.member);
				}
			}
			Effect::Switch(ref switch) =>
			{
				let ref mut p = self.parties[battle_command.command.party];
				p.members.swap(p.active[switch.member], switch.target);
				// self.switch(battle_command.command.party, battle_command.command.monster, target);
			}
			Effect::None(_) => ()
		}
	}
}
