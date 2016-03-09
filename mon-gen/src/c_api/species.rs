//! C wrapper for user generated species list.
use std::os::raw::c_char;

use base::types::element;
use gen::species_list::SPECIES_LIST;

pub use base::types::species::*;

#[no_mangle]
pub extern fn mon_species_count() -> Id
{
	SPECIES_LIST.len() as Id
}

#[no_mangle]
pub extern fn mon_species_name(species: Id) -> *const c_char
{
	SPECIES_LIST[species as usize].name.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_description(species: Id) -> *const c_char
{
	SPECIES_LIST[species as usize].description.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_kind(species: Id) -> *const c_char
{
	SPECIES_LIST[species as usize].kind.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_form_count(species: Id) -> FormId
{
	SPECIES_LIST[species as usize].forms.len() as FormId
}

#[no_mangle]
pub extern fn mon_species_form_name(species: Id, form: FormId) -> *const c_char
{
	SPECIES_LIST[species as usize].forms[form as usize].as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_element(species: Id, form: FormId, index: usize) -> element::Id
{
	SPECIES_LIST[species as usize].elements[form as usize][index as usize] as element::Id
}

#[no_mangle]
pub extern fn mon_species_element_count(species: Id, form: FormId) -> element::Id
{
	SPECIES_LIST[species as usize].elements[form as usize].len() as element::Id
}

#[no_mangle]
pub extern fn mon_species_group(species: Id, index: usize) -> GroupId
{
	SPECIES_LIST[species as usize].groups[index] as GroupId
}

#[no_mangle]
pub extern fn mon_species_group_count(species: Id) -> GroupId
{
	SPECIES_LIST[species as usize].groups.len() as GroupId
}

#[no_mangle]
pub extern fn mon_species_growth(species: Id) -> GrowthId
{
	SPECIES_LIST[species as usize].growth as GrowthId
}

#[no_mangle]
pub extern fn mon_species_color(species: Id) -> ColorId
{
	SPECIES_LIST[species as usize].color as ColorId
}

#[no_mangle]
pub extern fn mon_species_habitat(species: Id) -> HabitatId
{
	SPECIES_LIST[species as usize].habitat as HabitatId
}

#[no_mangle]
pub extern fn mon_species_height(species: Id, form: FormId) -> f32
{
	SPECIES_LIST[species as usize].height[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_weight(species: Id, form: FormId) -> f32
{
	SPECIES_LIST[species as usize].weight[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_base_health(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_health[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_attack(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_attack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_defense(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_defense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spattack(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_spattack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spdefense(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_spdefense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_speed(species: Id, form: FormId) -> StatBaseType
{
	SPECIES_LIST[species as usize].base_speed[form as usize] as StatBaseType
}
