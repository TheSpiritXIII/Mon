#![feature(custom_derive, plugin, associated_consts, const_fn)]

extern crate rand;
extern crate num;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate abort_on_panic;

pub mod base;
mod gen;
mod calculate;

#[cfg(feature = "c_api")]
pub mod c_api;

#[cfg(test)]
mod test;

pub use base::monster::Monster;
pub use base::battle::Battle;
pub use gen::element::Element;
// pub use gen::species_list::SpeciesType;
pub use gen::species_list::*; // TODO: Only keep forms and SpeciesType from species_list.
pub use gen::attack_list::AttackType;
pub use base::types::species::FormId; // TODO:: Remove this.
pub use base::party::Party;
