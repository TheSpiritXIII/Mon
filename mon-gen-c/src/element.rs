use std::os::raw::c_char;
use num::FromPrimitive;

pub use mon_gen::species::{Element, ElementId, EffectType};

#[no_mangle]
pub extern fn mon_element_count() -> ElementId
{
	Element::count() as ElementId
}

#[no_mangle]
pub extern fn mon_element_name(element: ElementId) -> *const c_char
{
	Element::from_usize(element as usize).unwrap().name_raw().as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_element_effectiveness(offending: ElementId, defending: ElementId) -> EffectType
{
	Element::from_usize(offending as usize).unwrap().effectiveness(
		Element::from_usize(defending as usize).unwrap())
}
