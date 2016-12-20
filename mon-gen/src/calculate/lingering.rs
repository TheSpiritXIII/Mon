use base::runner::{BattleEffects, BattleState};
use base::effect::Lingering;
use calculate::common::knock_out_member;

#[derive(Debug, Clone, PartialEq)]
pub struct DeathAllTurns
{
	turn: u8,
	affected: Vec<(usize, usize)>,
}

impl DeathAllTurns
{
	pub fn new(turns: u8) -> Self
	{
		DeathAllTurns
		{
			turn: turns,
			affected: Vec::new(),
		}
	}
	pub fn turns(&self) -> u8
	{
		self.turn
	}
}

impl Lingering for DeathAllTurns
{
	fn effect(&self, effects: &mut BattleEffects, state: &BattleState) -> bool
	{
		for &(party_index, member_index) in &self.affected
		{
			knock_out_member(effects, state, party_index, member_index);
		}
		true
	}

	fn state_change(&mut self) -> bool
	{
		self.turn -= 1;
		self.turn == 0
	}

	fn after_create(&mut self, state: &BattleState)
	{
		for party_index in 0..state.parties().len()
		{
			let party = &state.parties()[party_index];
			for active_index in 0..party.active_count()
			{
				self.affected.push((party_index, party.active_member_reference(active_index)));
			}
		}
	}

	fn after_turn(&self) -> bool
	{
		true
	}
}

// TODO: Using a tagged union makes it difficult to expose this to C.

#[derive(Debug, Clone, PartialEq)]
pub enum LingeringType
{
	DeathAllTurns(DeathAllTurns),
}

impl Lingering for LingeringType
{
	fn effect(&self, effects: &mut BattleEffects, state: &BattleState) -> bool
	{
		match *self
		{
			LingeringType::DeathAllTurns(ref linger_state) =>
			{
				linger_state.effect(effects, state)
			}
		}
	}

	fn state_change(&mut self) -> bool
	{
		match *self
		{
			LingeringType::DeathAllTurns(ref mut linger_state) =>
			{
				linger_state.state_change()
			}
		}
	}

	fn after_create(&mut self, state: &BattleState)
	{
		match *self
		{
			LingeringType::DeathAllTurns(ref mut linger_state) =>
			{
				linger_state.after_create(state)
			}
		}
	}

	fn after_turn(&self) -> bool
	{
		match *self
		{
			LingeringType::DeathAllTurns(ref linger_state) =>
			{
				linger_state.after_turn()
			}
		}
	}
}
