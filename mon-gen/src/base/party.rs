use base::monster::Monster;
use base::types::monster::StatType;
use base::types::attack::AccuracyType;

use std::slice;
use std::num::Wrapping;

use base::statmod::StatModifiers;

pub struct PartyMember<'a>
{
	pub member: &'a Monster,
	pub modifiers: &'a StatModifiers,
}

impl<'a> PartyMember<'a>
{
	fn stat(stat: StatType, modifier: AccuracyType) -> StatType
	{
		(stat as AccuracyType * modifier) as StatType
	}
	pub fn attack(&self) -> StatType
	{
		PartyMember::stat(self.member.get_stat_attack(), self.modifiers.attack_value())
	}
	pub fn defense(&self) -> StatType
	{
		PartyMember::stat(self.member.get_stat_defense(), self.modifiers.defense_value())
	}
	pub fn sp_attack(&self) -> StatType
	{
		PartyMember::stat(self.member.get_stat_spattack(), self.modifiers.sp_attack_value())
	}
	pub fn sp_defense(&self) -> StatType
	{
		PartyMember::stat(self.member.get_stat_spdefense(), self.modifiers.sp_defense_value())
	}
	pub fn speed(&self) -> StatType
	{
		PartyMember::stat(self.member.get_stat_speed(), self.modifiers.speed_value())
	}
}

#[derive(Debug)]
struct PartyMemberMeta
{
	member: usize,
	modifiers: StatModifiers,
}

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],
	active: Vec<Option<PartyMemberMeta>>,
	side: u8,
	modifiers_default: StatModifiers,
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
			modifiers_default: StatModifiers::new(),
		};

		let mut current = Wrapping(usize::max_value());
		for _ in 0..party.active.capacity()
		{
			current = Wrapping(party.next_alive((current + Wrapping(1usize)).0));
			if current.0 == party.members.len()
			{
				break;
			}
			party.active.push(Some(PartyMemberMeta
			{
				member: current.0,
				modifiers: StatModifiers::new(),
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
	// pub fn member(&self, index: usize) -> &Monster
	// {
	// 	&self.members[index]
	// }
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
	// pub fn active_member_o(&self, index: usize) -> Option<&Monster>
	// {
	// 	self.active[index].as_ref().map(|active_member| &self.members[active_member.member])
	// }
	pub fn active_member(&self, index: usize) -> Option<PartyMember>
	{
		self.active[index].as_ref().map(|active_member|
		{
			PartyMember
			{
				member: &self.members[active_member.member],
				modifiers: &active_member.modifiers,
			}
		})
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
		self.active[active] = Some(PartyMemberMeta
		{
			member: target,
			modifiers: StatModifiers::new(),
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
	pub fn active_member_modifiers(&self, index: usize) -> &StatModifiers
	{
		&self.active[index].as_ref().unwrap().modifiers
	}
	pub fn active_stage_modifiers_mut(&mut self, index: usize) -> &mut StatModifiers
	{
		&mut self.active[index].as_mut().unwrap().modifiers
	}
}
