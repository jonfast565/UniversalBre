// use operationtype;

use program::Program;
use token::Token;
use token_type::TokenType;

struct Parser { 
	location: usize,
	tokens: Vec<Token>
}

impl Parser {
	fn init(tokens: Vec<Token>) -> Parser {
		Parser {
			location: 0,
			tokens: tokens
		}
	}

	fn get_location(&self) -> usize {
		self.location
	}

	fn get_lookahead(&self) -> TokenType {
		self.tokens[self.location].get_token_type()
	}

	fn get_token(&self) -> Token {
		self.tokens[self.location].clone()
	}

	fn eat_token(&self, actual: TokenType, expected: TokenType) {
		
	}

	pub fn parse() -> Program {
		

		Program {}
	}
}