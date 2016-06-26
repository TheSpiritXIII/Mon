//! Defines data types and their respective sizes.

pub mod generic
{
	/// The location identifier.
	pub type LocationId = u8;

	/// The sub-location identifier.
	pub type SubLocationId = u8;
}

pub mod element
{
	/// The identifier type for Element.
	pub type Id = u8;

	/// The bonus effectiveness type for two opposing Element.
	pub type EffectType = f32;
}

pub mod attack
{
	/// The identifier type for Attack.
	pub type Id = u16;

	/// The type storing the base power for Attack.
	pub type PowerType = u8;

	/// The accuracy precision type for Attack.
	pub type AccuracyType = f32;

	/// The usage limit type for Attack.
	pub type LimitType = u8;

	/// The identifier value for Attack categories.
	pub type CategoryId = u8;

	/// The attack priority order type for Attack.
	pub type PriorityType = i8;
}

pub mod gender
{
	/// The identifier value used for storing `Gender	.
	pub type GenderId = u16;

	/// The identifier value used for storing `GenderRatio`.
	pub type GenderRatioId = u16;
}

pub mod species
{
	/// The identifier type for Species.
	pub type Id = u16;

	/// The forms identifier, unique to each individual, for Species.
	pub type FormId = u8;

	/// The ability identifier. Each Species typically has a few different abilties.
	pub type AbilityId = u8;

	/// The group classifier type for Species. Defines how monsters can breed.
	pub type GroupId = u8;

	/// The growth classifier type for Species. Defines how experience is gained.
	pub type GrowthId = u8;

	/// The color classifier type for Species used for filtering.
	pub type ColorId = u8;

	/// The natural habitat type that a Species is located in.
	pub type HabitatId = u8;

	/// A type denoting how fast or slow a Species gains experience within its growth group.
	pub type ExperienceGrowthType = u16;

	/// A type denoting how easy a monster is to catch.
	pub type RarenessType = u8;

	/// A type denoting how happy a monster starts.
	pub type FriendshipType = u8;

	/// A type denoting the maximum egg cycles a monster has before it hatches.
	pub type HatchType = u8;

	/// The type used for metrics for Species.
	pub type MetricType = f32;

	/// The maximum number a Species can have its base statistics.
	pub type StatBaseType = u8;

	/// The maximum number a Species can yield for monster statistic increases.
	pub type StatEvType = u8;
}

pub mod monster
{
	/// The experience level type for Monster.
	pub type LevelType = u8;

	/// Tge random personality value, unique to each Monster.
	pub type PersonalityType = u32;

	/// The nature for Monster which determines individual statistic bonuses.
	pub type NatureId = u8;

	/// The type used to hold statistics for Monster.
	pub type StatType = u16;

	/// The individual statistic value type.
	pub type StatIvType = u8;

	/// The identifier corresponding to the way a Monster was caught.
	pub type CaughtId = u8;

	/// A experience type for Monster.
	pub type ExperienceType = u8;
}

pub mod battle
{
	/// The type storing dynamic statistic modifiers while in battle.
	pub type StatModifierType = i8;
}
