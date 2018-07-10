// Copyright 2018 Joshua Minter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::CompileError;
use std::fmt::{Display, Formatter, Error};
use std::str::Chars;
use std::iter::Peekable;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
	Number(f64),
	Name(String),
	Symbol(String)
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&Token::Number(value) => write!(f, "number {}", value),
			&Token::Name(ref value) => write!(f, "name '{}'", value),
			&Token::Symbol(ref value) => write!(f, "symbol '{}'", value)
		}
	}
}

pub struct Lexer<'a> {
	input: Peekable<Chars<'a>>,
	peeked: Option<Result<Token, CompileError>>
}

impl<'a> Lexer<'a> {
	pub fn new(input: Chars) -> Lexer {
		Lexer {
			input: input.peekable(),
			peeked: None
		}
	}

	fn read_symbol(&mut self) -> Token {
		let c = self.input.next().unwrap();
		let mut symbol = c.to_string();
		if c == '<' {
			if let Some(&c) = self.input.peek() {
				if "-=>".contains(c) {
					symbol.push(c);
				}
			}
		} else if c == '>' {
			if let Some(&c) = self.input.peek() {
				if c == '=' {
					symbol.push(c);
				}
			}
		}
		Token::Symbol(symbol)
	}

	pub fn peek(&mut self) -> Option<Result<Token, CompileError>> {
		if self.peeked == None {
			while self.input.peek()?.is_whitespace() {
				self.input.next();
			}
			let c = *self.input.peek()?;

			self.peeked = Some(if "[],.+-*/=<>&|()?:#".contains(c) {
				Ok(self.read_symbol())
			} else {
				Err(CompileError::UnknownToken(Token::Symbol(c.to_string())))
			});
		}
		self.peeked.clone()
	}

	pub fn peek_result(&mut self) -> Result<Token, CompileError> {
		self.peek().unwrap_or(Err(CompileError::UnexpectedEnd))
	}

	pub fn next_result(&mut self) -> Result<Token, CompileError> {
		let result = self.peek_result();
		self.peeked = None;
		result
	}
}

impl<'a> Iterator for Lexer {
	type Item = Result<Token, CompileError>;
	fn next(&mut self) -> Option<Result<Token, CompileError>> {
		let option = self.peek();
		self.peeked = None;
		option
	}
}