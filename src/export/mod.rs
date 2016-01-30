//! Exports C comaptible bindings.
#[macro_use]
macro_rules! extern_if
{
	($func:ty) =>
	{
		//#[cfg(feature = $feature_name)]
		#[no_mangle]
		pub extern $func
		
	}
}

pub mod element_c;
pub mod monster_c;
pub mod species_c;

pub mod gm;
