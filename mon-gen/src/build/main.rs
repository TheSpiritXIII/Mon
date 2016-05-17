//! Main build crate.
#![feature(custom_derive, plugin, core_intrinsics, unicode)]
#![plugin(serde_macros)]

extern crate serde;
extern crate toml;
extern crate filetime;
extern crate num;

mod build;
#[macro_use] mod util;
#[path="../base/types.rs"] mod types;

// To deserialize:
mod elements;
mod gender;
mod species;
mod locations;
mod monster;
mod attacks;
mod battle;

fn main()
{
	// Cargo options.
	println!("cargo:rerun-if-changed=src/build/");
	println!("cargo:rerun-if-changed=../resouces");
	println!("");

 	let rebuild = cfg!(feature = "rebuild");
	let build_cache_dir = "target/gen/";
	let input_dir = "../resources/";
	let output_dir = "src/gen/";

	match build::build(build_cache_dir, input_dir, output_dir, rebuild)
	{
		Ok(b) =>
		{
			if !b
			{
				panic!("Failed to build")
			}
		}
		Err(_) =>
		{
			panic!("An unknown error has occured");
		}
	}
}
