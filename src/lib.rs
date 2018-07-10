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

mod lexer;
pub use lexer::Token;

use std::fmt::{Display, Formatter, Error};

#[derive(Clone, PartialEq, Debug)]
pub enum CompileError {
	UnknownToken(Token),
	UnexpectedToken(Token),
	UnexpectedEnd
}

impl Display for CompileError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&CompileError::UnknownToken(ref token) => write!(f, "Unknown {}", token),
			&CompileError::UnexpectedToken(ref token) => write!(f, "Unexpected {}", token),
			&CompileError::UnexpectedEnd => write!("Unexpected end")
		}
	}
}