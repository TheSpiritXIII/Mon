use std::cmp::Ordering;

use base::command::CommandType;
use base::party::Party;
use base::runner::BattleFlagsType;

#[derive(Debug)]
struct PartyCommand
{
	party: bool,
	commands: Vec<Option<CommandType>>,
	ready: usize,
	total: usize,
}

impl PartyCommand
{
	fn new(members: usize) -> Self
	{
		let mut commands = Vec::with_capacity(members);
		for _ in 0..members
		{
			commands.push(None);
		}
		PartyCommand
		{
			party: false,
			commands: commands,
			ready: 0,
			total: members,
		}
	}
	fn command_count(&self) -> usize
	{
		self.commands.len()
	}
	fn command_get(&self, index: usize) -> Option<&CommandType>
	{
		self.commands[index].as_ref()
	}
	fn command_take(&mut self, index: usize) -> CommandType
	{
		debug_assert!(self.commands[index].is_some());
		self.ready -= 1;
		self.commands[index].take().unwrap()
	}
	fn command_add(&mut self, command: CommandType, member: usize) -> isize
	{
		let mut change = 0;
		if self.party
		{
			// All members are now waiting for their individual commands.
			change = - (self.total as isize) + 1;
			self.party = false;
			for i in 0..self.total
			{
				self.commands[i] = None;
			}
			self.ready = 1;
		}
		else if !self.commands[member].is_some()
		{
			change = 1;
			self.ready += 1;
		}
		self.commands[member] = Some(command);
		change
	}
	fn command_add_party(&mut self, command: CommandType) -> usize
	{
		let change = self.total - self.ready;
		if self.party
		{
			for i in 1..self.total
			{
				self.commands[i] = None;
			}
		}
		self.commands[0] = Some(command);
		self.ready = self.total;
		self.party = true;
		change
	}
	fn command_remove(&mut self, member: usize) -> usize
	{
		if let Some(_) = self.commands[member]
		{
			self.commands[member] = None;
			self.ready -= 1;
			1
		}
		else
		{
			0
		}
	}
	fn member_remove(&mut self)
	{
		self.total -= 1;
	}
}

/// Manages a list of upcoming battle commands.
///
/// By default, the queue is considered not ready. At this state, new commands can be added. Once
/// all parties have a command for each of their members, the queue is considered ready. At that
/// point, commands are sorted and consumed. Once all commands are consumed, the queue goes back to
/// its default state where it is not ready and no commands have been associated with any parties.
///
#[derive(Debug)]
pub struct BattleQueue
{
	waiting: usize,
	total: usize,
	queue: Vec<PartyCommand>,
}

impl BattleQueue
{
	/// Initializes a new empty queue that is not ready.
	pub fn new(parties: &[Party]) -> Self
	{
		let mut queue = Vec::with_capacity(parties.len());
		let mut total = 0;
		for party in parties
		{
			total += party.active_count();
			queue.push(PartyCommand::new(party.active_count()));
		}
		BattleQueue
		{
			waiting: total,
			total: total,
			queue: queue,
		}
	}

	/// Returns true if the queue was populated or is in the process of being consumed.
	pub fn ready(&self) -> bool
	{
		self.waiting == 0
	}

	/// Returns the command for the indicated party member.
	pub fn command_get(&self, party: usize, member: usize) -> Option<&CommandType>
	{
		self.queue[party].command_get(member)
	}

	/// Adds the given command to the queue for the indicated members of the given party.
	///
	/// This will override any commands already given to this party member. If the given party
	/// already has an attached command, then all members of that party will be invalidated.
	///
	pub fn command_add(&mut self, command: CommandType, party: usize, member: usize)
	{
		let change = self.queue[party].command_add(command, member);
		self.waiting = (self.waiting as isize - change) as usize;
	}

	/// Adds the given command to the queue for all party members of the given party.
	///
	/// This will override any commands for the given party.
	///
	pub fn command_add_party(&mut self, command: CommandType, party: usize)
	{
		self.waiting -= self.queue[party].command_add_party(command);
	}

	/// Removes any command requested by the indicated member of the given party.
	///
	/// This command will not remove any other commands that reference this member.
	///
	pub fn command_remove(&mut self, party: usize, member: usize)
	{
		self.queue[party].command_remove(member);
	}

	/// Finds the highest priority command in the queue and pops it.
	///
	/// The queue must be ready before calling this method.
	///
	pub fn command_consume(&mut self, parties: &[Party], flags: BattleFlagsType) -> CommandType
	{
		let mut finished = true;
		let mut priority = 0;
		let mut priority_index = 0;

		// Find a party to start from.
		'outer: for party_index in 0..self.queue.len()
		{
			if self.queue[party_index].command_count() > 0
			{
				for command_index in 0..self.queue[party_index].command_count()
				{
					if let Some(_) = self.queue[party_index].command_get(command_index)
					{
						priority = party_index;
						priority_index = command_index;
						break 'outer;
					}
				}
			}
		}

		// Find the minimum uses the starting point as a base.
		for party_index in priority..self.queue.len()
		{
			for command_index in 0..self.queue[party_index].command_count()
			{
				let priority_command = self.queue[priority].command_get(priority_index);

				if let Some(command) = self.queue[party_index].command_get(command_index)
				{
					if priority != party_index || priority_index != command_index
					{
						finished = false;
					}
					if CommandType::cmp(command, priority_command.unwrap(), parties, flags) == Ordering::Less
					{
						priority = party_index;
						priority_index = command_index;
					}
				}
			}
		}

		let command = self.queue[priority].command_take(priority_index);
		if finished
		{
			self.waiting = self.total;
		}
		command
	}

	pub fn member_remove(&mut self, party: usize, _: usize)
	{
		self.total -= 1;
		self.queue[party].member_remove();
	}
}
