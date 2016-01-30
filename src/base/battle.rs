use super::monster::Monster;
use std::collections::{VecDeque, BinaryHeap};
use super::types::{MoveId, StatType};

struct Trainer
{
	party: Vec<Monster>,
}

struct Battle
{
	trainers: Vec<Trainer>,
	current: Option<Command>,
	results: VecDeque<CommandResult>,
	commands: BinaryHeap<Command>
}

enum BattleExecution
{
	Empty,
	Command,
	Queue,
}

enum Action
{
	Attack { attack: MoveId, target_trainer: usize, target_monster: usize},
	//Item(ItemId),
	//Switch(ToId),
	Escape,
}

struct Command
{
	action: Action,
	from: usize,
	trainer: usize,
}

enum ActionResult
{
	Damage { amount: StatType },
	//Status(StatusId),
	//Bonus(BonusType),
	//Ability(AbilityId),
}

struct CommandResult
{
	action: ActionResult,
	from: usize,
	trainer: usize,
}

impl Battle
{
	/// True if the command is a valid command.
	pub fn add_command(&self, command: Command) -> bool
	{
		self.commands.push(command);
		true
	}
	
	/// Executes the next command action result.
	fn execute_command(&self) -> BattleExecution
	{
		match self.commands.peek()
		{
			Some(command) =>
			{
				// TODO: Push some results as self.results.push_back();
				BattleExecution::Command
			},
			None =>
			{
				self.current = None;
				BattleExecution::Empty
			},
		}
	}
	
	/// Executes the next battle action.
	pub fn execute(&self) -> BattleExecution
	{
		match self.current
		{
			Some(command) =>
			{
				match self.results.pop_front()
				{
					Some(_) =>
					{
						BattleExecution::Queue
					},
					None =>
					{
						self.execute_command()
					}
				}
			}
			None =>
			{
				self.execute_command()
			}
		}
	}
	
	/// The current executing command.
	pub fn get_command(&self) -> Option<Command>
	{
		self.current
	}
	
	/// The current executing result of the current executing command.
	pub fn get_result(&self) -> Option<&CommandResult>
	{
		self.results.front()
	}
}
