use std::collections::{VecDeque};
use std::cmp::Ordering;
use std::num::Wrapping;

use base::types::monster::StatType;
use base::monster::Monster;

use calculate::damage::calculate_damage;

use rand::{Rng, StdRng};

#[derive(Debug)]
pub struct Party<'a>
{
	pub members: &'a mut [Monster],
	side: u8,
	active: Vec<usize>,
}

impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], out: usize) -> Self
	{
		let mut party = Party
		{
			members: members,
			side: 1,
			active: Vec::with_capacity(out),
		};

		let mut current = Wrapping(usize::max_value());
		for _ in 0..party.active.capacity()
		{
			current = Wrapping(party.next_alive((current + Wrapping(1usize)).0));
			println!("Yolo swag {}", current);
			if current.0 == party.members.len()
			{
				break;
			}
			party.active.push(current.0);
			if party.active.len() == party.active.capacity()
			{
				break;
			}
		}
		party
	}
	fn next_alive(&self, party: usize) -> usize
	{
		for member_index in party..self.members.len()
		{
			if self.members[member_index].get_health() != 0
			{
				return member_index;
			}
		}
		println!("Yolo swag");
		self.members.len()
	}
	pub fn active_member(&self, index: usize) -> &Monster
	{
		&self.members[index]
	}
	pub fn active_count(&self) -> usize
	{
		self.active.len()
	}
}

#[derive(Debug)]
pub struct CommandAttack
{
	pub attack_index: usize,
	pub party: usize,
	pub monster: usize,
}

#[derive(Debug)]
pub enum CommandType
{
	Attack(CommandAttack),
	//Item(ItemId),
	//Switch(ToId),
	Escape,
}

#[derive(Debug)]
pub struct Command
{
	pub command_type: CommandType,
	pub party: usize,
	pub monster: usize,
}

impl Command
{
	fn new(command_type: CommandType, party: usize, monster: usize,) -> Self
	{
		Command
		{
			command_type: command_type,
			party: party,
			monster: monster,
		}
	}
}

impl Command
{
	fn cmp(command_self: &Command, monster_self: &Monster, command_other: &Command,
		monster_other: &Monster) -> Ordering
	{
		match command_self.command_type
		{
			CommandType::Attack(_) =>
			{
				if let CommandType::Attack(_) = command_other.command_type
				{
					monster_other.get_stat_speed().cmp(&monster_self.get_stat_speed())
				}
				else
				{
					Ordering::Greater
				}
			}
			CommandType::Escape =>
			{
				if let CommandType::Escape = command_other.command_type
				{
					let group = command_self.party.cmp(&command_other.party);
					if group == Ordering::Equal
					{
						command_self.monster.cmp(&command_other.monster)
					}
					else
					{
						group
					}
				}
				else
				{
					Ordering::Less
				}
			}
		}
	}
}

impl CommandType
{
	fn effects<'a, R: Rng>(&self, parties: &Vec<Party<'a>>, command: &Command, rng: &mut R) -> VecDeque<Effect>
	{
		let mut v = VecDeque::new();
		match *self
		{
			CommandType::Attack(ref target) =>
			{
				let offense = &parties[command.party].members[command.monster];
				let defense = &parties[target.party].members[target.monster];
				let damage = Damage
				{
					amount: calculate_damage(offense, 0, defense, false, 1f32, rng),
					party: target.party,
					monster: target.monster
				};
				v.push_back(Effect::Damage(damage));
			}
			CommandType::Escape =>
			{
				v.push_back(Effect::None(Reason::Escape));
			},
		}
		v
	}
}

#[derive(Debug)]
pub struct Damage
{
	amount: StatType,
	party: usize,
	monster: usize,
}

