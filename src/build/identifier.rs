//! Creates and stores valid code-compatible identifiers.
use std::rc::Rc;

/// Stores a valid code-compatible identifier.
pub struct Identifier
{
	regular: Rc<String>,
	capitalized: Rc<String>,
}

impl Identifier
{
	pub fn from_name(name: Rc<String>, internal: Option<String>) -> Option<Self>
	{
		if Identifier::valid(&name)
		{
			return Some(Identifier
			{
				regular: name.clone(),
				capitalized: Rc::new(name.to_uppercase()),
			});
		}
		else if let Some(other) = internal
		{
			if Identifier::valid(&other)
			{
				return Some(Identifier
				{
					regular: Rc::new(other.clone()),
					capitalized: Rc::new(other.to_uppercase()),
				});
			}
		}
		None
	}
	
	pub fn identifier(&self) -> &String
	{
		&self.regular
	}
	
	pub fn identifier_capitalized(&self) -> &String
	{
		&self.capitalized
	}
	
	fn valid(identifier: &String) -> bool
	{
		let mut iter = identifier.chars();
		if let Some(c) = iter.next()
		{
			if c.is_alphabetic()
			{
				return identifier.chars().all(|c| (c.is_alphanumeric() || c == '_'));
			}
		}
		false
	}
}