//! Data types and their respective sizes.

pub mod generic
{
	/// The location identifier.
	pub type LocationId = u8;

	/// The sub-location identifier, as paired with `LocationId`.
	pub type SubLocationId = u8;
}

pub mod element
{
	/// The identifier type for `Element`.
	pub type ElementId = u8;

	/// The effectiveness value type for two opposing `Element`.
	pub type EffectType = f32;
}

pub mod attack
{
	/// The identifier type for `Attack`.
	pub type AttackId = u16;

	/// The base power type for `Attack`.
	pub type PowerType = u8;

	/// The accuracy precision type for `Attack`.
	pub type AccuracyType = f32;

	/// The usage limit type for `Attack`.
	pub type LimitType = u8;

	/// The identifier value for `Attack` categories, `Category` enum.
	pub type CategoryId = u8;

	/// The attack priority order type for `Attack`.
	pub type PriorityType = i8;
}

pub mod gender
{
	/// The identifier value for `Gender`.
	pub type GenderId = u8;

	/// The identifier value for `GenderRatio`.
	pub type GenderRatioId = u8;
}

pub mod species
{
	/// The identifier type for `Species`.
	pub type SpeciesId = u16;

	/// The forms identifier, unique to each individual species, for `Species`.
	pub type FormId = u8;

	/// The ability identifier. Each `Species` typically has a few different abilties.
	pub type AbilityId = u8;

	/// The identifier type for `Species` groups, `Group` enum.
	pub type GroupId = u8;

	/// The identifier type for `Species` growth rates, `Growth` enum.
	pub type GrowthId = u8;

	/// The identifier type for `Species` colors, `Color` enum.
	pub type ColorId = u8;

	/// The identifier type for `Species` natural habitats, `Habitat` enum.
	pub type HabitatId = u8;

	/// The experience that a `Species` yields for defeat.
	pub type ExperienceYieldType = u16;

	/// How easy a `Species` is to recruit.
	pub type RarenessType = u8;

	/// How friendly a `Species` is upon recruit.
	pub type FriendshipType = u8;

	/// The maximum egg cycles a `Species` must before through before it hatches.
	pub type HatchType = u8;

	/// Metric type for `Species` used for height and weight.
	pub type MetricType = f32;

	/// The value type for `Species` base statistics.
	pub type StatBaseType = u8;

	/// The value type for `Species` can yield for statistic bonuses.
	pub type StatYieldType = u8;
}

pub mod monster
{
	/// The value type for `Monster` displaying how much experience it has gained.
	pub type LevelType = u8;

	/// Tge random personality value, unique to each `Monster`.
	pub type PersonalityType = u32;

	/// The identifier type for `Monster` natures, `Nature` enum.
	pub type NatureId = u8;

	/// The value type for `Monster` statistics.
	pub type StatType = u16;

	/// The value type for `Monster` unique statistics.
	pub type StatIndividualType = u8;

	/// The identifier type for `Monster` recruit methods, `RecruitMethod` enum.
	pub type RecruitMethodId = u8;

	/// A value type for `Monster` experience points.
	pub type ExperienceType = u32;
}

pub mod battle
{
	/// The type storing dynamic statistic modifiers while in battle.
	pub type StatModifierType = i8;
}
