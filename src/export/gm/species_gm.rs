//! GameMaker wwrapper for user generated species list.
use types::{SpeciesId, FormId};
use libc::{c_double, c_char};
use super::species_c;

#[no_mangle]
pub extern fn mon_species_count_gm() -> c_double
{
	species_c::mon_species_count() as c_double
}

#[no_mangle]
pub extern fn mon_species_name_gm(species: c_double) -> *const c_char
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_name(s)
}

#[no_mangle]
pub extern fn mon_species_description_gm(species: c_double) -> *const c_char
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_description(s)
}

#[no_mangle]
pub extern fn mon_species_kind_gm(species: c_double) -> *const c_char
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_kind(s)
}

#[no_mangle]
pub extern fn mon_species_form_id_gm(species: c_double, index: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_form_id(s, index as usize) as c_double
}

#[no_mangle]
pub extern fn mon_species_form_count_gm(species: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_form_count(s) as c_double
}

#[no_mangle]
pub extern fn mon_species_form_name_gm(species: c_double, form: c_double) -> *const c_char
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_form_name(s, form as FormId)
}

#[no_mangle]
pub extern fn mon_species_element_gm(species: c_double, form: c_double, index: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_element(s, form as FormId, index as usize) as c_double
}

#[no_mangle]
pub extern fn mon_species_element_count_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_element_count(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_group_gm(species: c_double, index: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_group(s, index as usize) as c_double
}

#[no_mangle]
pub extern fn mon_species_group_count_gm(species: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_group_count(s) as c_double
}

#[no_mangle]
pub extern fn mon_species_growth_gm(species: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_growth(s) as c_double
}

#[no_mangle]
pub extern fn mon_species_color_gm(species: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_color(s) as c_double
}

#[no_mangle]
pub extern fn mon_species_habitat_gm(species: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_habitat(s) as c_double
}

#[no_mangle]
pub extern fn mon_species_height_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_height(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_weight_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_weight(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_health_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_health(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_attack_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_attack(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_defense_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_defense(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_spattack_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_spattack(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_spdefense_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_spdefense(s, form as FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_speed_gm(species: c_double, form: c_double) -> c_double
{
	let s = species_c::mon_species_get(species as SpeciesId);
	species_c::mon_species_base_speed(s, form as FormId) as c_double
}
