use std::collections::HashMap;
use std::io;

use base::attack::Target;
use base::command::{CommandType, CommandAttack, CommandSwitch, CommandEscape, CommandRetreat};
use base::effect::Effect;
use base::queue::BattleQueue;
use base::party::Party;
use base::runner::{BattleRunner, BattleExecution, BattlePartyMember, BattleState};

/// Indicates an error adding a command to a battle.
#[derive(Debug, PartialEq)]
pub enum BattleError
{
	/// There was no error.
	None,
	/// Occurs when the battle turn is in progress. New commands cannot be added.
	Rejected,
	/// Occurs when the chosen attack is unable to be used due to having reached the use limit.
	AttackLimit,
	/// Occurs when the chosen attack is unable to target the chosen party and respective member.
	AttackTarget,
	/// Occurs when a switch cannot occur because the target is the same as what is being switched.
	SwitchActive,
	/// Occurs when a switch cannot occur because the target has no health.
	SwitchHealth,
	/// Occurs when a switch cannot occur because the target has already been queued to switch.
	SwitchQueued,
}

#[derive(Debug, PartialEq, Eq)]
enum BattleInputState
{
	Ready,
	Processing,
	Switching,
	Retreat(BattlePartyMember),
}

/// Battle runner that takes and validates user input.
pub struct Battle<'a>
{
	runner: BattleRunner<'a>,
	queue: BattleQueue,
	processing: BattleInputState,
	post_switch: HashMap<usize, usize>,
	// switch_waiting: Option<usize>
}