impl Damage
{
	pub fn amount(&self) -> StatType
	{
		self.amount
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn member(&self) -> usize
	{
		self.monster
	}
}

#[derive(Debug)]
pub enum Reason
{
	Critical,
	Escape,
}

#[derive(Debug)]
pub enum Effect
{
	Damage(Damage),
	// Status(StatusId),
	// Bonus(BonusType),
	// Ability(AbilityId),
	// Miss,
	// ,
	// Uneffective,
	// LessEffective,
	// SuperEffecitve,
	None(Reason),
}

#[derive(Debug)]
struct BattleCommand
{
	command: Command,
	effects: VecDeque<Effect>,
}

impl BattleCommand
{
	fn new<'a, R: Rng>(command: Command, parties: &Vec<Party<'a>>, rng: &mut R) -> Self
	{
		let effects = command.command_type.effects(parties, &command, rng);
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
	ready: Vec<Vec<bool>>,

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

	// TODO: lingering effects.
}

pub enum BattleExecution
{
	Command,
	Queue,
	Waiting,
}

impl<'a> Battle<'a>
{
	pub fn new(parties: Vec<Party<'a>>) -> Self
	{
		let mut total = 0;
		let mut ready = Vec::with_capacity(parties.len());
		for group in parties.iter()
		{
			total += group.active_count();
			ready.push(vec![false; group.active_count()]);
		}
		Battle
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
		}
	}
	pub fn party(&self, index: usize) -> &Party<'a>
	{
		&self.parties[index]
	}
	pub fn monster(&self, party: usize, monster: usize) -> &Monster
	{
		&self.parties[party].members[monster]
	}
	pub fn monster_active(&self, party: usize, monster: usize) -> &Monster
	{
		self.parties[party].active_member(monster)
	}
	pub fn monster_active_count(&self, party: usize) -> usize
	{
		self.parties[party].active_count()
	}
	fn monster_command(&self, command: &Command) -> &Monster
	{
		self.monster(command.party, command.monster)
	}
	/// Adds a command to the turn queue. Returns true if the command is a valid command.
	pub fn add_command(&mut self, command: CommandType, party: usize, monster: usize) -> bool
	{
		if self.started
		{
			println!("Already starte1.");
			return false;
		}
		if party >= self.parties.len()
		{
			println!("Already len 1.");
			return false;
		}
		if self.ready[party][monster] == true
		{
			println!("Already ready {}.", monster);
			return false;
		}
		if monster >= self.parties[party].members.len()
		{
			println!("Already len.");
			return false
		}

		// TODO: Check if attack is valid against target.

		self.ready[party][monster] = true;
		self.waiting -= 1;
		println!("{} {:?}", self.waiting, self.ready);

		self.queue.push(Command::new(command, party, monster));
		true
	}

	fn execute_command(&mut self) -> BattleExecution
	{
		let mut min_index = 0;
		for index in 1..self.queue.len()
		{
			if Command::cmp(&self.queue[index], self.monster_command(&self.queue[index]),
				&self.queue[min_index], self.monster_command(&self.queue[min_index])) == Ordering::Less
			{
				min_index = index;
			}
		}
		let command = self.queue.swap_remove(min_index);
		if let CommandType::Attack(ref attack) = command.command_type
		{
			let target = self.parties[command.party].members.get_mut(command.monster).unwrap();
			target.get_attacks_mut()[attack.attack_index].limit_left_take(1);
		}

		let battle_command = BattleCommand::new(command, &self.parties, &mut self.rng);
		self.commands.push(battle_command);
		self.current = 0;
		BattleExecution::Command
	}

	/// Executes the next battle action.
	pub fn execute(&mut self) -> BattleExecution
	{
		if self.started
		{
			if self.current != self.commands.last().unwrap().effects.len()
			{
				self.apply_effect();

				self.current += 1;
				BattleExecution::Queue
			}
			else if !self.queue.is_empty()
			{
				self.execute_command()
			}
			else
			{
				// Reset the waiting for new commands.
				self.waiting = self.total;
				self.started = false;
				for ready_party in self.ready.iter_mut()
				{
					for ready in ready_party.iter_mut()
					{
						*ready = false;
					}
				}
				BattleExecution::Waiting
			}
		}
		else
		{
			if self.waiting != 0
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

	fn apply_effect(&mut self)
	{
		let battle_command = self.commands.last().unwrap();
		match battle_command.effects[self.current]
		{
			Effect::Damage(ref effect) =>
			{
				let target = self.parties[effect.party].members.get_mut(effect.monster).unwrap();
				target.lose_health(effect.amount);
			}
			_ => ()
		}
	}
}
