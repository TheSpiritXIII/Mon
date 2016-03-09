//! GameMaker wwrapper for user generated species list.
use std::os::raw::{c_double, c_char};
use mon_gen::c_api::species;

#[no_mangle]
pub extern fn mon_species_count_gm() -> c_double
{
	species::mon_species_count() as c_double
}

#[no_mangle]
pub extern fn mon_species_name_gm(species: c_double) -> *const c_char
{
	species::mon_species_name(species as species::Id)
}

#[no_mangle]
pub extern fn mon_species_description_gm(species: c_double) -> *const c_char
{
	species::mon_species_description(species as species::Id)
}

#[no_mangle]
pub extern fn mon_species_kind_gm(species: c_double) -> *const c_char
{
	species::mon_species_kind(species as species::Id)
}

#[no_mangle]
pub extern fn mon_species_form_count_gm(species: c_double) -> c_double
{
	species::mon_species_form_count(species as species::Id) as c_double
}

#[no_mangle]
pub extern fn mon_species_form_name_gm(species: c_double, form: c_double) -> *const c_char
{
	species::mon_species_form_name(species as species::Id, form as species::FormId)
}

#[no_mangle]
pub extern fn mon_species_element_gm(species: c_double, form: c_double, index: c_double) -> c_double
{
	species::mon_species_element(species as species::Id, form as species::FormId, index as usize) as c_double
}

#[no_mangle]
pub extern fn mon_species_element_count_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_element_count(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_group_gm(species: c_double, index: c_double) -> c_double
{
	species::mon_species_group(species as species::Id, index as usize) as c_double
}

#[no_mangle]
pub extern fn mon_species_group_count_gm(species: c_double) -> c_double
{
	species::mon_species_group_count(species as species::Id) as c_double
}

#[no_mangle]
pub extern fn mon_species_growth_gm(species: c_double) -> c_double
{
	species::mon_species_growth(species as species::Id) as c_double
}

#[no_mangle]
pub extern fn mon_species_color_gm(species: c_double) -> c_double
{
	species::mon_species_color(species as species::Id) as c_double
}

#[no_mangle]
pub extern fn mon_species_habitat_gm(species: c_double) -> c_double
{
	species::mon_species_habitat(species as species::Id) as c_double
}

#[no_mangle]
pub extern fn mon_species_height_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_height(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_weight_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_weight(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_health_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_health(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_attack_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_attack(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_defense_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_defense(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_spattack_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_spattack(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_spdefense_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_spdefense(species as species::Id, form as species::FormId) as c_double
}

#[no_mangle]
pub extern fn mon_species_base_speed_gm(species: c_double, form: c_double) -> c_double
{
	species::mon_species_base_speed(species as species::Id, form as species::FormId) as c_double
}
