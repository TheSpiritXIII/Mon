//! C wrapper for user generated species list.
use resource::ResourceList;
use types::{SpeciesId, FormId, ElementId, GroupId, GrowthId, ColorId, HabitatId, BaseStatType};
use gen::species_gen::SpeciesList;
use libc::c_char;
use species::Species;

const SPECIES_LIST: SpeciesList = SpeciesList::new();

#[no_mangle]
pub extern fn mon_species_get(id: SpeciesId) -> *const Species
{
	SPECIES_LIST.get(id).unwrap()
}

#[no_mangle]
pub extern fn mon_species_count() -> SpeciesId
{
	SpeciesList::count()
}

#[no_mangle]
pub extern fn mon_species_id(species: *const Species) -> SpeciesId
{
	unsafe
	{
		(*species).id()
	}
}

#[no_mangle]
pub extern fn mon_species_name(species: *const Species) -> *const c_char
{
	unsafe
	{
		(*species).name().as_ptr() as *const c_char
	}
}

#[no_mangle]
pub extern fn mon_species_description(species: *const Species) -> *const c_char
{
	unsafe
	{
		(*species).description().as_ptr() as *const c_char
	}
}

#[no_mangle]
pub extern fn mon_species_kind(species: *const Species) -> *const c_char
{
	unsafe
	{
		(*species).kind().as_ptr() as *const c_char
	}
}

#[no_mangle]
pub extern fn mon_species_form_id(species: *const Species, index: usize) -> FormId
{
	unsafe
	{
		(*species).form_ids()[index] as FormId
	}
}

#[no_mangle]
pub extern fn mon_species_form_count(species: *const Species) -> FormId
{
	unsafe
	{
		(*species).form_ids().len() as FormId
	}
}

#[no_mangle]
pub extern fn mon_species_form_name(species: *const Species, form: FormId) -> *const c_char
{
	unsafe
	{
		(*species).form_name(form).as_ptr() as *const c_char
	}
}

#[no_mangle]
pub extern fn mon_species_element(species: *const Species, form: FormId, index: usize) -> ElementId
{
	unsafe
	{
		(*species).elements(form)[index] as ElementId
	}
}

#[no_mangle]
pub extern fn mon_species_element_count(species: *const Species, form: FormId) -> ElementId
{
	unsafe
	{
		(*species).elements(form).len() as ElementId
	}
}

#[no_mangle]
pub extern fn mon_species_group(species: *const Species, index: usize) -> GroupId
{
	unsafe
	{
		(*species).groups()[index] as GroupId
	}
}

#[no_mangle]
pub extern fn mon_species_group_count(species: *const Species) -> GroupId
{
	unsafe
	{
		(*species).groups().len() as GroupId
	}
}

#[no_mangle]
pub extern fn mon_species_growth(species: *const Species) -> GrowthId
{
	unsafe
	{
		(*species).growth() as GrowthId
	}
}

#[no_mangle]
pub extern fn mon_species_color(species: *const Species) -> ColorId
{
	unsafe
	{
		(*species).color() as ColorId
	}
}

#[no_mangle]
pub extern fn mon_species_habitat(species: *const Species) -> HabitatId
{
	unsafe
	{
		(*species).habitat() as HabitatId
	}
}

#[no_mangle]
pub extern fn mon_species_height(species: *const Species, form: FormId) -> f32
{
	unsafe
	{
		(*species).height(form) as f32
	}
}

#[no_mangle]
pub extern fn mon_species_weight(species: *const Species, form: FormId) -> f32
{
	unsafe
	{
		(*species).height(form) as f32
	}
}

#[no_mangle]
pub extern fn mon_species_base_health(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_health(form) as BaseStatType
	}
}

#[no_mangle]
pub extern fn mon_species_base_attack(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_attack(form) as BaseStatType
	}
}

#[no_mangle]
pub extern fn mon_species_base_defense(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_defense(form) as BaseStatType
	}
}

#[no_mangle]
pub extern fn mon_species_base_spattack(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_spattack(form) as BaseStatType
	}
}

#[no_mangle]
pub extern fn mon_species_base_spdefense(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_spdefense(form) as BaseStatType
	}
}

#[no_mangle]
pub extern fn mon_species_base_speed(species: *const Species, form: FormId) -> BaseStatType
{
	unsafe
	{
		(*species).base_speed(form) as BaseStatType
	}
}
