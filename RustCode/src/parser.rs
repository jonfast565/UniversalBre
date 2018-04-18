// use operationtype;

use program::Program;
use token::Token;
use token_type::TokenType;
use statement::Statement;
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
		self.tokens[self.location].get_token_type().clone()
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

	pub fn parse(&mut self) -> Program {
		self.parse_program()
	}

	pub fn parse_program(&mut self) -> Program {
		log::log_debug("Parse program");
		let mut statements = Vec::<Statement>::new();

		while self.get_lookahead() != TokenType::EndOfFile {
			statements.push(self.parse_possible_statement())
		}

		let eof_lookahead = self.get_lookahead(); 
		self.eat_token(eof_lookahead, TokenType::EndOfFile);
		Program::init(statements)
	}

	pub fn parse_possible_statement(&mut self) -> Statement {
		match self.get_lookahead() {
			TokenType::Identifier => self.parse_assignment_statement(),
			TokenType::InfiniteKeyword => self.parse_infinite_loop(),
			_ => panic!("Unrecognized statement lookahead")
		}
	} 

	pub fn parse_assignment_statement(&mut self) -> Statement {
		Statement {}
	}

	pub fn parse_infinite_loop(&mut self) -> Statement {
		Statement {}
	}
}