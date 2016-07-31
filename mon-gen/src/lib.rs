#![feature(custom_derive, plugin, associated_consts, const_fn)]

extern crate rand;
extern crate num;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate enum_primitive;

mod base;
mod calculate;
mod types;

// Use a macro to expand modules so non-existing modules will not fail compile.
macro_rules! mod_gen_default
{
	(true) =>
	{
		mod gen;
	};
	(false) =>
	{
		pub mod gen_test;
		pub use gen_test as gen;
	};
}

#[cfg(not(feature = "test"))]
mod_gen_default!(true);

#[cfg(feature = "test")]
mod_gen_default!(false);

// pub use base::monster::Monster;
// pub use base::battle::Battle;
// pub use gen::element::Element;
// // pub use gen::species_list::SpeciesType;
// pub use gen::species_list::*; // TODO: Only keep forms and SpeciesType from species_list.
// pub use gen::attack_list::AttackType;
// pub use types::species::FormId; // TODO:: Remove this.
// pub use base::party::Party;

// pub use gen::species::*;
// pub use gen::gender::*;

pub mod attack
{
	pub use base::attack::*;
	pub use gen::attack_list::*;
}

pub mod species
{
	pub use base::species::*;
	pub use gen::species::*;
	pub use gen::species_list::*;
	pub use gen::element::*;
	pub use gen::gender::*;
}

pub mod monster
{
	pub use base::monster::*;
}

pub mod battle
{
	pub use base::party::*;
	pub use base::battle::*;
}
