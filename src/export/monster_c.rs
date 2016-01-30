use species::Species;
use types::{LevelType, FormId, StatType, NatureId, PersonalityType, GenderType};
use monster::Monster;
use libc::c_char;

#[no_mangle]
pub extern fn mon_monster_create(species: *const Species, level: LevelType) -> Option<*mut Monster>
{
	return Monster::new(species, level).map(|monster| return Box::into_raw(Box::new(monster)))
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
pub extern fn mon_monster_get_species(monster: *mut Monster) -> *const Species
{
	unsafe
	{
		(*monster).get_species()
	}
}

#[no_mangle]
pub extern fn mon_monster_get_form(monster: *mut Monster) -> FormId
{
	unsafe
	{
		(*monster).get_form()
	}
}

#[no_mangle]
pub extern fn mon_monster_set_form(monster: *mut Monster, form: FormId)
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
		(*monster).get_nick()
	}
}

#[no_mangle]
pub extern fn mon_monster_set_nick(monster: *mut Monster, nick: *const c_char)
{
	unsafe
	{
		(*monster).set_nick(nick);
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
pub extern fn mon_monster_get_gender(monster: *mut Monster) -> GenderType
{
	unsafe
	{
		((*monster).get_gender()) as GenderType
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
