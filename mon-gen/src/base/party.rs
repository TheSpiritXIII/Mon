use base::monster::Monster;
// use base::types::battle::StatModifier;

use std::slice;
use std::num::Wrapping;

// #[derive(Debug)]
// struct PartyMember
// {
// 	member: usize,
// 	attack: StatModifier,
// 	defense: StatModifier,
// 	sp_attack: StatModifier,
// 	sp_defense: StatModifier,
// 	speed: StatModifier,
// 	evasion: StatModifier,
// 	accuracy: StatModifier,
// 	item_locked: bool,
// }

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],

	side: u8,
	active: Vec<Option<usize>>,
	// TODO: Cache count of post-turn switch waiting members here maybe?
	// TODO: Add for experience gaining: gain_experience: bool
}


impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], out: usize) -> Self
	{
		let mut party = Party
		{
			members: members,
			side: 1,
			active: Vec::with_capacity(out),
		};

		let mut current = Wrapping(usize::max_value());
		for _ in 0..party.active.capacity()
		{
			current = Wrapping(party.next_alive((current + Wrapping(1usize)).0));
			if current.0 == party.members.len()
			{
				break;
			}
			party.active.push(Some(current.0));
			if party.active.len() == party.active.capacity()
			{
				break;
			}
		}
		party
	}
	fn next_alive(&self, party: usize) -> usize
	{
		for member_index in party..self.members.len()
		{
			if self.members[member_index].get_health() != 0 &&
				!self.active.contains(&Some(member_index))
			{
				return member_index;
			}
		}
		self.members.len()
	}
	pub fn member(&self, index: usize) -> &Monster
	{
		&self.members[index]
	}
	pub fn member_mut(&mut self, index: usize) -> &mut Monster
	{
		// TODO: Maybe remove this function?
		self.members.get_mut(index).unwrap()
	}
	pub fn member_count(&self) -> usize
	{
		self.members.len()
	}
	pub fn member_is_active(&self, index: usize) -> bool
	{
		self.active.contains(&Some(index))
	}
	pub fn switch_active(&mut self, member: usize, target: usize)
	{
		// TODO: Allow this when member is already active.
		self.members.swap(self.active[member].unwrap(), target);
	}
	pub fn switch_waiting(&self) -> Option<usize>
	{
		self.active.iter().position(|member| member.is_none())
	}
	pub fn active_member(&self, index: usize) -> Option<&Monster>
	{
		self.active[index].map(|active_index| &self.members[active_index])
	}
	pub fn active_member_index(&self, index: usize) -> Option<usize>
	{
		self.active[index].map(|active_index| active_index)
	}
	pub fn active_count(&self) -> usize
	{
		self.active.len()
	}
	pub fn active_set(&mut self, active: usize, target: usize)
	{
		// TODO: Should be allowed when active already set?
		self.active[active] = Some(target);
	}
	pub fn active_reset(&mut self, active: usize)
	{
		// TODO: Maybe just add a lose_health function that does this automatically?
		self.active[active] = None;
	}
	pub fn iter(&self) -> slice::Iter<Monster>
	{
		self.members.iter()
	}
	pub fn count(&self) -> usize
	{
		self.members.len()
	}
}
