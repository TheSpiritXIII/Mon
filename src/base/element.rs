//! Generic element list trait.
use super::types::{ElementId, ElementEffectType};

/// Describes elements.
pub trait ElementList
{
	/// The number of elements.
	fn count() -> ElementId;
	
	/// Returns the name of the given element. Must be valid from 0 to `count()`.
	fn name(element: ElementId) -> &'static [u8];
	
	/// Given two opposing elements, return the effectiveness factor.
	fn effect(offense: ElementId, defense: ElementId) -> ElementEffectType;
}
