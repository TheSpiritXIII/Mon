use std::io;

use rand::Rng;
use rand::os::OsRng;

pub use base::command::Command;
pub use base::effect::Effect;

#[derive(Debug)]
enum BattleCommand
{
	Command(Command),
	Turn,
}

#[derive(Debug)]
struct BattleEffect
{
	command: BattleCommand,
	effects: Vec<Effect>,
}

/// Stores meta-data required to deterministically replay a battle sequence.
pub struct BattleReplay
{
	effects: Vec<BattleEffect>,

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
			effects: Vec::new(),
		})
	}
	pub fn seed(&self) -> usize
	{
		self.seed
	}
	// pub fn command(&self, index: usize) -> &BattleCommand
	// {
	// 	&self.effects[index].command
	// }
	pub fn command_count(&self) -> usize
	{
		self.effects.len()
	}
	pub fn effect_count(&self, command: usize) -> usize
	{
		self.effects[command].effects.len()
	}
	pub fn command_add(&mut self, command: Command, effects: Vec<Effect>)
	{
		self.effects.push(BattleEffect
		{
			command: BattleCommand::Command(command),
			effects: effects,
		});
	}
}
