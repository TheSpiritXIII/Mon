use std::io::stdin;

pub fn clear()
{
	print!("\x1b[2J\x1b[1;1H")
}

pub fn input_range(max: usize) -> usize
{
	let error = "Invalid input. Please try again:";
	let mut buffer = String::new();
	loop
	{
		if let Ok(_) = stdin().read_line(&mut buffer)
		{
			let number = buffer[..buffer.len() - 1].parse::<usize>();
			match number
			{
				Ok(n) =>
				{
					if n > 0 && n <= max
					{
						return n;
					}
					println!("{}", error);
				}
				Err(_) =>
				{
					println!("Error: {:?}; {}", buffer.len(), error);
				}
			}
			buffer.clear();
		}
	}
}

pub fn wait()
{
	println!("Press [ENTER] to continue.");
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).unwrap();
}
