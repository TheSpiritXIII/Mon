use base::monster::Monster;
use base::types::monster::StatType;
use base::types::attack::AccuracyType;

use std::slice;

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

#[derive(Debug)]
struct PartyMemberMeta
{
	member: usize,
	modifiers: StatModifiers,
	// TODO: Reward lineup.
}

#[derive(Debug)]
pub struct Party<'a>
{
	members: &'a mut [Monster],
	active: Vec<Option<PartyMemberMeta>>,
	side: u8,
	modifiers_default: StatModifiers,
	gain_experience: bool,

	// The number of party members still alive (has health greater than or equal to 0) exluding
	/// active.
	alive: usize,

	// The number of party members waiting to be switched out. UNUSED RIGHT NOW.
	switch_waiting: usize,

	// TODO: Cache count of post-turn switch waiting members here maybe?
	// TODO: Add for experience gaining: gain_experience: bool
	// TODO: Add vec item_locked: bool,
}

impl<'a> Party<'a>
{
	pub fn new(members: &'a mut [Monster], side: u8, out: usize) -> Self
	{
		let mut party = Party
		{
			members: members,
			active: Vec::with_capacity(out),
			side: side,
			modifiers_default: Default::default(),
			gain_experience: true,
			alive: 0,
			switch_waiting: 0,
		};

		for member_index in 0..party.members.len()
		{
			if party.members[member_index].get_health() != 0
			{
				if party.active.len() != party.active.capacity()
				{
					party.active.push(Some(PartyMemberMeta
					{
						member: member_index,
						modifiers: Default::default(),
					}));
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
	pub fn member_count(&self) -> usize
	{
		self.members.len()
	}
	pub fn active_remove(&mut self, index: usize)
	{
		// TODO: I don't like this function. Purging should be done entirely automatically here instead.
		self.active.remove(index);
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
		self.active.iter().position(|member| member.as_ref().map_or(true, |mm| self.members[mm.member].get_health() == 0))
	}
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
	pub fn active_member_alive(&self, index: usize) -> Option<PartyMember>
	{
		self.active_member(index).and_then(|member|
		{
			if member.member.get_health() != 0
			{
				Some(member)
			}
			else
			{
				None
			}
		})
		//self.active[index].as_ref().map_or(false, |member| self.members[member.member].get_health() != 0)
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
			modifiers: Default::default(),
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
	pub fn active_member_modifiers_add(&mut self, index: usize, modifiers: &StatModifiers)
	{
		self.active[index].as_mut().unwrap().modifiers.apply(modifiers);
	}
	pub fn active_member_lose_health(&mut self, member: usize, amount: u16) -> bool
	{
		let target = self.members.get_mut(member).unwrap();
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
