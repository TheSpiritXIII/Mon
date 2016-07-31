//! General attributes that monsters commonly share.
// TODO: FormId is not used here, so a bit unnecessary.
pub use base::types::species::{FormId, RarenessType, FriendshipType, HatchType, MetricType, StatBaseType, ExperienceYieldType};
use base::types::monster::{LevelType};
use base::util::as_rust_str;
use gen::species::{Growth, Color, Habitat, Group};
use gen::element::Element;
use gen::gender::GenderRatio;
use gen::attack_list::AttackType;

pub struct Species
{
	/// The default name of the species.
	pub name: &'static [u8],

	/// A short description for the species.
	pub description: &'static [u8],

	/// The descriptive kind category that this species is.
	pub kind: &'static [u8],

	/// The gender ratio for this species.
	pub gender: GenderRatio,

	/// Returns the growth rate identifier for this species.
	pub growth: Growth,

	/// The general color of the species.
	pub color: Color,

	/// The natural habitat for the species.
	pub habitat: Habitat,

	/// The rareness value of the species. A lower value denotes a rarer species.
	pub rareness: RarenessType,

	/// The base friendliness value of the species. A lower value denotes a less friendly species.
	pub friendship: FriendshipType,

	/// The base hatch value of the species. A ower value denotes shorter to hatching time.
	pub hatch: HatchType,

	/// The units of experience this species yields in defeat.
	pub experience_yield: ExperienceYieldType,

	/// The form display names.
	pub forms: &'static [&'static [u8]],

	/// The species' elements per form.
	pub elements: &'static [&'static [Element]],

	/// The list of breeding groups that the species belongs to.
	pub groups: &'static [Group],

	/// The species base height in meters per form.
	pub height: &'static [MetricType],

	/// The species base weight in kilograms per form.
	pub weight: &'static [MetricType],

	/// The base health statistic per form.
	pub base_health: &'static [StatBaseType],

	/// The base attack statistic per form.
	pub base_attack: &'static [StatBaseType],

	/// The base defense statistic per form.
	pub base_defense: &'static [StatBaseType],

	/// The base special attack statistic per form.
	pub base_spattack: &'static [StatBaseType],

	/// The base special defense statistic per form.
	pub base_spdefense: &'static [StatBaseType],

	/// The base speed statistic per form.
	pub base_speed: &'static [StatBaseType],

	/// The yeild health amount, gained for default a monster of this species, per form.
	pub yield_health: &'static [StatBaseType],

	/// The yeild attack amount, gained for default a monster of this species, per form.
	pub yield_attack: &'static [StatBaseType],

	/// The yeild defense amount, gained for default a monster of this species, per form.
	pub yield_defense: &'static [StatBaseType],

	/// The yeild special amount, gained for default a monster of this species, statistic per form.
	pub yield_spattack: &'static [StatBaseType],

	/// The yeild special amount, gained for default a monster of this species, statistic per form.
	pub yield_spdefense: &'static [StatBaseType],

	/// The yeild speed amount, gained for default a monster of this species, per form.
	pub yield_speed: &'static [StatBaseType],

	/// The learnable attacks by this monster sorted, by level. Each level stores a list of
	/// attacks, per form.
	///
	/// The data here is expected to be sorted by level order, so that binary search is valid.
	///
	pub attacks_learnable: &'static [(LevelType, &'static [&'static [AttackType]])],

	// /// Returns the other species that the given monster is capable of evolving into.
	//pub evolve: fn(&Monster) -> Vec<Id>,
}

impl Species
{
	pub fn name(&self) -> &'static str
	{
		as_rust_str(self.name)
	}
	pub fn description(&self) -> &'static str
	{
		as_rust_str(self.description)
	}
	pub fn kind(&self) -> &'static str
	{
		as_rust_str(self.kind)
	}
	pub fn form(&self, form: usize) -> &'static str
	{
		as_rust_str(self.forms[form])
	}
}
