//! Manages allocation for monster based on memory relative indices.
use std::os::raw::{c_double, c_char};
use std::collections::{HashMap};
use std::ptr;

use mon_gen::base::monster::Monster;
use mon_gen::c_api::monster;
use mon_gen::c_api::species;

use mon_gen::base::types::monster::LevelType;

fn bool_cast_gm(b: bool) -> c_double
{
	if b
	{
		1.0
	}
	else
	{
		0.0
	}
}

pub static mut monster_counter: usize = 0;
pub static mut monster_map: *mut HashMap<usize, *mut Monster> = ptr::null_mut();

#[no_mangle]
pub extern fn mon_monster_create_gm(species: c_double, level: c_double) -> c_double
{
	return match monster::mon_monster_create(species as species::Id, level as LevelType)
	{
		Some(monster) => unsafe
		{
			let index = monster_counter;
			monster_counter += 1;

			(*monster_map).insert(index, monster);
			index as c_double
		},
		None => -1 as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_exists_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		bool_cast_gm((*monster_map).contains_key(&index))
	}
}

#[no_mangle]
pub extern fn mon_monster_destroy_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		return match (*monster_map).remove(&index)
		{
			Some(mon) =>
			{
				monster::mon_monster_destroy(mon);
				1 as c_double
			},
			None => 0 as c_double
		}
	}
}

#[no_mangle]
pub extern fn mon_monster_get_species_id_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_species(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_form_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_form(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_set_form_gm(monster: c_double, form: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_set_form(*(*monster_map).get(&index).unwrap(), form as species::FormId);
	}
	1 as c_double
}

#[no_mangle]
pub extern fn mon_monster_get_nick_gm(monster: c_double) -> *const c_char
{
	unsafe
	{
		let index = monster as usize;
		monster::mon_monster_get_nick(*(*monster_map).get(&index).unwrap())
	}
}

#[no_mangle]
pub extern fn mon_monster_set_nick_gm(monster: c_double, nick: *const c_char) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_set_nick(*(*monster_map).get(&index).unwrap(), nick);
	}
	1 as c_double
}

#[no_mangle]
pub extern fn mon_monster_get_level_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_level(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_personality_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_personality(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_gender_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_gender(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_nature_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_nature(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_health_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_health(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_attack_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_attack(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_defense_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_defense(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_spattack_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_spattack(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_spdefense_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_spdefense(*(*monster_map).get(&index).unwrap()) as c_double
	}
}

#[no_mangle]
pub extern fn mon_monster_get_stat_speed_gm(monster: c_double) -> c_double
{
	let index = monster as usize;
	unsafe
	{
		monster::mon_monster_get_stat_speed(*(*monster_map).get(&index).unwrap()) as c_double
	}
}
