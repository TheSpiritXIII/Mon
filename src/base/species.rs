//! Generic species trait.
use types::{SpeciesId, FormId, MetricType, BaseStatType};
use classifiers_gen::{Group, Growth, Color, Habitat};
use gen::element_gen::Element;
use super::gender_ratio::GenderRatio;

pub trait Species
{
	fn id(&self) -> SpeciesId;
	
	/// Returns the display name of the species.
	fn name(&self) -> &'static [u8];
	
	/// Returns a short description for the species.
	fn description(&self) -> &'static [u8];
	
	/// Returns the descriptive kind that this species is.
	fn kind(&self) -> &'static [u8];
	
	/// Returns a vector containing valid form IDs for this species.
	fn form_ids(&self) -> Vec<FormId>;
	
	/// Returns the form name for the given form ID. Must be valid from 0 to `form_count()`.
	fn form_name(&self, form: FormId) -> &'static [u8];
	
	/// Returns the element at the given index. Must be valid from 0 to `element_count()`.
	fn elements(&self, form: FormId) -> Vec<Element>;
	
	// /// Returns the ability at the given index. Must be valid from 0 to `ability_count()`.
	// fn ability(&self, index: u8) -> AbilityId;
	
	// /// Returns the number of abilities this species has.
	// fn ability_count(&self) -> u8;
	
	/// Returns the group at the given index. Must be valid from 0 to `group_size`.
	fn groups(&self) -> Vec<Group>;
	
	/// Returns the growth rate type of this species.
	fn growth(&self) -> Growth;
	
	/// The general color of the species.
	fn color(&self) -> Color;
	
	/// The natural habitat for the species.
	fn habitat(&self) -> Habitat;
	
	/// The gender ratio value.
	fn gender_ratio(&self) -> GenderRatio;
	
	/// The species base height in meters.
	fn height(&self, form: FormId) -> MetricType;
	
	/// The species base weight in kilograms.
	fn weight(&self, form: FormId) -> MetricType;
	
	// /// The rareness value of the species. Lower number means rarer.
	// fn rareness(&self) -> RarenessSize;
	
	// /// The base friendliness value of the species. Lower number means less friendly.
	// fn friendship(&self) -> HappinessSize;
	
	// /// The base hatch value of the species. Lower number means shorter to hatch.
	// fn hatch(&self) -> HatchSize;
	
	/// The base health statistic.
	fn base_health(&self, form: FormId) -> BaseStatType;
	
	/// The base attack statistic.
	fn base_attack(&self, form: FormId) -> BaseStatType;
	
	/// The base defense statistic.
	fn base_defense(&self, form: FormId) -> BaseStatType;
	
	/// The base special attack statistic.
	fn base_spattack(&self, form: FormId) -> BaseStatType;
	
	/// The base special defense statistic.
	fn base_spdefense(&self, form: FormId) -> BaseStatType;
	
	/// The base speed statistic.
	fn base_speed(&self, form: FormId) -> BaseStatType;
	
	// /// The yield health amount, gained for defating a monster of this species.
	// fn yield_health(&self) -> YieldType;
	
	// /// The yield attack amount, gained for defating a monster of this species.
	// fn yield_attack(&self) -> YieldType;
	
	// /// The yield defense amount, gained for defating a monster of this species.
	// fn yield_defense(&self) -> YieldType;
	
	// /// The yield special attack amount, gained for defating a monster of this species.
	// fn yield_spattack(&self) -> YieldType;
	
	// /// The yield special defense amount, gained for defating a monster of this species.
	// fn yield_spdefense(&self) -> YieldType;
	
	// /// The yield speed amount, gained for defating a monster of this species.
	// fn yield_speed(&self) -> YieldType;
	
	// /// Returns the learnable moves at the given level.
	// fn moves(&self, monster: Monster, location: LocationId, party: bool) -> Vec<MoveId>;
	
	// /// Returns the evolutions 
	// fn evolve(&self, monster: Monster, location: LocationId, party: bool, party: bool) -> Vec<SpeciesType>;
}
