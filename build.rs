//! Parses and builds resources in the `res` folder.
#![feature(custom_derive, plugin, custom_attribute, unicode)]
#![plugin(serde_macros)]

extern crate serde;
extern crate toml;

#[path = "src/build/mod.rs"]
mod build;

#[path = "src/base/types.rs"]
mod types;

use std::fs;
use std::path::Path;

fn main()
{
	// Cargo options.
	println!("cargo:rerun-if-changed=./res/");
	
	println!("Building source...\n");
	if !Path::new("import").exists()
	{
		if let Err(err) = fs::create_dir("import")
		{
			panic!("Unable to create directory `import`: {:?}", err.kind());
		}
	}
	else
	{
		let constants_file = fs::OpenOptions::new().truncate(true).create(true).open(
			"import/constants.txt");
		if let Err(err) = constants_file
		{
			panic!("Unable to remove file `constants.txt`: {:?}", err.kind());
		}
	}
	
	build::parse_classifiers();
	println!("");
	
	build::parse_elements();
	println!("");
	
	build::parse_species();
	println!("");

	println!("Done.");
}
