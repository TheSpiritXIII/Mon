#![feature(custom_derive, plugin, associated_consts, const_fn, question_mark)]

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

/// Actions that can be used during battle between parties by members.
pub mod attack
{
	pub use base::attack::*;
	pub use gen::attack_list::*;
	pub use types::attack::*;
}

/// Parties and versing between them.
pub mod battle
{
	pub use base::party::*;
	pub use base::runner::BattleRunner;
	pub use base::runner::BattleExecution;
	pub use base::queue::BattleQueue;
	pub use base::battle::Battle;
	pub use base::battle::BattleError;
	pub use base::replay::BattleCommand;
	pub use base::command::CommandType;
	pub use base::effect::*;
	pub use base::statmod::*;
	pub use types::battle::*;
}

/// Party members with meta-data and actions.
pub mod monster
{
	pub use base::monster::*;
	pub use types::monster::*;
}

/// General metadata for party members.
pub mod species
{
	pub use base::species::*;
	pub use gen::species::*;
	pub use gen::species_list::*;
	pub use gen::element::*;
	pub use gen::gender::*;
	pub use types::species::*;
	pub use types::gender::*;
}