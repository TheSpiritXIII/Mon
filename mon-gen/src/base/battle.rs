#![allow(unknown_lints)]
use std::cmp::Ordering;

use rand::{Rng, StdRng};

use base::monster::Monster;
use base::attack::Target;
use base::party::{Party, PartyMember};
pub use base::command::{Command, CommandType, CommandAttack, CommandSwitch};
pub use base::effect::{Effect, NoneReason};
pub use base::types::battle::StatModifierType;
pub use base::statmod::StatModifiers;

use calculate::damage::{MemberIndex, calculate_experience};
use base::effect::ExperienceGain;

use std::collections::HashMap;

// TODO: This class is redundant. Break it up
#[derive(Debug)]
struct BattleCommand
{
	command: Command,
	effects: Vec<Effect>,
}

impl BattleCommand
{
	fn with_miss(command: Command) -> Self
	{
		let mut effects = Vec::new();
		effects.push(Effect::None(NoneReason::Miss));
		BattleCommand
		{
			effects: effects,
			command: command,
		}
	}
	fn new<'a, R: Rng>(command: Command, parties: &[Party<'a>], rng: &mut R) -> Self
	{
		let mut effects = Vec::new();
		command.command_type.effects(parties, &command, rng, &mut effects);
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

	party_switch_waiting: usize,

	// Maps the number of sides
	sides_alive: HashMap<u8, usize>,

	// switch_waiting: usize,

	// TODO: Vec for lingering effects. Store index to playback and also turn number.
	// Lingering effect will apply at end of each turn.
}

pub enum BattleExecution
{
	Command,
	Queue,
	Waiting,
	Switch(usize),
	SwitchWaiting,
	/// Occurs when the battle is over. Further commands cannot be added or processed.
	Finished(u8),
}

