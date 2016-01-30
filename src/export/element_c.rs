//! C wrapper for user generated element list.
use element::ElementList as BaseElementList;
use gen::element_gen::ElementList;
use types::{ElementId, ElementEffectType};
use libc::c_char;

#[no_mangle]
pub extern fn mon_element_count() -> ElementId
{
	ElementList::count()
}

#[no_mangle]
pub extern fn mon_element_name(element: ElementId) -> *const c_char
{
	ElementList::name(element).as_ptr() as *const c_char
}

#[no_mangle]
pub extern fn mon_element_effect(offending: ElementId, defending: ElementId) -> ElementEffectType
{
	ElementList::effect(offending, defending)
}
