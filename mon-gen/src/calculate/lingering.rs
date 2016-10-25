trait Lingering
{
	/// Creates an effect. Returns `true` to terminate after activing.
	fn effect(effects: &mut BattleEffects) -> bool;

	/// Updates the underlying object's state. Returns `true` to active an effect.
	fn state_change(&mut self) -> bool;

	/// Activates after the object was created.
	fn after_create<R: Rng>(&mut self, state: &BattleState, rng: &mut R);

	/// Activates at the end of a turn. Returns `true` to change state.
	///
	/// Returns `false` by default.
	///
	fn after_turn(&self) -> bool
	{
		false
	}
}

enum Lingering
{
	PerishSong,
}

struct PerishSongEffect
{
	turn: u8,
	affected: Vec<(usize, usize)>,
}

impl PerishSongEffect
{
	fn new() -> Self
	{
		PerishSongEffect
		{
			turn: 0,
			affected: Vec::new(),
		}
	}
	fn turns() -> usize
	{
		self.turn
	}
}

impl LastingEffect for PerishSongEffect
{
	fn effect(effects: &mut BattleEffects) -> bool
	{
		// TODO: Kill all affected.
	}

	fn state_change(&mut self) -> bool
	{
		turn += 1;
		turn == 5
	}

	fn after_turn(&self) -> bool
	{
		true
	}
}
