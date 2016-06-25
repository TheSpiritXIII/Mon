use base::monster::Monster;
use base::types::battle::StatModifier;

use std::slice;
use std::num::Wrapping;

#[derive(Debug)]
pub struct MemberStatModifiers
{
	pub attack: StatModifier,
	pub defense: StatModifier,
	pub sp_attack: StatModifier,
	pub sp_defense: StatModifier,
	pub speed: StatModifier,
	pub evasion: StatModifier,
	pub accuracy: StatModifier,
	pub critical: StatModifier,
}

#[derive(Debug)]
struct PartyMember
{
	member: usize,
	modifiers: MemberStatModifiers,
}

impl MemberStatModifiers
{
	fn new() -> MemberStatModifiers
	{
		MemberStatModifiers
		{
			attack: 0,
			defense: 0,
			sp_attack: 0,
			sp_defense: 0,
			speed: 0,
			evasion: 0,
			accuracy: 0,
			critical: 0,
		}
	}
}

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],
	active: Vec<Option<PartyMember>>,
	side: u8,
	// TODO: Cache count of post-turn switch waiting members here maybe?
	// TODO: Add for experience gaining: gain_experience: bool
	// TODO: Add vec item_locked: bool,
}

impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], out: usize) -> Self
	{
		let mut party = Party
		{
			members: members,
			active: Vec::with_capacity(out),
			side: 1,
		};

		let mut current = Wrapping(usize::max_value());
		for _ in 0..party.active.capacity()
		{
			current = Wrapping(party.next_alive((current + Wrapping(1usize)).0));
			if current.0 == party.members.len()
			{
				break;
			}
			party.active.push(Some(PartyMember
			{
				member: current.0,
				modifiers: MemberStatModifiers::new(),
			}));
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
				!self.member_is_active(member_index)
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
		self.active.iter().any(|active_member_option|
		{
			if let Some(ref active_member) = *active_member_option
			{
				active_member.member == index
			}
			else
			{
				false
			}
		})
	}
	pub fn switch_active(&mut self, member: usize, target: usize)
	{
		// TODO: Allow this when member is already active.
		self.members.swap(self.active[member].as_ref().unwrap().member, target);
	}
	pub fn switch_waiting(&self) -> Option<usize>
	{
		self.active.iter().position(|member| member.is_none())
	}
	pub fn active_member(&self, index: usize) -> Option<&Monster>
	{
		self.active[index].as_ref().map(|active_member| &self.members[active_member.member])
	}
	pub fn active_member_index(&self, index: usize) -> Option<usize>
	{
		self.active[index].as_ref().map(|active_member| active_member.member)
	}
	pub fn active_count(&self) -> usize
	{
		self.active.len()
	}
	pub fn active_set(&mut self, active: usize, target: usize)
	{
		// TODO: Should be allowed when active already set?
		self.active[active] = Some(PartyMember
		{
			member: target,
			modifiers: MemberStatModifiers::new(),
		});
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
	pub fn active_member_modifiers(&self, index: usize) -> &MemberStatModifiers
	{
		&self.active[index].as_ref().unwrap().modifiers
	}
	pub fn active_stage_modifiers_mut(&mut self, index: usize) -> &mut MemberStatModifiers
	{
		&mut self.active[index].as_mut().unwrap().modifiers
	}
}