impl<'a> Battle<'a>
{
	/// Generates a new battle object with a randomly generated RNG and empty command history.
	pub fn new(parties: Vec<Party<'a>>) -> Result<Self, io::Error>
	{
		let queue = BattleQueue::new(&parties);
		Ok(Battle
		{
			runner: BattleRunner::new(parties)?,
			queue: queue,
			processing: BattleInputState::Ready,
			post_switch: HashMap::new(),
			// switch_waiting: None,
		})
	}

	pub fn state(&self) -> &BattleState
	{
		self.runner.state()
	}

	pub fn current_command(&self) -> &CommandType
	{
		self.runner.current_command()
	}

	pub fn current_effect(&self) -> &Effect
	{
		self.runner.current_effect()
	}

	fn is_adjacent_with(to: usize, from: usize) -> bool
	{
		to == from || (to > 0 && to - 1 == from) || (to < usize::max_value() && to + 1 == from)
	}

	/// Adds a command for attacking another party's member.
	pub fn command_add_attack(&mut self, party: usize, active: usize, attack: usize,
		target_party: usize, target_active: usize) -> BattleError
	{
		debug_assert!(party <= self.state().parties().len());
		debug_assert!(active <= self.state().parties()[party].active_count());
		debug_assert!(target_party <= self.state().parties().len());
		debug_assert!(target_active <= self.state().parties()[target_party].active_count());

		if self.processing != BattleInputState::Ready
		{
			return BattleError::Rejected;
		}

		let active_attack = &self.runner.state().parties()[party].active_member(active).member.attacks()[attack];
		if active_attack.limit_left() == 0
		{
			return BattleError::AttackLimit;
		}

		let same_party = party == target_party;
		if (active_attack.attack().target & Target::SIDE_ENEMY) == 0 && !same_party
		{
			return BattleError::AttackTarget;
		}
		if (active_attack.attack().target & Target::SIDE_ALLY) == 0 && same_party
		{
			return BattleError::AttackTarget;
		}

		let is_adjacent = Battle::is_adjacent_with(active, target_active);
		if (active_attack.attack().target & Target::RANGE_ADJACENT) == 0 && is_adjacent
		{
			return BattleError::AttackTarget;
		}
		if (active_attack.attack().target & Target::RANGE_OPPOSITE) == 0 && !is_adjacent
		{
			return BattleError::AttackTarget;
		}

		let same_member = active == target_active;
		if (active_attack.attack().target & Target::TARGET_SELF) == 0 && same_party && same_member
		{
			return BattleError::AttackTarget;
		}

		let command_attack = CommandAttack
		{
			party: party,
			member: active,
			target_party: target_party,
			target_member: target_active,
			attack_index: attack,
		};

		self.queue.command_add(CommandType::Attack(command_attack), party, active);

		BattleError::None
	}

	/// Adds a command for switching an active party member with another party member.
	///
	/// The requested `target` party member must be referenced by member index, even if it is an
	/// active member.
	///
	pub fn command_add_switch(&mut self, party: usize, active: usize, target: usize) -> BattleError
	{
		debug_assert!(party <= self.state().parties().len());
		debug_assert!(active <= self.state().parties()[party].active_count());
		debug_assert!(target <= self.state().parties()[party].member_count());

		if self.processing != BattleInputState::Ready
		{
			BattleError::Rejected
		}
		else if self.state().parties()[party].member(target).health() == 0
		{
			BattleError::SwitchHealth
		}
		else if self.state().parties()[party].active_member_index(active) == target
		{
			BattleError::SwitchActive
		}
		else
		{
			for active_index in 0..self.state().parties()[party].active_count()
			{
				if let Some(command) = self.queue.command_get(party, active_index)
				{
					if let CommandType::Switch(ref switch) = *command
					{
						if switch.target == target && switch.member != active
						{
							return BattleError::SwitchQueued;
						}
					}
				}
			}

			let command_switch = CommandSwitch
			{
				party: party,
				member: active,
				target: target,
			};
			self.queue.command_add(CommandType::Switch(command_switch), party, active);
			BattleError::None
		}
	}

	/// Adds a party central command for escaping which prematurely ends the battle.
	///
	/// This command will remove any commands attached to individual party members.
	///
	pub fn command_add_escape(&mut self, party: usize) -> BattleError
	{
		debug_assert!(party <= self.state().parties().len());

		if self.processing != BattleInputState::Ready
		{
			BattleError::Rejected
		}
		else
		{
			self.queue.command_add_party(CommandType::Escape(CommandEscape
			{
				party: party,
			}), party);
			BattleError::None
		}
	}

	pub fn command_add_post_switch(&mut self, party: usize, active: usize, target: usize) -> BattleError
	{
		debug_assert!(party <= self.state().parties().len());
		debug_assert!(active <= self.state().parties()[party].active_count());
		debug_assert!(target <= self.state().parties()[party].member_count());

		if self.processing != BattleInputState::Switching
		{
			BattleError::Rejected
		}
		else if self.state().parties()[party].active_member_index(active) == target
		{
			BattleError::SwitchActive
		}
		else
		{
			if self.state().parties()[party].active_member(active).member.health() == 0 &&
				self.state().parties()[party].member(target).health() != 0 &&
				!self.state().parties()[party].member_is_active(target)
			{
				let remove =
				{
					let count = self.post_switch.get_mut(&party).unwrap();
					*count -= 1;
					*count == 0
				};
				
				if remove
				{
					self.post_switch.remove(&party);
				}
			}

			let command_switch = CommandType::Switch(CommandSwitch
			{
				party: party,
				member: active,
				target: target,
			});
			let command = command_switch;//Command::new(command_switch, party);
			self.runner.command_add(command);
			BattleError::None
		}
	}

	pub fn command_add_retreat(&mut self, target: usize) -> BattleError
	{
		if let BattleInputState::Retreat(ref retreat) = self.processing
		{
			debug_assert!(target <= self.state().parties()[retreat.party].member_count());

			if self.state().parties()[retreat.party].active_member_index(retreat.member) == target
			{
				return BattleError::SwitchActive;
			}
			else
			{
				let command_retreat = CommandRetreat
				{
					target: target,
				};
				self.runner.sub_command_add(Some(command_retreat));
			}
		}
		else
		{
			return BattleError::Rejected;
		}
		self.processing = BattleInputState::Processing;
		BattleError::None
	}

	/// Executes the next consecutive command effect. Returns the result of the command.
	///
	/// Execution goes as follows:
	/// - Confirm that the queue is still ready to be consumed.
	/// - Check if waiting for the user to switch party members.
	/// - Run the battle runner.
	/// - Allow the queue to be mutated.
	///
	pub fn execute(&mut self) -> BattleExecution
	{
		match self.processing
		{
			BattleInputState::Ready =>
			{
				println!("Queue ready? {:?}", self.queue);
				if self.queue.ready()
				{
					let execution = self.execute_command();
					self.processing = BattleInputState::Processing;
					execution
				}
				else
				{
					BattleExecution::Waiting
				}
			}
			BattleInputState::Processing =>
			{
				let execution = self.execute_runner();
				if execution == BattleExecution::Ready
				{
					if self.queue.ready()
					{
						self.execute_command()
					}
					else if let CommandType::Turn = *self.runner.current_command()
					{
						let execution = self.execute_runner();
						if let BattleExecution::Ready = self.execute_runner()
						{
							self.execute_switch()
						}
						else
						{
							execution
						}
					}
					else
					{
						self.runner.command_add(CommandType::Turn);
						self.execute_runner()
					}
				}
				else
				{
					execution
				}
			}
			BattleInputState::Switching =>
			{
				self.execute_switch()
			}
			BattleInputState::Retreat(ref party_member) =>
			{
				BattleExecution::RetreatWaiting(BattlePartyMember
				{
					party: party_member.party,
					member: party_member.member,
				})
			}
		}
	}

	fn execute_runner(&mut self) -> BattleExecution
	{
		let execution = self.runner.run();
		if let BattleExecution::Death(ref party_member) = execution
		{
			if self.state().parties()[party_member.party].active_waiting()
			{
				*self.post_switch.entry(party_member.party).or_insert(0) += 1;
			}
			else
			{
				self.queue.member_remove(party_member.party, party_member.member);
			}
			self.queue.command_remove(party_member.party, party_member.member);
		}
		else if let BattleExecution::RetreatWaiting(ref party_member) = execution
		{
			self.processing = BattleInputState::Retreat(BattlePartyMember
			{
				party: party_member.party,
				member: party_member.member,
			});

		}
		execution
	}

	fn execute_command(&mut self) -> BattleExecution
	{
		let command = self.queue.command_consume(self.runner.state().parties(), self.runner.state().flags());
		self.runner.command_add(command);
		self.execute_runner()
	}

	fn execute_switch(&mut self) -> BattleExecution
	{
		let execution = self.execute_runner();
		if let BattleExecution::Ready = execution
		{
			if !self.post_switch.is_empty()
			{
				self.processing = BattleInputState::Switching;
				BattleExecution::SwitchWaiting
			}
			else
			{
				self.processing = BattleInputState::Ready;
				BattleExecution::Waiting
			}
		}
		else
		{
			execution
		}
	}
}
