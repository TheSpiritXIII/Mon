use base::monster::Monster;
use base::types::monster::StatType;
use base::types::attack::AccuracyType;

use std::slice;
use std::collections::{HashMap, HashSet};

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
	pub fn modifiers(&self) -> &'a StatModifiers
	{
		self.modifiers
	}
}

#[derive(Debug, Clone)]
struct PartyMemberMeta
{
	member: usize,
	modifiers: StatModifiers,
	// exposed: HashSet<(usize, usize)>,
	exposed_new: HashMap<usize, HashSet<usize>>,
}

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],
	active: Vec<PartyMemberMeta>,
	side: u8,
	modifiers_default: StatModifiers,
	gain_experience: bool,

	// The number of party members still alive (has health greater than or equal to 0) exluding
	/// active.
	alive: usize,

	// The number of party members waiting to be switched out. UNUSED RIGHT NOW.
	switch_waiting: usize,

	// The indices of party members to map back to original order.
	reference_order: Vec<usize>,

	// TODO: Add vec item_locked: bool,
}

impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], side: u8, out: usize) -> Self
	{
		let mut reference_order = Vec::with_capacity(members.len());
		for i in 0..members.len() {
			reference_order.push(i);
		}
		let mut party = Party
		{
			members: members,
			active: Vec::with_capacity(out),
			side: side,
			modifiers_default: Default::default(),
			gain_experience: true,
			alive: 0,
			switch_waiting: 0,
			reference_order: reference_order,
		};

		for member_index in 0..party.members.len()
		{
			if party.members[member_index].get_health() != 0
			{
				if party.active.len() != party.active.capacity()
				{
					party.active.push(PartyMemberMeta
					{
						member: member_index,
						modifiers: Default::default(),
						exposed_new: HashMap::new(),
					});
				}
				else
				{
					party.alive += 1;
				}
			}
		}
		party
	}
	pub fn side(&self) -> u8
	{
		self.side
	}
	pub fn member(&self, index: usize) -> &Monster
	{
		&self.members[index]
	}
	pub fn expose_add(&mut self, party: &Party, index: usize)
	{
		for active_self in &mut self.active
		{
			let exposed_set = active_self.exposed_new.entry(index).or_insert_with(HashSet::new);
			for active_other in &party.active
			{
				exposed_set.insert( party.reference_order[active_other.member]);
			}
		}
	}
	pub fn expose_add_member(&mut self, index: usize, member: usize)
	{
		for active_self in &mut self.active
		{
			let exposed_set = active_self.exposed_new.entry(index).or_insert_with(HashSet::new);
			exposed_set.insert(member); 
		}
	}
	pub fn expose_add_active(&mut self, party: &Party, index: usize, active: usize)
	{
		self.expose_add_member(index, party.reference_order[party.active[active].member]);
	}
	pub fn expose_reference(&mut self, active: usize) -> usize
	{
		self.reference_order[self.active[active].member]
	}
	pub fn expose_clear_all(&mut self)
	{
		for active in &mut self.active
		{
			active.exposed_new.clear();
		}
	}
	pub fn expose_gain_experience(&self)
	{

	}
	pub fn member_count(&self) -> usize
	{
		self.members.len()
	}
	pub fn active_purge(&mut self)
	{
		// Decrease the number of active members if there is no one to take their place.
		if self.switch_waiting >
		 self.alive
		{
			let mut lol = self.active.clone();
			lol.retain(|ref mut active_member|
			{
				self.members[active_member.member].get_health() != 0
			});
			self.active = lol;
			self.switch_waiting = 0;
		}
	}
	pub fn member_is_active(&self, index: usize) -> bool
	{
		// self.active.iter().any(|active_member_option|
		// {
		// 	if let Some(ref active_member) = *active_member_option
		// 	{
		// 		active_member.member == index
		// 	}
		// 	else
		// 	{
		// 		false
		// 	}
		// })
		self.active.iter().any(|active_member|
		{
			active_member.member == index
		})
	}
	pub fn switch_active(&mut self, member: usize, target: usize)
	{
		// TODO: Allow this when member is already active.
		self.members.swap(self.active[member].member, target);
		self.reference_order.swap(self.active[member].member, target);
		if self.switch_waiting > 0
		{
			self.switch_waiting -= 1;
			self.alive -= 1;
		}
	}
	pub fn switch_waiting(&self) -> Option<usize>
	{
		for i in 0..self.active.len()
		{
			if self.members[self.active[i].member].get_health() == 0
			{
				return Some(i);
			}
		}
		None
	}
	pub fn active_waiting(&self) -> bool
	{
		self.switch_waiting <= self.alive
	}
	pub fn active_member(&self, index: usize) -> PartyMember
	{
		PartyMember
		{
			member: &self.members[self.active[index].member],
			modifiers: &self.active[index].modifiers,
		}
	}
	pub fn active_member_alive(&self, index: usize) -> Option<PartyMember>
	{
		let member = self.active_member(index).member;
		if member.get_health() != 0
		{
			Some(self.active_member(index))
		}
		else
		{
			None
		}
		//self.active[index].as_ref().map_or(false, |member| self.members[member.member].get_health() != 0)
	}
	pub fn active_are_alive(&self) -> bool
	{
		self.active.iter().any(|active| self.members[active.member].get_health() != 0)
	}
	pub fn active_member_index(&self, index: usize) -> usize
	{
		// self.active[index].as_ref().map(|active_member| active_member.member)
		self.active[index].member
	}
	pub fn active_count(&self) -> usize
	{
		self.active.len()
	}
	// pub fn active_set(&mut self, active: usize, target: usize)
	// {
	// 	// TODO: Should be allowed when active already set?
	// 	self.active[active] = PartyMemberMeta
	// 	{
	// 		member: target,
	// 		modifiers: Default::default(),
	// 	};
	// }
	pub fn iter(&self) -> slice::Iter<Monster>
	{
		self.members.iter()
	}
	pub fn active_member_modifiers(&self, index: usize) -> &StatModifiers
	{
		// &self.active[index].as_ref().unwrap().modifiers
		&self.active[index].modifiers
	}
	pub fn active_member_modifiers_add(&mut self, index: usize, modifiers: &StatModifiers)
	{
		// self.active[index].as_mut().unwrap().modifiers.apply(modifiers);
		self.active[index].modifiers.apply(modifiers);
	}
	pub fn active_member_lose_health(&mut self, member: usize, amount: u16) -> bool
	{
		let target = self.members.get_mut(self.active[member].member).unwrap();
		target.lose_health(amount);
		if target.get_health() == 0
		{
			self.switch_waiting += 1;
			true
		}
		else
		{
			false
		}
	}
	pub fn active_member_attack_limit_take(&mut self, member: usize, attack: usize)
	{
		let target = self.members.get_mut(member).unwrap();
		target.get_attacks_mut()[attack].limit_left_take(1);
	}
	pub fn member_waiting_count(&self) -> usize
	{
		self.alive
	}
}
