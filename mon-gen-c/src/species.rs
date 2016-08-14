use std::os::raw::c_char;

use mon_gen::species::{SpeciesType, SpeciesId, FormId, GroupId, GrowthId, ColorId, HabitatId,
	StatBaseType, ElementId};

#[no_mangle]
pub extern fn mon_species_count() -> SpeciesId
{
	SpeciesType::count()
}

#[no_mangle]
pub extern fn mon_species_name(species: SpeciesId) -> *const c_char
{
	SpeciesType::from_id(species).name.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_description(species: SpeciesId) -> *const c_char
{
	SpeciesType::from_id(species).description.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_kind(species: SpeciesId) -> *const c_char
{
	SpeciesType::from_id(species).kind.as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_form_count(species: SpeciesId) -> FormId
{
	SpeciesType::from_id(species).forms.len() as FormId
}

#[no_mangle]
pub extern fn mon_species_form_name(species: SpeciesId, form: FormId) -> *const c_char
{
	SpeciesType::from_id(species).forms[form as usize].as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_species_element(species: SpeciesId, form: FormId, index: usize) -> ElementId
{
	SpeciesType::from_id(species).elements[form as usize][index as usize] as ElementId
}

#[no_mangle]
pub extern fn mon_species_element_count(species: SpeciesId, form: FormId) -> ElementId
{
	SpeciesType::from_id(species).elements[form as usize].len() as ElementId
}

#[no_mangle]
pub extern fn mon_species_group(species: SpeciesId, index: usize) -> GroupId
{
	SpeciesType::from_id(species).groups[index] as GroupId
}

#[no_mangle]
pub extern fn mon_species_group_count(species: SpeciesId) -> GroupId
{
	SpeciesType::from_id(species).groups.len() as GroupId
}

#[no_mangle]
pub extern fn mon_species_growth(species: SpeciesId) -> GrowthId
{
	SpeciesType::from_id(species).growth as GrowthId
}

#[no_mangle]
pub extern fn mon_species_color(species: SpeciesId) -> ColorId
{
	SpeciesType::from_id(species).color as ColorId
}

#[no_mangle]
pub extern fn mon_species_habitat(species: SpeciesId) -> HabitatId
{
	SpeciesType::from_id(species).habitat as HabitatId
}

#[no_mangle]
pub extern fn mon_species_height(species: SpeciesId, form: FormId) -> f32
{
	SpeciesType::from_id(species).height[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_weight(species: SpeciesId, form: FormId) -> f32
{
	SpeciesType::from_id(species).weight[form as usize] as f32
}

#[no_mangle]
pub extern fn mon_species_base_health(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_health[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_attack(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_attack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_defense(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_defense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spattack(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_spattack[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_spdefense(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_spdefense[form as usize] as StatBaseType
}

#[no_mangle]
pub extern fn mon_species_base_speed(species: SpeciesId, form: FormId) -> StatBaseType
{
	SpeciesType::from_id(species).base_speed[form as usize] as StatBaseType
}
