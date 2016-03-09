extern crate mon_gen;

use std::str;

use mon_gen::gen::species_list::Species;

fn main()
{
	println!("Hello mon! ID: {:?}", Species::Bulbasaur as usize);
	println!("Name: {}", str::from_utf8(Species::Bulbasaur.species().name).unwrap());
	println!("Description: {}", str::from_utf8(Species::Bulbasaur.species().description).unwrap());
}
