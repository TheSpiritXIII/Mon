//! Defines data types and their respective sizes.

/// The identifier type for Element.
pub type ElementId = u8;

/// The precision effectiveness for opposing Element.
pub type ElementEffectType = f32;

/// The identifier type for Ability.
pub type AbilityId = u8;

/// The identifier type for Move.
pub type MoveId = u16;

/// The identifier type for Species.
pub type SpeciesId = u16;

// ::Species Types::

// The maximum level a monster can become.
pub type LevelType = u8;

/// The maximum number of forms a Species can have.
pub type FormId = u8;

/// The group classifier type for Species. Defines how monsters can breed.
pub type GroupId = u8;

/// The growth classifier type for Species. Defines how experience is gained.
pub type GrowthId = u8;

/// The color classifier type for Species used for filtering.
pub type ColorId = u8;

pub type HabitatId = u8;

pub type MetricType = f32;

/// The maximum rareness value a monster can be. Defines how easy a monster is to catch.
pub type RarenessType = u8;

/// The maximum happiness value a monster can be. Defines how happy a monster can be.
pub type HappinessType = u8;

/// The maximum egg hatch cycles a monster has.
pub type HatchType = u8;

/// The maximum number a Species can have its base statistics.
pub type BaseStatType = u8;

/// The maximum number a Species can yield for monster statistic increases.
pub type YieldType = u8;

pub type LocationId = u16;

// ::Monster Types::

pub type NatureId = u8;

pub type StatType = u16;

pub type PersonalityType = u32;

pub type EvType = u8;

pub type IvType = u8;

pub type GenderType = u8;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum Gender
{
	Genderless = 0,
	Male       = 1,
	Female     = 2,
}
