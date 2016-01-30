//! Contains wrapper types that change the way collections are displayed.
use std::fmt::{Display, Formatter, Error};
use std::collections::{HashMap};
use std::hash::{Hash};
use std::ascii::AsciiExt;

pub fn format_null_terminated(s: &str) -> String
{
	format!("b\"{}\\0\"", s.chars().map(|c|
	{
		if !c.is_ascii()
		{
			let mut bytes = [0; 4];
			let mut escaped = String::new();
			let size = c.encode_utf8(&mut bytes).unwrap();
			for i in 0..size
			{
				escaped.push_str(&format!("\\x{:0width$x}", bytes[i], width = 2));
			}
			return escaped;
		}
		format!("{}", c)
	})
	.collect::<String>())
}

/// Displays each pair of a HashMap as `Element[i] = Key[i]`, separated by a comma and space.
pub struct DisplayEqualsHashMap<U, V>(pub HashMap<U, V>);

impl<U, V> Display for DisplayEqualsHashMap<U, V> where U: Display + Eq + Hash, V: Display
{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error>
	{
		for (k, v) in &self.0
		{
			try!(write!(f, "{} = {}, ", k, v));
		}
		Ok(())
	}
}

/// Displays each value of a Vec separated by a comma and space.
pub struct DisplayVec<T>(pub Vec<T>);

impl<T> Display for DisplayVec<T> where T: Display
{
	fn fmt(&self, f:&mut Formatter) -> Result<(), Error>
	{
		for i in &self.0
		{
			try!(write!(f, "{}, ", i));
		}
		Ok(())
	}
}
