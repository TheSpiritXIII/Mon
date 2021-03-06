extern crate mon_gen_c;

pub mod element;
pub mod species;
pub mod monster;

use mon_gen::base::monster::Monster;
use std::collections::HashMap;

/// Allocates data for the library.
///
/// This function must be used before any other function. mon_free() must always always follow this
/// function. Calling any other function before this one is undefined behavior.
///
#[no_mangle]
pub extern fn mon_init_gm()
{
	let hsh: HashMap<usize, *mut Monster> = HashMap::new();
	let monster_map_box = Box::new(hsh);
	unsafe
	{
		monster::monster_map = Box::into_raw(monster_map_box);
	}
}

/// Deallocates all data for the library.
///
/// This function must only be used after a call from `mon_init()`. After calling this function,
/// `mon_init()` must be called again in order to use any other functions.
///
#[no_mangle]
pub extern fn mon_free_gm()
{
	unsafe
	{
		drop(Box::from_raw(monster::monster_map));
	}
}
