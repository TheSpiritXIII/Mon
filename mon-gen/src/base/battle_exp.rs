use std::io;

use base::attack::Target;
use base::command::{CommandType, CommandAttack, CommandSwitch};
use base::queue::BattleQueue;
use base::party::Party;
use base::replay::BattleCommand;
use base::runner::{BattleRunner, BattleExecution};

// enum InputState
// {
// 	// Waiting for normal input.
// 	Waiting,
// 	// Processing all given inputs.
// 	Processing,
// 	// Waiting for user to switch.
// 	Switching(usize),
// 	// Waiting for user to 
// 	Ending,
// }

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

/// Battle runner that takes and validates user input.
pub struct Battle<'a>
{
	runner: BattleRunner<'a>,
	queue: BattleQueue,
	processing: bool,
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
			processing: false,
			// switch_waiting: None,
		})
	}

	/// Returns the runner for accessing battle meta-data.
	pub fn runner(&self) -> &BattleRunner
	{
		&self.runner
	}

	fn is_adjacent_with(to: usize, from: usize) -> bool
	{
		to == from || (to > 0 && to - 1 == from) || (to < usize::max_value() && to + 1 == from)
	}

	/// Adds a command for attacking another party's member.
	pub fn command_add_attack(&mut self, party: usize, active: usize, attack: usize,
		target_party: usize, target_active: usize) -> BattleError
	{
		debug_assert!(party <= self.runner.parties().len());
		debug_assert!(active <= self.runner.parties()[party].active_count());
		debug_assert!(target_party <= self.runner.parties().len());
		debug_assert!(target_active <= self.runner.parties()[target_party].active_count());

		if self.processing
		{
			return BattleError::Rejected;
		}

		let active_attack =
		{
			&self.runner.parties()[party].active_member(active).member.attacks()[attack]
		};
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
		debug_assert!(party <= self.runner.parties().len());
		debug_assert!(active <= self.runner.parties()[party].active_count());
		debug_assert!(target <= self.runner.parties()[party].member_count());

		if self.processing
		{
			BattleError::Rejected
		}
		else if self.runner.parties()[party].member(target).health() == 0
		{
			BattleError::SwitchHealth
		}
		else if self.runner.parties()[party].active_member_index(active) == target
		{
			BattleError::SwitchActive
		}
		else
		{
			for active_index in 0..self.runner.parties()[party].active_count()
			{
				if let Some(command) = self.queue.command_get(party, active_index)
				{
					if let CommandType::Switch(ref switch) = *command.command_type()
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
		debug_assert!(party <= self.runner.parties().len());

		if self.processing
		{
			BattleError::Rejected
		}
		else
		{
			self.queue.command_add_party(CommandType::Escape, party);
			BattleError::None
		}
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
		if self.processing
		{
			// Check if waiting for mandatory monster switch.
			let execution = self.execute_runner();
			if execution == BattleExecution::Ready
			{
				if self.queue.ready()
				{
					self.execute_command()
				}
				else if let BattleCommand::Turn = *self.runner.current_command()
				{
					let execution = self.execute_runner();
					if let BattleExecution::Ready = self.execute_runner()
					{
						self.processing = false;
						BattleExecution::Waiting
					}
					else
					{
						execution
					}
				}
				else
				{
					self.runner.replay_mut().command_add(BattleCommand::Turn);
					self.execute_runner()
				}
			}
			else
			{
				execution
			}
			// If runner is waiting:
			// - Check if waiting for post monster switch.
			// - wait for new inputs before inserting new commands.
		}
		else if self.queue.ready()
		{
			let execution = self.execute_command();
			self.processing = true;
			execution
		}
		else
		{
			BattleExecution::Waiting
		}
	}

	fn execute_runner(&mut self) -> BattleExecution
	{
		let execution = self.runner.run();
		if let BattleExecution::Death(ref party_member) = execution
		{
			self.queue.command_remove(party_member.party, party_member.member);
		}
		execution
	}

	fn execute_command(&mut self) -> BattleExecution
	{
		let command = self.queue.command_consume(self.runner.parties());
		self.runner.replay_mut().command_add(BattleCommand::Action(command));
		self.execute_runner()
	}

	// fn execute_post_switch(&mut self, active: usize, target: usize)
	// {

	// }
}
