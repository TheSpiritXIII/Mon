use rand::{StdRng, SeedableRng};
use std::io;

use base::command::{Command, CommandType};
use base::effect::Effect;
use base::party::Party;
use base::replay::{BattleReplay, BattleCommand};

use calculate::damage::{MemberIndex, calculate_experience};
use base::effect::{ExperienceGain, NoneReason};

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct BattlePartyMember
{
	pub party: usize,
	pub member: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BattleExecution
{
	/// A new command has been executed. An effect for this command has not been executed yet.
	Command,
	/// A new effect has been executed. A command for this effect is active.
	Effect,
	/// No possible execution is possible. The entire command list has been consumed.
	Ready,
	/// Waiting for user input.
	Waiting,
	/// A damage effect has been executed that resulted in the death of a party member.
	Death(BattlePartyMember),
	// Switch(usize),
	// SwitchWaiting,

	/// Occurs when the battle is over. Further commands cannot be added or processed.
	Finished(u8),
}

pub struct BattleRunner<'a>
{
	parties: Vec<Party<'a>>,
	replay: BattleReplay,
	effects: VecDeque<Effect>,
	rng: StdRng,
	command: usize,
	turn: usize,
	sides_alive: HashMap<u8, usize>,
	party_switch_waiting: usize,
	effect_current: Effect,
}

impl<'a> BattleRunner<'a>
{
	/// Generates a new battle object with a randomly generated RNG and empty command history.
	pub fn new(mut parties: Vec<Party<'a>>) -> Result<Self, io::Error>
	{
		let replay = BattleReplay::new()?;
		let rng = StdRng::from_seed(&[replay.seed()]);

		// Expose active parties for experience gaining.
		// Done after code that can return Result::Err so it's not done needlessly.
		let mut sides = HashMap::new();
		for party in &mut parties
		{
			party.expose_clear_all();
			let side_count = sides.entry(party.side()).or_insert(0);
			*side_count += 1;
		}
		for party_index in 0..parties.len()
		{
			BattleRunner::expose_party(&mut parties, party_index);
		}

		Ok(BattleRunner
		{
			parties: parties,
			replay: replay,
			effects: VecDeque::with_capacity(1),
			rng: rng,
			command: 0,
			turn: 0,
			sides_alive: sides,
			party_switch_waiting: 0,
			effect_current: Effect::None(NoneReason::None),
		})
	}

	/// Returns the list of participating parties.
	pub fn parties(&self) -> &[Party]
	{
		&self.parties
	}
	pub fn replay_mut(&mut self) -> &mut BattleReplay
	{
		&mut self.replay
	}

	/// The current executing command.
	///
	/// This method should not be called before run() is called. At this point, no commands exist
	/// and so this call will fail and crash.
	///
	pub fn current_command(&self) -> &BattleCommand
	{
		self.replay.command(self.command - 1)
	}

	#[deprecated]
	pub fn current_command_fix(&self) -> Option<&Command>
	{
		if let BattleCommand::Action(ref action) = *self.replay.command(self.command - 1)
		{
			Some(action)
		}
		else
		{
			None
		}
	}

	/// The current executing result of the current executing command.
	///
	/// See `current_command` for a note on calling this function.
	///
	pub fn current_effect(&self) -> &Effect
	{
		&self.effect_current
	}

	/// Executes the next consecutive command effect. Returns the result of the command.
	///
	/// Execution goes as follows:
	/// - For the current command, all effects are applied.
	/// - Checks are done to see if there is a winner yet.
	/// - The next command is lined up for execution.
	///
	pub fn run(&mut self) -> BattleExecution
	{
		if self.command != 0 && !self.effects.is_empty()
		{
			let effect = self.effects.pop_back().unwrap();
			let execution = self.apply_effect(&effect);
			self.effect_current = effect;
			execution
		}
		else if self.sides_alive.len() <= 1
		{
			// TODO: Handle ties.
			BattleExecution::Finished(*self.sides_alive.keys().next().unwrap())
		}
		else if self.command < self.replay.command_count()
		{
			self.effects.clear();
			
			if let BattleCommand::Action(ref command) = *self.replay.command(self.command)
			{
				let hit = if let CommandType::Attack(ref attack_command) = *command.command_type()
				{
					let hit =
					{
						self.parties[attack_command.target_party].active_member_alive(attack_command.target_member).is_some()
					};

					let party = &mut self.parties[command.party()];
					party.active_member_attack_limit_take(attack_command.member,
						attack_command.attack_index);

					hit
				}
				else
				{
					true
				};

				if hit
				{
					let mut effects = Vec::new();
					command.command_type().effects(&self.parties, command, &mut self.rng, &mut effects);

					self.effects = VecDeque::from(effects);
					// TODO: Optimize command_type().effect() to take in a VecDeque.
				}
				else
				{
					self.effects.push_back(Effect::None(NoneReason::Miss));
				}
			}
			else // BattleCommand::Turn
			{
				for x in 0..self.parties.len()
				{
					let party = self.parties.get_mut(x).unwrap();
					party.active_purge();
				}
				self.effects.push_back(Effect::None(NoneReason::None));
				self.turn += 1;
			}
			self.command += 1;
			BattleExecution::Command
		}
		else
		{
			BattleExecution::Ready
		}
		// Check battle finished with winner.
		// Increment command counter.
	}

	// fn affects_member(command: &Command, member: usize) -> bool
	// {
	// 	match *command.command_type()
	// 	{
	// 		CommandType::Attack(ref attack_command) =>
	// 		{
	// 			attack_command.member == member
	// 		}
	// 		CommandType::Switch(ref switch_command) =>
	// 		{
	// 			switch_command.member == member
	// 		}
	// 		CommandType::Escape =>
	// 		{
	// 			false
	// 		}
	// 	}
	// }

	fn apply_effect_damage(&mut self, user_party: usize, user_active: usize, target_party: usize,
		target_active: usize, amount: u16) -> BattleExecution
	{
		let member = target_active;

		if self.parties[target_party].active_member_lose_health(member, amount)
		{
			let offense_party = user_party;
			if self.parties[offense_party].gain_experience()
			{
				let offense = MemberIndex
				{
					party: offense_party,
					member: user_active,
				};
				let defense = MemberIndex
				{
					party: target_party,
					member: target_active,
				};
				let experience_map = calculate_experience(&self.parties, Some(offense), defense);

				// TODO: Add item/ability modification here.

				for experience_party in &experience_map
				{
					let party = experience_party.0;
					for experience_member in experience_party.1.iter()
					{
						let member = *experience_member.0;
						let amount = *experience_member.1;
						let level = self.parties[*party].member(member).level();
						let gain = ExperienceGain::new(*party, member, amount, level);
						self.effects.push_back(Effect::ExperienceGain(gain));
					}
				}
			}

			if !self.parties[target_party].active_waiting()
			{
				if !self.parties[target_party].active_are_alive()
				{
					let side = self.parties[target_party].side();
					let left = *self.sides_alive.get(&side).unwrap();
					if left == 1
					{
						self.sides_alive.remove(&side);
					}
					else
					{
						*self.sides_alive.get_mut(&side).unwrap() -= 1;
					}
				}
			}
			else
			{
				self.party_switch_waiting += 1;
			}

			BattleExecution::Death(BattlePartyMember
			{
				party: target_party,
				member: target_active,
			})
		}
		else
		{
			BattleExecution::Effect
		}
	}

	fn apply_effect(&mut self, effect: &Effect) -> BattleExecution
	{
		match *effect
		{
			Effect::Damage(ref effect) =>
			{
				let party = self.current_command_fix().unwrap().party();
				self.apply_effect_damage(party, effect.member(), effect.party(), effect.active,
					effect.amount())
			}
			Effect::Switch(ref switch) =>
			{
				let party_index = switch.party;
				self.parties[party_index].switch_active(switch.member, switch.target);
				BattleRunner::expose_party(&mut self.parties, party_index);
				BattleExecution::Effect
			}
			Effect::Retreat(_) =>
			{
				// TODO: Implement this.
				BattleExecution::Effect
			}
			Effect::Modifier(ref modifiers) =>
			{
				let party = &mut self.parties[modifiers.party()];
				party.active_member_modifiers_add(modifiers.active(), modifiers.modifiers());
				BattleExecution::Effect
			}
			Effect::ExperienceGain(ref experience_gain) =>
			{
				self.parties[experience_gain.party].member_experience_add(experience_gain.member,
					experience_gain.amount);
				BattleExecution::Effect
			}
			Effect::None(_) =>
			{
				// Ignore.
				BattleExecution::Effect
			}
		}
	}
	fn expose_party(parties: &mut Vec<Party<'a>>, party_index: usize)
	{
		// Allow clippy lint to be ignored.
		// Clippy is wrong in this case because the index is used to prevent mutable borrow.
		#![allow(needless_range_loop)]
		let switch_side = parties[party_index].side();
		for index in 0..parties.len()
		{
			if parties[index].side() != switch_side
			{
				for active_index in 0..parties[party_index].active_count()
				{
					let expose_reference = parties[party_index].expose_reference(active_index);
					parties[index].expose_add_member(party_index, expose_reference);
				}
			}
		}
	}
}
