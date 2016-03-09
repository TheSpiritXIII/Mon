//! GameMaker wwrapper for user generated element list.
use std::os::raw::{c_double, c_char};

use mon_gen::c_api::element;

#[no_mangle]
pub extern fn mon_element_count_gm() -> c_double
{
	element::mon_element_count() as c_double
}

#[no_mangle]
pub extern fn mon_element_name_gm(element: c_double) -> *const c_char
{
	element::mon_element_name(element as element::Id)
}

#[no_mangle]
pub extern fn mon_element_effect_gm(offending: c_double, defending: c_double) -> c_double
{
	element::mon_element_effect(offending as element::Id, defending as element::Id) as c_double
}
