//! Generic attack trait.
use base::types::attack::{PowerType, AccuracyType, LimitType, PriorityType};

use gen::element::Element;
use gen::battle::Category;

/// Stores the target flags for Attack.
pub type TargetType = u8;

pub mod target
{
	/// The attack hits enemies.
	pub const SIDE_ENEMY: super::TargetType     = 0b00001;

	/// The attack hits allies.
	pub const SIDE_ALLY: super::TargetType      = 0b00010;

	/// The attack hits both enemies and allies.
	pub const SIDE_ALL: super::TargetType       = 0b00011;

	/// The attack hits targets adjacent to itself.
	pub const RANGE_ADJACENT: super::TargetType = 0b00100;

	/// The attack hits targets opposite of itself.
	pub const RANGE_OPPOSITE: super::TargetType = 0b01000;

	/// The attack hits is capable of hitting any target.
	pub const RANGE_ALL: super::TargetType      = 0b01100;

	/// The attack target may includes itself.
	pub const TARGET_SELF: super::TargetType    = 0b10000;
}

/// Defines a single attacking action.
#[derive(Debug)]
pub struct AttackMeta
{
	/// The default name of the attack.
	pub name: &'static [u8],

	/// A short description of the attack.
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

	// /// The priority of the move in terms of whether it hits first.
	pub priority: PriorityType,

	/// The targets that this attack is capable of hitting.
	pub target: TargetType,
}
