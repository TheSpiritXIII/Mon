// Test species values populated correctly, especially form differences:
// Base stats: Deoxys
// Elements: Shaymin (type 2)
// Ability: Shaymin
// Moveset: Deoxys
// Weight: Shaymin
// Hold Items: Basculin
// EV: Shaymin
extern crate mon_gen;

use mon_gen::SpeciesType;

use std::str;

fn from_utf8(string: &'static [u8]) -> &'static str
{
	&str::from_utf8(string).unwrap()[0 .. string.len() -1]
}

#[test]
fn species_values()
{
	let deoxys = SpeciesType::Deoxys.species();
	assert_eq!(from_utf8(deoxys.name), "Deoxys");
}
