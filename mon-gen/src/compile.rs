//! Main build crate.
#![feature(custom_derive, plugin, core_intrinsics, unicode)]
#![plugin(serde_macros)]

extern crate serde;
extern crate toml;
extern crate filetime;
extern crate num;

mod build;
mod types;

fn main()
{
	println!("cargo:rerun-if-changed=src/build/");
	println!("cargo:rerun-if-changed=../resouces");
	println!("");

 	let rebuild = cfg!(feature = "rebuild");
	let (build_cache_dir, input_dir, output_dir) = if !cfg!(feature = "test")
	{
		println!("Running without tests");
		("target/gen/", "../resources/", "src/gen/")
	}
	else
	{
		println!("Running with tests");
		("target/gen_test/", "../sample/", "src/gen_test/")
	};

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
