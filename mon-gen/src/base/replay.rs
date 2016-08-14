use std::io;

use rand::Rng;
use rand::os::OsRng;

pub use base::command::Command;
pub use base::effect::Effect;

#[derive(Debug)]
struct BattleCommandNew
{
	command: Command,
	effects: Vec<Effect>,
}

/// Stores meta-data required to deterministically replay a battle sequence.
pub struct BattleReplay
{
	commands: Vec<BattleCommandNew>,

	// Copy of original party.

	seed: usize,
}

impl BattleReplay
{
	pub fn new() -> Result<Self, io::Error>
	{
		Ok(BattleReplay
		{
			seed: OsRng::new()?.gen(),
			commands: Vec::new(),
		})
	}
	pub fn seed(&self) -> usize
	{
		self.seed
	}
	pub fn command(&self, index: usize) -> &Command
	{
		&self.commands[index].command
	}
	pub fn command_count(&self) -> usize
	{
		self.commands.len()
	}
	pub fn effect_count(&self, command: usize) -> usize
	{
		self.commands[command].effects.len()
	}
	pub fn command_add(&mut self, command: Command, effects: Vec<Effect>)
	{
		self.commands.push(BattleCommandNew
		{
			command: command,
			effects: effects,
		});
	}
}
