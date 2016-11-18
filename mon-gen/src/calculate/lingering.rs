use base::runner::{BattleEffects, BattleState};
use base::effect::Lingering;

#[derive(Debug, Clone, PartialEq)]
pub struct PerishSong
{
	turn: u8,
	affected: Vec<(usize, usize)>,
}

impl PerishSong
{
	pub fn new() -> Self
	{
		PerishSong
		{
			turn: 0,
			affected: Vec::new(),
		}
	}
	pub fn turns(&self) -> u8
	{
		self.turn
	}
}

impl Lingering for PerishSong
{
	fn effect(&self, _: &mut BattleEffects, _: &BattleState) -> bool
	{
		// for (party_index, member_index) in self.affected
		// {
		// 	// TODO: Kill all affected.

		// }
		true
	}

	fn state_change(&mut self) -> bool
	{
		self.turn += 1;
		self.turn == 5
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
	PerishSong(PerishSong),
}

impl Lingering for LingeringType
{
	fn effect(&self, effects: &mut BattleEffects, state: &BattleState) -> bool
	{
		match *self
		{
			LingeringType::PerishSong(ref linger_state) =>
			{
				linger_state.effect(effects, state)
			}
		}
	}

	fn state_change(&mut self) -> bool
	{
		match *self
		{
			LingeringType::PerishSong(ref mut linger_state) =>
			{
				linger_state.state_change()
			}
		}
	}

	fn after_create(&mut self, state: &BattleState)
	{
		match *self
		{
			LingeringType::PerishSong(ref mut linger_state) =>
			{
				linger_state.after_create(state)
			}
		}
	}

	fn after_turn(&self) -> bool
	{
		match *self
		{
			LingeringType::PerishSong(ref linger_state) =>
			{
				linger_state.after_turn()
			}
		}
	}
}
