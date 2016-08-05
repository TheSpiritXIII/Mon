extern crate mon_gen;

use mon_gen::monster::{Monster, StatType};
use mon_gen::battle::Party;
use mon_gen::species::SpeciesType;

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

	party_data[5].health_lose(StatType::max_value());

	// Basic case.
	{
		let party = Party::new(&mut party_data, 0, 2, false);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0) == 0);
		assert!(party.active_member_index(1) == 1);

		assert!(party.member_waiting_count() == 3);
	}

	// First member not alive.
	party_data[0].health_lose(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2, false);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0) == 1);
		assert!(party.active_member_index(1) == 2);

		assert!(party.member_waiting_count() == 2);
	}

	// Gaps between alive members.
	party_data[2].health_lose(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2, false);
		assert!(party.active_count() == 2);

		assert!(party.active_member_index(0) == 1);
		assert!(party.active_member_index(1) == 3);

		assert!(party.member_waiting_count() == 1);
	}

	// Less alive members than needed active.
	party_data[3].health_lose(StatType::max_value());
	party_data[4].health_lose(StatType::max_value());

	{
		let party = Party::new(&mut party_data, 0, 2, false);
		assert!(party.active_count() == 1);

		assert!(party.active_member_index(0) == 1);

		assert!(party.member_waiting_count() == 0);
	}
}