/// Indicates an error adding a command to a battle.
#[derive(Debug, PartialEq)]
pub enum BattleError
{
	/// There was no error.
	None,
	/// Occurs when the battle turn is in progress. New commands cannot be added.
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
		let mut sides = HashMap::new();
		let mut total = 0;
		let mut ready = Vec::with_capacity(parties.len());
		for group in &parties
		{
			total += group.active_count();
			ready.push(vec![None; group.active_count()]);
			let side_count = sides.entry(group.side()).or_insert(0);
			*side_count += 1;
		}
		let mut b = Battle
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
			party_switch_waiting: 0,
			sides_alive: sides,
		};
		for i in 0..b.parties.len()
		{
			Battle::expose_party(&mut b.parties, i);
		}
		b
	}
	pub fn party(&self, index: usize) -> &Party<'a>
	{
		&self.parties[index]
	}
	pub fn monster(&self, party: usize, monster: usize) -> &Monster
	{
		self.parties[party].member(monster)
	}
	pub fn monster_active(&self, party: usize, monster: usize) -> PartyMember
	{
		self.parties[party].active_member(monster)
	}
	pub fn monster_active_alive(&self, party: usize, monster: usize) -> Option<PartyMember>
	{
		self.parties[party].active_member_alive(monster)
	}
	pub fn monster_is_active(&self, party: usize, monster: usize) -> bool
	{
		self.parties[party].member_is_active(monster)
	}
	pub fn monster_active_count(&self, party: usize) -> usize
	{
		self.parties[party].active_count()
	}
	fn is_member_valid(&self, party: usize, member: usize) -> BattleError
	{
		assert!(party < self.parties.len());
		assert!(member < self.parties[party].member_count());
		BattleError::None
	}
	fn is_switch_valid(&self, party: usize, member: usize) -> BattleError
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
	fn is_command_valid(&self, party: usize, member: usize) -> BattleError
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
			let monster_attack = attack_command.attack(party, self);
			if monster_attack.limit_left() == 0
			{
				return BattleError::Limit;
			}

			let attack = monster_attack.attack();

			let same_party = party == attack_command.target_party;
			if (attack.target & Target::SIDE_ENEMY) == 0 && !same_party
			{
				return BattleError::Target;
			}
			if (attack.target & Target::SIDE_ALLY) == 0 && same_party
			{
				return BattleError::Target;
			}

			let is_adjacent = is_adjacent_with(attack_command.member, attack_command.target_member);
			if (attack.target & Target::RANGE_ADJACENT) == 0 && is_adjacent
			{
				return BattleError::Target;
			}
			if (attack.target & Target::RANGE_OPPOSITE) == 0 && !is_adjacent
			{
				return BattleError::Target;
			}

			let same_member = attack_command.member == attack_command.target_member;
			if (attack.target & Target::TARGET_SELF) == 0 && same_party && same_member
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
				if command.party() == party
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
			for member in &mut self.ready[party]
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
			debug_assert!(self.queue[queue_index].party() == party);
			self.queue[queue_index].command_type = command;
		}
		else
		{
			self.ready[party][member] = Some(self.queue.len());
			self.waiting -= 1;

			self.queue.push(Command::new(command, party));
		}
	}

	pub fn execute_post_switch(&mut self, party: usize, member: usize, target: usize) -> BattleError
	{
		let err = self.is_switch_valid(party, target);
		if err != BattleError::None
		{
			err
		}
		else
		{
			self.switch(party, member, target);
			if self.parties[party].active_waiting()
			{
				self.party_switch_waiting -= 1;
			}
			// self.parties[party].active_set(member, target);
			// self.switch_waiting -= 1;
			BattleError::None
		}
	}

	pub fn is_party_post_switch_waiting(&self, party: usize) -> Option<usize>
	{
		self.parties[party].switch_waiting()
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
		let p = &mut self.parties[party];
		p.switch_active(member, with);
	}

	fn execute_command(&mut self) -> BattleExecution
	{
		let mut min_index = 0;
		for index in 1..self.queue.len()
		{
			if Command::cmp(&self.queue[index], &self.queue[min_index], self) == Ordering::Less
			{
				min_index = index;
			}
		}
		let command = self.queue.swap_remove(min_index);

		let hit = if let CommandType::Attack(ref attack_command) = command.command_type
		{
			let hit =
			{
				self.parties[attack_command.target_party].active_member_alive(attack_command.target_member).is_some()
			};

			let party = &mut self.parties[command.party()];
			party.active_member_attack_limit_take(attack_command.member,
				attack_command.attack_index);

			hit
		}
		else
		{
			true
		};

		self.commands.push(if hit
		{
			BattleCommand::new(command, &self.parties, &mut self.rng)
		}
		else
		{
			BattleCommand::with_miss(command)
		});
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
			else if self.sides_alive.len() == 1
			{
				BattleExecution::Finished(*self.sides_alive.keys().next().unwrap())
			}
			else if !self.queue.is_empty()
			{
				self.execute_command()
			}
			else if self.party_switch_waiting != 0// TODO.
			// else if self.switch_waiting != 0
			{
				BattleExecution::SwitchWaiting
			}
			else
			{
				// let mut side_alive_count = 0;
				// let mut side_alive = 0;
				for x in 0..self.parties.len()
				{
					
					// TODO: Make work again!
					let party = self.parties.get_mut(x).unwrap();
					party.active_purge();
					// let mut i = 0;
					// while i != party.active_count()
					// {
					// 	if party.active_member(i).is_none()
					// 	{
					// 		party.active_remove(i);
					// 	}
					// 	else
					// 	{
					// 		i += 1;
					// 	}
					// }
					// if party.active_count() != 0
					// {
					// 	side_alive_count += 1;
					// 	side_alive = party.side();
					// }
				}
				// TODO: Detect loss earlier. This will prevent cases when tie.
				// if side_alive_count <= 1
				// {
				// 	return BattleExecution::Finished(side_alive);
				// }

				// Reset the waiting for new commands.
				self.waiting = self.total;
				self.started = false;
				for ready_party in &mut self.ready
				{
					for ready in ready_party.iter_mut()
					{
						*ready = None;
					}
				}
				BattleExecution::Waiting
			}
		}
		else if self.waiting != 0
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

	fn affects_member(command: &Command, member: usize) -> bool
	{
		match command.command_type
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

	fn apply_effect(&mut self)
	{
		let mut damage_kill = false;
		let mut damage_party = 0;
		let mut damage_active = 0;

		match self.commands.last().unwrap().effects[self.current]
		{
			Effect::Damage(ref effect) =>
			{
				damage_party = effect.party;
				damage_active = effect.active;

				let member = effect.active;

				let dead = self.parties[effect.party()].active_member_lose_health(member, effect.amount());

				if dead
				{
					damage_kill = true;

					for i in 0..self.queue.len()
					{
						if self.queue[i].party() == effect.party() &&
							Battle::affects_member(&self.queue[i], effect.active)
						{
							self.queue.swap_remove(i);
							break;
						}
					}

					if !self.parties[effect.party()].active_waiting()
					{
						if !self.parties[effect.party()].active_are_alive()
						{
							let side = self.parties[effect.party()].side();
							let left = *self.sides_alive.get(&side).unwrap();
							if left == 1
							{
								self.sides_alive.remove(&side);
							}
							else
							{
								*self.sides_alive.get_mut(&side).unwrap() -= 1;
							}
						}

						// At this point, it doesn't matter which we remove because they're all false.
						self.ready[effect.party()].pop();
						self.total -= 1;
					}
					else
					{
						self.party_switch_waiting += 1;
					}
				}
			}
			Effect::Switch(ref switch) =>
			{
				let party_index = self.commands.last().unwrap().command.party();
				{
					self.parties[party_index].switch_active(switch.member, switch.target);
				}
				
				Battle::expose_party(&mut self.parties, party_index);
			}
			Effect::Modifier(ref modifiers) =>
			{
				let party = self.parties.get_mut(modifiers.party()).unwrap();
				party.active_member_modifiers_add(modifiers.active(), modifiers.modifiers());
			}
			Effect::ExperienceGain(ref experience_gain) =>
			{
				// TODO: See TODO about sides_alive.len() == 1 check in this function.
				// We also want it to ignore experience effects.

				self.parties[experience_gain.party].member_experience_add(experience_gain.member,
					experience_gain.amount);
				return;
			}
			Effect::None(_) => ()
		}


		let mut offense_party = 0;
		let offense =
		{
			if let CommandType::Attack(ref attack_command) = self.commands.last().unwrap().command.command_type
			{
				offense_party = self.commands.last().unwrap().command.party();
				Some(MemberIndex
				{
					party: offense_party,
					member: attack_command.member,
				})
			}
			else
			{
				None
			}
		};

		// TODO: I don't like how this branch is done after every execution.
		// It only needs to be done after damage. Done like this to avoid borrow error.
		if damage_kill && self.parties[offense_party].gain_experience()
		{
			if self.sides_alive.len() == 1
			{
				self.commands.last_mut().unwrap().effects.split_off(self.current + 1);
			}

			let defense = MemberIndex
			{
				party: damage_party,
				member: damage_active,
			};


			let experience_map = calculate_experience(&self.parties, offense, defense);

			// TODO: Add item/ability modification here.

			for experience_party in &experience_map
			{
				let party = experience_party.0;
				for experience_member in experience_party.1.iter()
				{
					let member = *experience_member.0;
					let amount = *experience_member.1;
					let level = self.parties[*party].member(member).get_level();
					let gain = ExperienceGain::new(*party, member, amount, level);
					self.commands.last_mut().unwrap().effects.insert(self.current + 1,
						Effect::ExperienceGain(gain));
				}
			}
		}
	}
	fn expose_party(parties: &mut Vec<Party<'a>>, party_index: usize)
	{
		// Allow clippy lint to be ignored.
		// Clippy is wrong in this case because the index is used to prevent mutable borrow.
		#![allow(needless_range_loop)]
		let switch_side = parties[party_index].side();
		for index in 0..parties.len()
		{
			if parties[index].side() != switch_side
			{
				for active_index in 0..parties[party_index].active_count()
				{
					let expose_reference = parties[party_index].expose_reference(active_index);
					parties[index].expose_add_member(party_index, expose_reference);
				}
			}
		}
	}
}
