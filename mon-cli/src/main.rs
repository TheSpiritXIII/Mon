extern crate mon_gen;
extern crate rand;

mod display;
mod terminal;
mod main_original;

// mod main_experimental;
// use main_experimental as main_original;

fn main()
{
	main_original::main();
}
