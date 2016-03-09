#![feature(custom_derive, plugin, associated_consts, const_fn)]

extern crate rand;
extern crate num;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate abort_on_panic;

pub mod base;
pub mod gen;
pub mod calculate;

#[cfg(feature = "c_api")]
pub mod c_api;
