use std::io;

use rand::Rng;
use rand::os::OsRng;

pub use base::command::Command;
pub use base::effect::Effect;

#[derive(Debug)]
pub enum BattleCommand
{
	Action(Command),
	Turn,
}

/// Stores meta-data required to deterministically replay a battle sequence.
pub struct BattleReplay
{
	effects: Vec<BattleCommand>,

	// Copy of original party.

	seed: usize,
}

impl BattleReplay
{
	pub fn new() -> Result<Self, io::Error>
	{
		let replay = BattleReplay
		{
			seed: OsRng::new()?.gen(),
			// At minimum, a battle only has two command-effect pairs.
			// Two sides versing, battle ends when one escapes.
			effects: Vec::with_capacity(3),
		};
		// replay.command_add(BattleCommand::Turn, Vec::new());
		Ok(replay)
	}
	pub fn seed(&self) -> usize
	{
		self.seed
	}
	pub fn command(&self, command: usize) -> &BattleCommand
	{
		&self.effects[command]
	}
	pub fn command_count(&self) -> usize
	{
		self.effects.len()
	}
	// pub fn effect(&self, command: usize, index: usize) -> &Effect
	// {
	// 	&self.effects[command].effects[index]
	// }
	// pub fn effect_count(&self, command: usize) -> usize
	// {
	// 	self.effects[command].effects.len()
	// }
	pub fn command_add(&mut self, command: BattleCommand)
	{
		self.effects.push(command);
	}
}
