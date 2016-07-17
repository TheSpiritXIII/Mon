use base::types::monster::StatType;
use base::statmod::StatModifiers;

#[derive(Debug)]
pub enum Effect
{
	Damage(Damage),
	Switch(Switch),
	Modifier(Modifier),
	// Status(StatusId),
	// Ability(AbilityId),
	// Miss,
	// ,
	None(NoneReason),
}

#[derive(Debug)]
pub struct Damage
{
	pub party: usize, //
	pub active: usize,
	pub member: usize, //
	pub meta: DamageMeta, //
}

impl Damage
{
	pub fn amount(&self) -> StatType
	{
		self.meta.amount
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn member(&self) -> usize
	{
		self.member
	}
	pub fn critical(&self) -> bool
	{
		self.meta.critical
	}
	pub fn type_bonus(&self) -> f32
	{
		self.meta.type_bonus
	}
}

#[derive(Debug)]
pub struct DamageMeta
{
	pub amount: StatType, //
	pub type_bonus: f32, //
	pub critical: bool, //
}

#[derive(Debug)]
pub struct Switch
{
	pub member: usize,
	pub target: usize,
}

#[derive(Debug)]
pub struct Modifier
{
	party: usize,
	active: usize,
	modifiers: StatModifiers,
}

impl Modifier
{
	pub fn new(party: usize, active: usize, modifiers: StatModifiers) -> Self
	{
		Modifier
		{
			party: party,
			active: active,
			modifiers: modifiers
		}
	}
	pub fn party(&self) -> usize
	{
		self.party
	}
	pub fn active(&self) -> usize
	{
		self.active
	}
	pub fn modifiers(&self) -> &StatModifiers
	{
		&self.modifiers
	}
}

#[derive(Debug)]
pub enum NoneReason
{
	Miss,
	Escape,
}
