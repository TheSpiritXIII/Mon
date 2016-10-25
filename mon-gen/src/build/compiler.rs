use std::str::CharIndices;
use std::iter::Peekable;

/// Compiles source and returns the compiled source string if it is possible.
///
/// Internally, this uses a very simple lexer and a recursive descent parser. The parsing and lexing
/// is done simultaneously so this function is very fast. The purpose of this is to auto-insert
/// function arguments for all identifiers and closures. It accepts the following regular grammar:
///
/// N := A (+ A)*
/// A := identifier
/// A := identifier ( B )
/// B := C (, C)*
/// C := number
/// C := N
///
/// Note that the tokenizer is incredibly lax in what it considers to be an identifier or number, so
/// those errors will be translated into the compiled code.
pub fn compile(source: &str) -> Option<String>
{
	#[derive(Debug, PartialEq, Eq, Clone, Copy)]
	enum Token
	{
		Identifier,
		Variable,
		ParanL,
		ParanR,
		Digit,
		Comma,
		Plus,
		End,
		Unknown,
	}

	#[derive(Debug, PartialEq, Eq, Clone, Copy)]
	struct TokenRange<'a>
	{
		token: Token,
		source: &'a str,
		begin: usize,
		end: usize,
	}

	impl<'a> TokenRange<'a>
	{
		fn new(token: Token, source: &'a str, begin: usize, end: usize) -> Self
		{
			TokenRange
			{
				token: token,
				source: source,
				begin: begin,
				end: end,
			}
		}

		fn token(&self) -> Token
		{
			self.token
		}

		fn as_str(&self) -> &'a str
		{
			unsafe { self.source.slice_unchecked(self.begin, self.end) }
		}

		fn as_error(&self, error: String)
		{
			let mut last_newline = 0;
			let mut split_index = self.source.len();
			for (index, character) in self.source.char_indices()
			{
				if character == '\n'
				{
					if index >= self.begin
					{
						split_index = index;
						break;
					}
					last_newline = index + 1;
				}
			}

			let (before, after) = self.source.split_at(split_index);
			println!("");
			println!("{}", before);


			let spaces = (0..(self.begin - last_newline)).map(|_| " ").collect::<String>();
			let lines = (0..(self.end - self.begin)).map(|_| "-").collect::<String>();
			println!("{}^{}", spaces, lines);

			println!("{}{}", spaces, error);
			println!("{}Found symbol `{:?}`.", spaces, self.token);
			println!("{}", after);
		}
	}

	struct Lexer<'a>
	{
		source: &'a str,
		chars: Peekable<CharIndices<'a>>,
	}

	impl<'a> Lexer<'a>
	{
		fn from_str(source: &'a str) -> Self
		{
			Lexer
			{
				source: source,
				chars: source.char_indices().peekable(),
			}
		}

		fn eof(&self) -> TokenRange<'a>
		{
			TokenRange::new(Token::End, self.source, self.source.len(), self.source.len())
		}

		fn take_alphanumeric(&mut self) -> Option<(Token, usize, usize)>
		{
			if let Some(&(start, character)) = self.chars.peek()
			{
				if character.is_alphabetic()
				{
					self.chars.next();
					let end;

					let mut token = Token::Identifier;
					loop
					{
						if let Some(&(index, character)) = self.chars.peek()
						{
							if character.is_alphanumeric() || character == '_' || character == '.'
							{
								if character == '.'
								{
									token = Token::Variable;
								}
								self.chars.next();
							}
							else if character == ':'
							{
								self.chars.next();
								if let Some(&(_, character)) = self.chars.peek()
								{
									if character != ':'
									{
										return None;
									}
									else
									{
										self.chars.next();
									}
								}
							}
							else
							{
								end = index;
								break;
							}
						}
						else
						{
							end = self.source.len();
							return Some((token, start, end));
						}
					}
					Some((token, start, end))
				}
				else
				{
					None
				}
			}
			else
			{
				None
			}
		}

		fn take_number(&mut self) -> Option<(usize, usize)>
		{
			if let Some(&(start, character)) = self.chars.peek()
			{
				if character.is_digit(10) || character == '-'
				{
					self.chars.next();
					let mut end = start;
					while let Some(&(index, character)) = self.chars.peek()
					{
						if character.is_digit(10) || character == '_'
						{
							self.chars.next();
						}
						else
						{
							end = index;
							break;
						}
					}
					Some((start, end))
				}
				else
				{
					None
				}
			}
			else
			{
				None
			}
		}
	}

	impl<'a> Iterator for Lexer<'a>
	{
		type Item = TokenRange<'a>;

		fn next(&mut self) -> Option<Self::Item>
		{
			while let Some(&(_, character)) = self.chars.peek()
			{
				if character.is_whitespace()
				{
					self.chars.next();
				}
				else
				{
					break;
				}
			}

			if let Some((token, begin, end)) = self.take_alphanumeric()
			{
				Some(TokenRange::new(token, self.source, begin, end))
			}
			else if let Some((begin, end)) = self.take_number()
			{
				Some(TokenRange::new(Token::Digit, self.source, begin, end))
			}
			else if let Some((index, character)) = self.chars.next()
			{
				let token = match character
				{
					'(' => Token::ParanL,
					')' => Token::ParanR,
					',' => Token::Comma,
					'+' => Token::Plus,
					_ => Token::Unknown
				};
				Some(TokenRange::new(token, self.source, index, index))
			}
			else
			{
				None
			}
		}
	}

	type ParseError<'a> = Result<(), (String, Option<TokenRange<'a>>)>;

	fn expect<'a, F>(lexer: &mut Peekable<Lexer<'a>>, expected: Token, mut action: F)
		-> ParseError<'a> where F: FnMut(&str)
	{
		if let Some(token) = lexer.next()
		{
			if token.token() == expected
			{
				action(token.as_str());
				Ok(())
			}
			else
			{
				let message = format!("Expected symbol `{:?}`.", expected);
				Err((message, Some(token)))
			}
		}
		else
		{
			let message = format!("Expected symbol `{:?}`.", expected);
			Err((message, None))
		}
	}

	const ADDITIONAL_ARGS: &'static str = "effects, command, party, state, rng";

	fn identifier<'a>(lexer: &mut Peekable<Lexer<'a>>, result: &mut String, inner: bool)
		-> ParseError<'a>
	{
		expect(lexer, Token::Identifier, |ident| result.push_str(ident))?;

		result.push('(');
		result.push_str(ADDITIONAL_ARGS);

		if let Some(&token) = lexer.peek()
		{
			if token.token() == Token::ParanL
			{
				result.push_str(", ");
				lexer.next();

				args(lexer, result)?;
				expect(lexer, Token::ParanR, |_| () )?;
			}
		}

		result.push(')');

		if let Some(&token) = lexer.peek()
		{
			if token.token() == Token::Plus
			{
				lexer.next();
				result.push_str("; ");
				identifier(lexer, result, inner)?;
			}
			else if !inner
			{
				let error_message = "Expected token '+'";
				return Err((error_message.to_string(), Some(token)));
			}
			Ok(())
		}
		else
		{
			Ok(())
		}
	}

	fn args<'a>(lexer: &mut Peekable<Lexer<'a>>, result: &mut String) -> ParseError<'a>
	{
		if let Some(&token) = lexer.peek()
		{
			println!("Got token {:?}", token.token());
			match token.token()
			{
				Token::Identifier =>
				{
					result.push('|');
					result.push_str(ADDITIONAL_ARGS);
					result.push_str("| {");
					identifier(lexer, result, true)?;
					result.push('}');
				}
				Token::Digit =>
				{
					result.push_str(token.as_str());
					lexer.next();
				}
				Token::Variable =>
				{
					result.push_str(token.as_str().replace(".", "::").as_str());
					lexer.next();
				}
				Token::ParanR =>
				{
					return Ok(());
				}
				_ =>
				{
					let error_message = "Expected argument (digit, closure or end paranthesis).";
					return Err((error_message.to_string(), Some(token)));
				}
			}

			if let Some(&token) = lexer.peek()
			{
				if token.token() == Token::Comma
				{
					result.push_str(", ");
					lexer.next();
					args(lexer, result)?;
				}
				else if token.token() != Token::ParanR
				{
					let error_message = "Expected comma or end paranthesis.";
					return Err((error_message.to_string(), Some(token)));
				}
			}
		}
		Ok(())
	}

	let mut result = String::with_capacity(source.len() + ADDITIONAL_ARGS.len());
	let lexer = Lexer::from_str(source);
	let eof = lexer.eof();
	
	let err =
	{
		identifier(&mut lexer.peekable(), &mut result, false)
	};
	match err
	{
		Ok(()) =>
		{
			Some(result)
		}
		Err((message, token)) =>
		{
			match token
			{
				Some(token) =>
				{
					token.as_error(message);
				}
				None =>
				{
					eof.as_error(message);
				}
			}

			None
		}
	}
}
