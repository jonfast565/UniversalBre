use error::CompileError;
use log;
use semantic_blocks::{ArgumentBlock, BinaryExpr, FunctionBlock, LoopBlock, LoopType, Program,
                      SemanticBlock, StatementBlock};
use token::{Token, TokenType};

pub struct Parser {
	location: usize,
	tokens: Vec<Token>,
}

impl Parser {
	pub fn init(tokens: Vec<Token>) -> Parser {
		Parser {
			location: 0,
			tokens: tokens,
		}
	}

	fn get_lookahead(&self) -> TokenType {
		self.tokens[self.location].get_token_type().clone()
	}

	fn eat_lookahead(&mut self, token_type: TokenType) {
		let lookahead = self.get_lookahead();
		self.eat_token(lookahead, token_type);
	}

	fn get_token(&self) -> Token {
		self.tokens[self.location].clone()
	}

	fn get_compile_error(&self, message: String) -> CompileError {
		let error_token = self.get_token();
		CompileError::init(0, error_token.get_line(), error_token.get_column(), message)
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

	pub fn parse(&mut self) -> Result<Program, CompileError> {
		log::log_debug("Parse program");
		let mut semantic_blocks = Vec::<SemanticBlock>::new();
		let mut matched = true;
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
				_ => {
					matched = false;
				}
			}

			if !matched {
				return Err(self.get_compile_error(
					"Cannot start with this statement type".to_string(),
				));
			}
		}

		self.eat_lookahead(TokenType::EndOfFile);
		Ok(Program::init(semantic_blocks))
	}

	fn parse_expression(&self) -> BinaryExpr {
		BinaryExpr {
			// expr
		}
	}

	fn parse_assignment_statement(&mut self) -> StatementBlock {
		log::log_debug("Parse assignment statement");
		let id = if self.get_lookahead() == TokenType::Identifier {
			self.get_token().get_lexeme()
		} else {
			"".to_string() // TODO: What?
		};
		self.eat_lookahead(TokenType::Identifier);

		let expression = self.parse_expression();
		self.eat_lookahead(TokenType::Semicolon);

		StatementBlock::init_with_assignment(id, expression)
	}

	fn parse_infinite_loop(&mut self) -> LoopBlock {
		log::log_debug("Parse infinite loop");
		self.eat_lookahead(TokenType::Semicolon);
		LoopBlock::init(LoopType::InfiniteLoop)
	}

	fn parse_argument_list(&mut self) -> Vec<ArgumentBlock> {
		let mut argument_list = Vec::<ArgumentBlock>::new();
		if self.get_lookahead() == TokenType::LeftParenthesis {
			self.eat_lookahead(TokenType::LeftParenthesis);
			while self.get_lookahead() == TokenType::Identifier {
				let id = self.get_token().get_lexeme();
				self.eat_lookahead(TokenType::Identifier);
				argument_list.push(ArgumentBlock::init(id));

				if self.get_lookahead() != TokenType::RightParenthesis {
					self.eat_lookahead(TokenType::ListDelimiter);
				}
			}
			self.eat_lookahead(TokenType::RightParenthesis);
		}
		argument_list
	}

	fn parse_function_block(&mut self) -> FunctionBlock {
		log::log_debug("Parse function");

		self.eat_lookahead(TokenType::FunctionKeyword);
		let id = if self.get_lookahead() == TokenType::Identifier {
			self.get_token().get_lexeme()
		} else {
			"".to_string() // TODO: What?
		};

		self.eat_lookahead(TokenType::Identifier);
		let argument_list = self.parse_argument_list();

		let mut function_body = Vec::<SemanticBlock>::new();
		self.eat_lookahead(TokenType::ScopeBeginOperator);
		while self.get_lookahead() != TokenType::ScopeEndOperator {
			let result_statement = self.parse_assignment_statement();
			function_body.push(SemanticBlock::init_with_statement(result_statement));
		}
		self.eat_lookahead(TokenType::ScopeEndOperator);

		FunctionBlock::init(id, argument_list, function_body)
	}
}
