//! C wrapper for user generated species list.
use std::os::raw::c_char;

use base::types::element;
use gen::species_list::SpeciesType;

pub use base::types::species::*;

#[no_mangle]
pub extern fn mon_species_count() -> Id
{
	SpeciesType::count()
}

#[no_mangle]
pub extern fn mon_species_name(species: Id) -> *const c_char
{
	SpeciesType::from_id(species).name.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_description(species: Id) -> *const c_char
{
	SpeciesType::from_id(species).description.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_kind(species: Id) -> *const c_char
{
	SpeciesType::from_id(species).kind.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_form_count(species: Id) -> FormId
{
	SpeciesType::from_id(species).forms.len() as FormId
}

#[no_mangle]
pub extern fn mon_species_form_name(species: Id, form: FormId) -> *const c_char
{
	SpeciesType::from_id(species).forms[form as usize].as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_element(species: Id, form: FormId, index: usize) -> element::Id
{
	SpeciesType::from_id(species).elements[form as usize][index as usize] as element::Id
}

#[no_mangle]
pub extern fn mon_species_element_count(species: Id, form: FormId) -> element::Id
{
	SpeciesType::from_id(species).elements[form as usize].len() as element::Id
}

#[no_mangle]
pub extern fn mon_species_group(species: Id, index: usize) -> GroupId
{
	SpeciesType::from_id(species).groups[index] as GroupId
}

#[no_mangle]
pub extern fn mon_species_group_count(species: Id) -> GroupId
{
	SpeciesType::from_id(species).groups.len() as GroupId
}

#[no_mangle]
pub extern fn mon_species_growth(species: Id) -> GrowthId
{
	SpeciesType::from_id(species).growth as GrowthId
}

#[no_mangle]
pub extern fn mon_species_color(species: Id) -> ColorId
{
	SpeciesType::from_id(species).color as ColorId
}

#[no_mangle]
pub extern fn mon_species_habitat(species: Id) -> HabitatId
{
	SpeciesType::from_id(species).habitat as HabitatId
}

#[no_mangle]
pub extern fn mon_species_height(species: Id, form: FormId) -> f32
{
	SpeciesType::from_id(species).height[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_weight(species: Id, form: FormId) -> f32
{
	SpeciesType::from_id(species).weight[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_base_health(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_health[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_attack(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_attack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_defense(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_defense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spattack(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_spattack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spdefense(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_spdefense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_speed(species: Id, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_speed[form as usize] as StatBaseType
}
