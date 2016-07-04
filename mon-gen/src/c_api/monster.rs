use std::os::raw::c_char;

use num::FromPrimitive;

use base::types::species;
use base::types::gender::GenderId;
use base::monster::Monster;
use gen::species_list::SpeciesType;

pub use base::types::monster::*;

#[no_mangle]
pub extern fn mon_monster_create(species: species::Id, level: LevelType) -> *mut Monster
{
	Box::into_raw(Box::new(Monster::new(SpeciesType::from_usize(species as usize).unwrap(),
		level)))
}

#[no_mangle]
pub extern fn mon_monster_destroy(monster: *mut Monster)
{
	unsafe
	{
		drop(Box::from_raw(monster));
	}
}

#[no_mangle]
pub extern fn mon_monster_get_species(monster: *mut Monster) -> species::Id
{
	unsafe
	{
		(*monster).get_species() as species::Id
	}
}

#[no_mangle]
pub extern fn mon_monster_get_form(monster: *mut Monster) -> species::FormId
{
	unsafe
	{
		(*monster).get_form()
	}
}

#[no_mangle]
pub extern fn mon_monster_set_form(monster: *mut Monster, form: species::FormId)
{
	unsafe
	{
		(*monster).set_form(form);
	}
}

#[no_mangle]
pub extern fn mon_monster_get_nick(monster: *mut Monster) -> *const c_char
{
	unsafe
	{
		(*monster).get_nick_raw().as_ptr() as *const c_char
	}
}

#[no_mangle]
pub extern fn mon_monster_set_nick(monster: *mut Monster, nick: *const c_char)
{
	unsafe
	{
		(*monster).set_nick_raw(nick);
	}
}

#[no_mangle]
pub extern fn mon_monster_get_level(monster: *mut Monster) -> LevelType
{
	unsafe
	{
		(*monster).get_level()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_personality(monster: *mut Monster) -> PersonalityType
{
	unsafe
	{
		(*monster).get_personality()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_gender(monster: *mut Monster) -> GenderId
{
	unsafe
	{
		((*monster).get_gender()) as GenderId
	}
}

#[no_mangle]
pub extern fn mon_monster_get_nature(monster: *mut Monster) -> NatureId
{
	unsafe
	{
		(*monster).get_nature() as NatureId
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_health(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_health()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_attack(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_attack()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_defense(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_defense()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_spattack(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_spattack()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_spdefense(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_spdefense()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_speed(monster: *mut Monster) -> StatType
{
	unsafe
	{
		(*monster).get_stat_speed()
	}
}
