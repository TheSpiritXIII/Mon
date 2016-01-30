//! Defines common functions for parsing and building resources.
extern crate num;

#[path = "../base/types.rs"]
mod types;

mod identifier;
mod display;

#[macro_use]
mod parse;

mod element;
mod species;
mod classifiers;

pub use self::element::parse_elements;
pub use self::species::parse_species;
pub use self::classifiers::parse_classifiers;
