use std::io;

use rand::Rng;
use rand::os::OsRng;

pub use base::command::CommandType;
pub use base::effect::Effect;
use base::command::CommandRetreat;

struct BattleCommandInstance
{
	command: CommandType,
	sub_command: Vec<Option<CommandRetreat>>,
}

/// Stores meta-data required to deterministically replay a battle sequence.
pub struct BattleReplay
{
	effects: Vec<BattleCommandInstance>,

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
	pub fn command(&self, command: usize) -> &CommandType
	{
		&self.effects[command].command
	}
	pub fn command_count(&self) -> usize
	{
		self.effects.len()
	}
	pub fn sub_command(&self, command: usize, sub_command: usize) -> &Option<CommandRetreat>
	{
		&self.effects[command].sub_command[sub_command]
	}
	pub fn sub_command_count(&self, command: usize) -> usize
	{
		self.effects[command].sub_command.len()
	}
	pub fn command_add(&mut self, command: CommandType)
	{
		self.effects.push(BattleCommandInstance
		{
			command: command,
			sub_command: Vec::new(),
		});
	}
	pub fn sub_command_add(&mut self, command: usize, sub_command: Option<CommandRetreat>)
	{
		self.effects[command].sub_command.push(sub_command);
	}
}
