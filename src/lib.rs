#![feature(libc)]
#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(box_patterns)]
#![feature(custom_derive, plugin)]
#![plugin(num_macros)]
#[macro_use]
extern crate libc;
extern crate num;
extern crate rand;

pub mod base;
pub mod gen;
pub mod export;

pub use gen::classifiers_gen;

pub use base::types;
pub use base::resource;
pub use base::element;
pub use base::species;
pub use base::monster;
pub use base::gender_ratio;

#[cfg(test)]
mod tests
{
	use gen::element::ElementList;
	
	use element::ElementList;

	#[test]
	fn elements_values_valid()
	{
		// Test to make sure the values do not crash.
		for element in 0..gen::element::ElementList::COUNT
		{
			gen::element::ElementList::name(element);
			for opposing in 0..gen::element::ElementList::COUNT
			{
				gen::element::ElementList::effect(element, opposing);
			}
		}
	}
}
