use std::os::raw::c_char;

use num::traits::FromPrimitive;

pub use gen::element::*;

#[no_mangle]
pub extern fn mon_element_count() -> Id
{
	Element::count() as Id
}

#[no_mangle]
pub extern fn mon_element_name(element: Id) -> *const c_char
{
	abort_on_panic!(
	{
		Element::from_usize(element as usize).unwrap().name().as_ptr() as *const c_char
	})
}

#[no_mangle]
pub extern fn mon_element_effect(offending: Id, defending: Id) -> EffectType
{
	abort_on_panic!(
	{
		Element::from_usize(offending as usize).unwrap().effectiveness(
			Element::from_usize(defending as usize).unwrap())
	})
}
