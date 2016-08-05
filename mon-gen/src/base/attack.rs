use base::util::as_rust_str;
use gen::element::Element;
use gen::battle::Category;
use types::attack::{AccuracyType, LimitType, PowerType, PriorityType};

/// The target flags value type for `Target`. 
pub type TargetType = u8;

/// Constants for battle target bitflags.
pub struct Target;

impl Target
{
	/// The attack hits enemies.
	pub const SIDE_ENEMY: TargetType     = 0b00001;

	/// The attack hits allies.
	pub const SIDE_ALLY: TargetType      = 0b00010;

	/// The attack hits both enemies and allies.
	pub const SIDE_ALL: TargetType       = 0b00011;

	/// The attack hits targets adjacent to itself.
	pub const RANGE_ADJACENT: TargetType = 0b00100;

	/// The attack hits targets opposite of itself.
	pub const RANGE_OPPOSITE: TargetType = 0b01000;

	/// The attack hits is capable of hitting any target.
	pub const RANGE_ALL: TargetType      = 0b01100;

	/// The attack target may includes itself.
	pub const TARGET_SELF: TargetType    = 0b10000;
}

/// A single action used by a `Monster` in `Battle`.
#[derive(Debug)]
pub struct AttackMeta
{
	/// The default name of the attack as a raw C compatible string.
	pub name: &'static [u8],

	/// A short description of the attack as a raw C compatible string.
	pub description: &'static [u8],

	/// The elemental category of the attack.
	pub element: Element,

	/// The category of the attack that determines the damage calculation.
	pub category: Category,

	/// The base power for the move.
	pub power: PowerType,

	/// The accuracy value for using this move.
	pub accuracy: AccuracyType,

	/// The limit that this move can be used.
	pub limit: LimitType,

	/// The priority of the move in terms of whether it hits first.
	pub priority: PriorityType,

	/// The targets that this attack is capable of hitting.
	pub target: TargetType,
}

impl AttackMeta
{
	/// The default name of the attack.
	pub fn name(&self) -> &'static str
	{
		as_rust_str(self.name)
	}
	/// A short description of the attack.
	pub fn description(&self) -> &'static str
	{
		as_rust_str(self.description)
	}
}
