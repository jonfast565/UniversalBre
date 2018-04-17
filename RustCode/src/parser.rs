// use operationtype;

use program::Program;
use token::Token;
use token_type::TokenType;
use log;

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

	fn eat_token(&mut self, actual: TokenType, expected: TokenType) {
		if actual == expected {
			let debug_message = format!("{:?} found", expected);
			log::log_debug(&debug_message);
		} else {
			let error_message = format!("Expected {:?} but got {:?}", expected, actual); 
			panic!(error_message);
		}
		self.location += 1;
	}

	pub fn parse(&self) -> Program {
		self.parse_program()
	}

	pub fn parse_program(&self) -> Program {
		Program {}
	}

	pub fn parse_possible_statement(&self) {
		//let statements = Vec<Statement>{};
		
	} 
}