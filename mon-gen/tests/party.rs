extern crate mon_gen;

use mon_gen::{Party, SpeciesType, Monster};
use mon_gen::base::types::monster::StatType;

#[test]
fn party_active_order()
{
	let mut party_data =
	[
		Monster::new(SpeciesType::Deoxys, 1),
		Monster::new(SpeciesType::Deoxys, 2),
		Monster::new(SpeciesType::Deoxys, 3),
		Monster::new(SpeciesType::Deoxys, 4),
		Monster::new(SpeciesType::Deoxys, 5),
		Monster::new(SpeciesType::Deoxys, 6),
	];

	party_data[5].lose_health(StatType::max_value());

	// Basic case.
	{
		let party = Party::new(&mut party_data, 0, 2);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0).unwrap() == 0);
		assert!(party.active_member_index(1).unwrap() == 1);

		assert!(party.member_waiting_count() == 3);
	}

	// First member not alive.
	party_data[0].lose_health(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0).unwrap() == 1);
		assert!(party.active_member_index(1).unwrap() == 2);

		assert!(party.member_waiting_count() == 2);
	}

	// Gaps between alive members.
	party_data[2].lose_health(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0).unwrap() == 1);
		assert!(party.active_member_index(1).unwrap() == 3);

		assert!(party.member_waiting_count() == 1);
	}

	// Less alive members than needed active.
	party_data[3].lose_health(StatType::max_value());
	party_data[4].lose_health(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2);
		assert!(party.active_count() == 1);

		assert!(party.active_member_index(0).unwrap() == 1);

		assert!(party.member_waiting_count() == 0);
	}
}
