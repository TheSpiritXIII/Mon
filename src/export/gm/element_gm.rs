//! GameMaker wwrapper for user generated element list.
use types::ElementId;
use libc::{c_double, c_char};
use super::element_c;

#[no_mangle]
pub extern fn mon_element_count_gm() -> c_double
{
	element_c::mon_element_count() as c_double
}

#[no_mangle]
pub extern fn mon_element_name_gm(element: c_double) -> *const c_char
{
	element_c::mon_element_name(element as ElementId)
}

#[no_mangle]
pub extern fn mon_element_effect_gm(offending: c_double, defending: c_double) -> c_double
{
	element_c::mon_element_effect(offending as ElementId, defending as ElementId) as c_double
}
