use std::str;

pub fn as_rust_str(string: &'static [u8]) -> &'static str
{
	unsafe
	{
		&str::from_utf8_unchecked(string)[0 .. string.len() -1]
	}
}

pub fn as_rust_str_from(string: &[u8]) -> &str
{
	unsafe
	{
		&str::from_utf8_unchecked(string)[0 .. string.len() -1]
	}
}
