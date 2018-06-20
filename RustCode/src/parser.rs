use log;
use semantic_blocks::{FunctionBlock, LoopBlock, LoopType, Program, SemanticBlock, StatementBlock,
                      StatementType, ArgumentBlock};
use token::Token;
use token_type::TokenType;

struct Parser {
	location: usize,
	tokens: Vec<Token>,
}

impl Parser {
	fn init(tokens: Vec<Token>) -> Parser {
		Parser {
			location: 0,
			tokens: tokens,
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
		let mut semantic_blocks = Vec::<SemanticBlock>::new();

		while self.get_lookahead() != TokenType::EndOfFile {
			match self.get_lookahead() {
				TokenType::Identifier => {
					let assignment_statement = self.parse_assignment_statement();
					semantic_blocks.push(SemanticBlock::init_with_statement(assignment_statement));
				}
				TokenType::InfiniteKeyword => {
					let infinite_loop = self.parse_infinite_loop();
					semantic_blocks.push(SemanticBlock::init_with_loop(infinite_loop));
				}
				TokenType::FunctionKeyword => {
					let function_block = self.parse_function_block();
					semantic_blocks.push(SemanticBlock::init_with_function(function_block));
				}
				// TODO: This should be a very user-friendly error
				_ => panic!("Unrecognized statement lookahead, aborted"),
			}
		}

		let eof_lookahead = self.get_lookahead();
		self.eat_token(eof_lookahead, TokenType::EndOfFile);
		Program::init(semantic_blocks)
	}

	pub fn parse_assignment_statement(&mut self) -> StatementBlock {
		log::log_debug("Parse assignment statement");
		let lookahead = self.get_lookahead();
		self.eat_token(lookahead, TokenType::Semicolon);
		StatementBlock::init(StatementType::AssignmentStatement)
	}

	pub fn parse_infinite_loop(&mut self) -> LoopBlock {
		log::log_debug("Parse infinite loop");
		let lookahead = self.get_lookahead();
		self.eat_token(lookahead, TokenType::Semicolon);
		LoopBlock::init(LoopType::InfiniteLoop)
	}

	pub fn parse_function_block(&mut self) -> FunctionBlock {
		log::log_debug("Parse function");
		let argument_list = Vec::new();
		FunctionBlock::init(argument_list)
	}
}
