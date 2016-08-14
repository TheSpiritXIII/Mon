use std::io;

use rand::{StdRng, SeedableRng};

use base::replay::BattleReplay;
use base::queue::BattleQueue;
use base::party::Party;

pub enum BattleExecutionNew
{
	/// A new command has been executed. An effect for this command has not been executed yet.
	Command,
	/// A new effect has been executed. A command for this effect is active.
	Effect,
	/// No possible execution is possible. The entire command list has been consumed.
	Ready,
	/// Waiting for user input.
	Waiting,
	// Switch(usize),
	// SwitchWaiting,
	// /// Occurs when the battle is over. Further commands cannot be added or processed.
	// Finished(u8),
}

pub struct BattleRunner
{
	replay: BattleReplay,
	rng: StdRng,
	command: usize,
	effect: usize,
	turn: usize,
}

impl BattleRunner
{
	/// Generates a new battle object with a randomly generated RNG and empty command history.
	pub fn new() -> Result<Self, io::Error>
	{
		let replay = BattleReplay::new()?;
		let rng = StdRng::from_seed(&[replay.seed()]);
		Ok(BattleRunner
		{
			replay: replay,
			rng: rng,
			command: 0,
			effect: 0,
			turn: 0,
		})
	}

	// pub fn from_replay(replay: BattleReplay) -> Self
	// {
	// 	let rng = StdRng::from_seed(&[replay.seed()]);
	// 	BattleRunner
	// 	{
	// 		replay: replay,
	// 		rng: rng,
	// 		command: 0,
	// 		effect: 0,
	// 		turn: 0,
	// 	}
	// }

	/// Executes the next consecutive command effect. Returns the result of the command.
	///
	/// Execution goes as follows:
	/// - For the current command, all effects are applied.
	/// - Checks are done to see if there is a winner yet.
	/// - The next command is lined up for execution.
	///
	pub fn run(&mut self) -> BattleExecutionNew
	{
		if self.effect < self.replay.effect_count(self.command)
		{
			// apply

			self.effect += 1;
			BattleExecutionNew::Effect
		}
		// else 
		// {
		// 	// Check battle has winner/
		// }
		else if self.command != self.replay.command_count()
		{
			self.command += 1;
			BattleExecutionNew::Command
		}
		else
		{
			BattleExecutionNew::Ready
		}
		// Check battle finished with winner.
		// Increment command counter.
	}
}

// Turn this into just 'Battle' once stable.
/// Battle that takes input.
pub struct BattleNew<'a>
{
	runner: BattleRunner,
	queue: BattleQueue,
	parties: Vec<Party<'a>>,
}

impl<'a> BattleNew<'a>
{
	/// Generates a new battle object with a randomly generated RNG and empty command history.
	pub fn new(parties: Vec<Party<'a>>) -> Result<Self, io::Error>
	{
		Ok(BattleNew
		{
			runner: BattleRunner::new()?,
			queue: BattleQueue::new(&parties),
			parties: Vec::new(),
		})
	}

	/// Executes the next consecutive command effect. Returns the result of the command.
	///
	/// Execution goes as follows:
	/// - Confirm that the queue is still ready to be consumed.
	/// - Check if waiting for the user to switch party members.
	/// - Run the battle runner.
	/// - Allow the queue to be mutated.
	///
	pub fn run(&mut self) -> BattleExecutionNew
	{
		if self.queue.ready()
		{
			// Check if waiting for mandatory monster switch.
			self.runner.run()
			// If runner is waiting:
			// - Check if waiting for post monster switch.
			// - wait for new inputs before inserting new commands.
		}
		else
		{
			BattleExecutionNew::Waiting
		}
	}
}