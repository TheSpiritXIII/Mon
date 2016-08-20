use std::os::raw::c_char;
use std::ffi::CStr;
use num::FromPrimitive;

use mon_gen::monster::{Monster, LevelType, PersonalityType, NatureId, StatType};
use mon_gen::species::{SpeciesId, SpeciesType, FormId, GenderId};

#[no_mangle]
pub extern fn mon_monster_create(species: SpeciesId, level: LevelType) -> *mut Monster
{
	Box::into_raw(Box::new(Monster::new(SpeciesType::from_usize(species as usize).unwrap(), level)))
}

#[no_mangle]
pub unsafe extern fn mon_monster_destroy(monster: *mut Monster)
{
	drop(Box::from_raw(monster));
}

#[no_mangle]
pub unsafe extern fn mon_monster_species(monster: *mut Monster) -> SpeciesId
{
	(*monster).species() as SpeciesId
}

#[no_mangle]
pub unsafe extern fn mon_monster_form(monster: *mut Monster) -> FormId
{
	(*monster).form()
}

#[no_mangle]
pub unsafe extern fn mon_monster_set_form(monster: *mut Monster, form: FormId)
{
	(*monster).form_set(form);
}

#[no_mangle]
pub unsafe extern fn mon_monster_nick(monster: *mut Monster) -> *const c_char
{
	(*monster).nick_raw().as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern fn mon_monster_set_nick(monster: *mut Monster, nick: *const c_char)
{
	// TODO: Validate safe utf8.
	let nick = CStr::from_ptr(nick).to_owned();
	(*monster).nick_raw_set(nick);
}

#[no_mangle]
pub unsafe extern fn mon_monster_level(monster: *mut Monster) -> LevelType
{
	(*monster).level()
}

#[no_mangle]
pub unsafe extern fn mon_monster_personality(monster: *mut Monster) -> PersonalityType
{
	(*monster).personality()
}

#[no_mangle]
pub unsafe extern fn mon_monster_gender(monster: *mut Monster) -> GenderId
{
	((*monster).gender()) as GenderId
}

#[no_mangle]
pub unsafe extern fn mon_monster_nature(monster: *mut Monster) -> NatureId
{
	(*monster).nature() as NatureId
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_health(monster: *mut Monster) -> StatType
{
	(*monster).stat_health()
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_attack(monster: *mut Monster) -> StatType
{
	(*monster).stat_attack()
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_defense(monster: *mut Monster) -> StatType
{
	(*monster).stat_defense()
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_spattack(monster: *mut Monster) -> StatType
{
	(*monster).stat_spattack()
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_spdefense(monster: *mut Monster) -> StatType
{
	(*monster).stat_spdefense()
}

#[no_mangle]
pub unsafe extern fn mon_monster_stat_speed(monster: *mut Monster) -> StatType
{
	(*monster).stat_speed()
}
