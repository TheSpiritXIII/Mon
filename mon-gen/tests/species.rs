// Test species values populated correctly, especially form differences:
// Elements: Shaymin (type 2)
// Ability: Shaymin
// Moveset: Deoxys
// Weight: Shaymin
// Hold Items: Basculin
// EV: Shaymin
extern crate mon_gen;

use mon_gen::{SpeciesType, GenderRatio, Growth, Color, Habitat, Group};
use mon_gen::DeoxysForm;

#[test]
fn species_values()
{
	let deoxys = SpeciesType::Deoxys.species();

	// Static traits (non-form changing).
	assert_eq!(deoxys.name(), "Deoxys");
	assert_eq!(deoxys.description(), "An alien virus that fell to earth on a meteor underwent a \
		DNA mutation to become this Pokémon.");
	assert_eq!(deoxys.kind(), "DNA");
	assert_eq!(deoxys.gender, GenderRatio::Genderless);
	assert_eq!(deoxys.growth, Growth::Slow);
	assert_eq!(deoxys.color, Color::Red);
	assert_eq!(deoxys.habitat, Habitat::Rare);
	assert_eq!(deoxys.rareness, 3);
	assert_eq!(deoxys.friendship, 0);
	assert_eq!(deoxys.hatch, 121);
	assert_eq!(deoxys.groups.len(), 1);
	assert_eq!(deoxys.groups[0], Group::Undiscovered);

	// Dynamic traits (form changing).
	assert_eq!(deoxys.base_health[DeoxysForm::Normal as usize], 50);
	assert_eq!(deoxys.base_attack[DeoxysForm::Normal as usize], 150);
	assert_eq!(deoxys.base_defense[DeoxysForm::Normal as usize], 50);
	assert_eq!(deoxys.base_spattack[DeoxysForm::Normal as usize], 150);
	assert_eq!(deoxys.base_spdefense[DeoxysForm::Normal as usize], 50);
	assert_eq!(deoxys.base_speed[DeoxysForm::Normal as usize], 150);

	assert_eq!(deoxys.base_health[DeoxysForm::Attack as usize], 50);
	assert_eq!(deoxys.base_attack[DeoxysForm::Attack as usize], 180);
	assert_eq!(deoxys.base_defense[DeoxysForm::Attack as usize], 20);
	assert_eq!(deoxys.base_spattack[DeoxysForm::Attack as usize], 180);
	assert_eq!(deoxys.base_spdefense[DeoxysForm::Attack as usize], 20);
	assert_eq!(deoxys.base_speed[DeoxysForm::Attack as usize], 150);

	assert_eq!(deoxys.base_health[DeoxysForm::Defense as usize], 50);
	assert_eq!(deoxys.base_attack[DeoxysForm::Defense as usize], 70);
	assert_eq!(deoxys.base_defense[DeoxysForm::Defense as usize], 160);
	assert_eq!(deoxys.base_spattack[DeoxysForm::Defense as usize], 70);
	assert_eq!(deoxys.base_spdefense[DeoxysForm::Defense as usize], 160);
	assert_eq!(deoxys.base_speed[DeoxysForm::Defense as usize], 90);

	assert_eq!(deoxys.base_health[DeoxysForm::Speed as usize], 50);
	assert_eq!(deoxys.base_attack[DeoxysForm::Speed as usize], 95);
	assert_eq!(deoxys.base_defense[DeoxysForm::Speed as usize], 90);
	assert_eq!(deoxys.base_spattack[DeoxysForm::Speed as usize], 95);
	assert_eq!(deoxys.base_spdefense[DeoxysForm::Speed as usize], 90);
	assert_eq!(deoxys.base_speed[DeoxysForm::Speed as usize], 180);
}
