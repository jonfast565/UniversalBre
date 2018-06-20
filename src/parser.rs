use error::CompileError;
use log;
use semantic_blocks::{ArgumentBlock, DataType, ExprNode, FunctionBlock, LoopBlock, LoopType,
                      OperationType, Program, SemanticBlock, StatementBlock};
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

	fn eat_lookahead(&mut self, token_type: TokenType) -> Option<CompileError> {
		let lookahead = self.get_lookahead();
		self.eat_token(lookahead, token_type)
	}

	fn get_token(&self) -> Token {
		self.tokens[self.location].clone()
	}

	fn get_compile_error(&self, message: String) -> CompileError {
		let error_token = self.get_token();
		CompileError::init(0, error_token.get_line(), error_token.get_column(), message)
	}

	fn get_static_compile_error(&self, message: &'static str) -> CompileError {
		self.get_compile_error(message.to_string())
	}

	fn eat_token(&mut self, actual: TokenType, expected: TokenType) -> Option<CompileError> {
		let compile_error: CompileError;
		if actual == expected {
			let debug_message = format!("{:?} found", expected);
			log::log_debug(&debug_message);
			return None
		} else {
			let error_message = format!("Expected {:?} but got {:?}", expected, actual);
			compile_error = self.get_compile_error(error_message);
		}
		self.location += 1;
		Some(compile_error)
	}

	pub fn parse(&mut self) -> Result<Program, CompileError> {
		log::log_debug("Parse program");
		let mut semantic_blocks = Vec::<SemanticBlock>::new();
		let mut matched = true;
		while self.get_lookahead() != TokenType::EndOfFile {
			match self.get_lookahead() {
				TokenType::Identifier => {
					let assignment_statement = match self.parse_assignment_statement() {
						Ok(assignment_statement_body) => assignment_statement_body,
						Err(contents) => return Err(contents)
					};
					semantic_blocks.push(SemanticBlock::init_with_statement(assignment_statement));
				}
				TokenType::InfiniteKeyword => {
					let infinite_loop = match self.parse_infinite_loop() {
						Ok(infinite_loop_body) => infinite_loop_body,
						Err(contents) => return Err(contents)
					};
					semantic_blocks.push(SemanticBlock::init_with_loop(infinite_loop));
				}
				TokenType::FunctionKeyword => {
					let function_block = match self.parse_function_block() {
						Ok(function_block) => function_block,
						Err(contents) => return Err(contents)
					};
					semantic_blocks.push(SemanticBlock::init_with_function(function_block));
				}
				_ => {
					matched = false;
				}
			}

			if !matched {
				return Err(self.get_static_compile_error(
					"Cannot start with this statement type",
				));
			}
		}

		self.eat_lookahead(TokenType::EndOfFile);
		Ok(Program::init(semantic_blocks))
	}

	fn parse_assignment_statement(&mut self) -> Result<StatementBlock, CompileError> {
		log::log_debug("Parse assignment statement");
		let id = if self.get_lookahead() == TokenType::Identifier {
			self.get_token().get_lexeme()
		} else {
			String::new() // TODO: What?
		};
		self.eat_lookahead(TokenType::Identifier);

		let expression = match self.parse_expression() {
			Ok(expression_contents) => expression_contents,
			Err(contents) => return Err(contents)
		};
		self.eat_lookahead(TokenType::Semicolon);

		Ok(StatementBlock::init_with_assignment(id, expression))
	}

	fn parse_infinite_loop(&mut self) -> Result<LoopBlock, CompileError> {
		log::log_debug("Parse infinite loop");
		self.eat_lookahead(TokenType::Semicolon);
		Ok(LoopBlock::init(LoopType::InfiniteLoop))
	}

	fn parse_argument_list(&mut self) -> Result<Vec<ArgumentBlock>, CompileError> {
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
		Ok(argument_list)
	}

	fn parse_function_block(&mut self) -> Result<FunctionBlock, CompileError> {
		log::log_debug("Parse function");

		self.eat_lookahead(TokenType::FunctionKeyword);
		let id = if self.get_lookahead() == TokenType::Identifier {
			self.get_token().get_lexeme()
		} else {
			String::new() // TODO: Still... what?
		};

		self.eat_lookahead(TokenType::Identifier);
		let argument_list = match self.parse_argument_list() {
			Ok(argument_list_body) => argument_list_body,
			Err(contents) => return Err(contents)
		};

		let mut function_body = Vec::<SemanticBlock>::new();
		self.eat_lookahead(TokenType::ScopeBeginOperator);
		while self.get_lookahead() != TokenType::ScopeEndOperator {
			match self.parse_assignment_statement() {
				Ok(result_statement_body) => { 
					function_body.push(SemanticBlock::init_with_statement(result_statement_body)) 
				},
				Err(contents) => {
					return Err(contents)
				}
			};
		}
		self.eat_lookahead(TokenType::ScopeEndOperator);

		Ok(FunctionBlock::init(id, argument_list, function_body))
	}

	fn parse_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse expression");
		self.parse_boolean_or_expression()
	}

	fn parse_boolean_or_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse boolean-or subexpr");
		let mut left_node = match self.parse_boolean_and_expression() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::BooleanOrOperator {
			match self.get_lookahead() {
				TokenType::BooleanOrOperator => {
					self.eat_lookahead(TokenType::BooleanOrOperator);
				}
				_ => break,
			}
			let right_node = match self.parse_boolean_and_expression() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};
			left_node =
				ExprNode::init_as_binary(left_node, right_node, OperationType::BooleanOrOperation);
		}
		Ok(left_node)
	}

	fn parse_boolean_and_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse boolean-and subexpr");
		let mut left_node = match self.parse_boolean_comparison_expression() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::BooleanAndOperator {
			match self.get_lookahead() {
				TokenType::BooleanAndOperator => {
					self.eat_lookahead(TokenType::BooleanAndOperator);
				}
				_ => break,
			}
			let right_node = match self.parse_boolean_comparison_expression() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};
			left_node =
				ExprNode::init_as_binary(left_node, right_node, OperationType::BooleanAndOperation);
		}
		Ok(left_node)
	}

	fn parse_boolean_comparison_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse boolean comparison subexpr");
		let mut left_node = match self.parse_boolean_equality_expression() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::BooleanGtOperator
			|| self.get_lookahead() == TokenType::BooleanGteOperator
			|| self.get_lookahead() == TokenType::BooleanLtOperator
			|| self.get_lookahead() == TokenType::BooleanLteOperator
		{
			let operation_type: OperationType;
			match self.get_lookahead() {
				TokenType::BooleanGtOperator => {
					self.eat_lookahead(TokenType::BooleanGtOperator);
					operation_type = OperationType::BooleanGtOperation;
				}
				TokenType::BooleanGteOperator => {
					self.eat_lookahead(TokenType::BooleanGteOperator);
					operation_type = OperationType::BooleanGteOperation;
				}
				TokenType::BooleanLtOperator => {
					self.eat_lookahead(TokenType::BooleanLtOperator);
					operation_type = OperationType::BooleanLtOperation;
				}
				TokenType::BooleanLteOperator => {
					self.eat_lookahead(TokenType::BooleanLteOperator);
					operation_type = OperationType::BooleanLteOperation;
				}
				_ => break,
			}
			let right_node = match self.parse_boolean_equality_expression() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};
			left_node = ExprNode::init_as_binary(left_node, right_node, operation_type);
		}
		Ok(left_node)
	}

	fn parse_boolean_equality_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse boolean equality subexpr");
		let mut left_node = match self.parse_concat_expression() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::BooleanEqOperator
			|| self.get_lookahead() == TokenType::BooleanNeOperator
		{
			let operation_type: OperationType;
			match self.get_lookahead() {
				TokenType::BooleanEqOperator => {
					self.eat_lookahead(TokenType::BooleanEqOperator);
					operation_type = OperationType::BooleanEqOperation;
				}
				TokenType::BooleanNeOperator => {
					self.eat_lookahead(TokenType::BooleanNeOperator);
					operation_type = OperationType::BooleanNeOperation;
				}
				_ => break,
			}
			let right_node = match self.parse_concat_expression() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};
			left_node = ExprNode::init_as_binary(left_node, right_node, operation_type);
		}
		Ok(left_node)
	}

	fn parse_concat_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse concat subexpr");
		let mut left_node = match self.parse_mathematical_expression() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::ConcatOperator {
			match self.get_lookahead() {
				TokenType::ConcatOperator => {
					self.eat_lookahead(TokenType::ConcatOperator);
				}
				_ => break,
			}
			let right_node = match self.parse_mathematical_expression() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};

			left_node =
				ExprNode::init_as_binary(left_node, right_node, OperationType::ConcatOperation);
		}
		Ok(left_node)
	}

	fn parse_mathematical_expression(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse mathematical subexpr");

		let mut left_node = match self.parse_term() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::PlusOperator
			|| self.get_lookahead() == TokenType::MinusOperator
		{
			let operation_type: OperationType;
			match self.get_lookahead() {
				TokenType::PlusOperator => {
					self.eat_lookahead(TokenType::PlusOperator);
					operation_type = OperationType::AdditionOperation;
				}
				TokenType::MinusOperator => {
					self.eat_lookahead(TokenType::MinusOperator);
					operation_type = OperationType::SubtractionOperation;
				}
				_ => break,
			}

			let right_node = match self.parse_term() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};

			left_node = ExprNode::init_as_binary(left_node, right_node, operation_type);
		}
		Ok(left_node)
	}

	fn parse_term(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse term subexpr");
		let mut left_node = match self.parse_factor() {
			Ok(left_node_unwrap) => left_node_unwrap,
			Err(contents) => return Err(contents)
		};

		while self.get_lookahead() == TokenType::MultiplyOperator
			|| self.get_lookahead() == TokenType::DivideOperator
		{
			let operation_type: OperationType;
			match self.get_lookahead() {
				TokenType::MultiplyOperator => {
					self.eat_lookahead(TokenType::MultiplyOperator);
					operation_type = OperationType::MultiplicationOperation;
				}
				TokenType::DivideOperator => {
					self.eat_lookahead(TokenType::DivideOperator);
					operation_type = OperationType::DivisionOperation;
				}
				_ => break,
			}
			let right_node = match self.parse_factor() {
				Ok(right_node_unwrap) => right_node_unwrap,
				Err(contents) => return Err(contents)
			};
			left_node = ExprNode::init_as_binary(left_node, right_node, operation_type);
		}
		Ok(left_node)
	}

	fn parse_factor(&mut self) -> Result<ExprNode, CompileError> {
		log::log_debug("Parse factor subexpr");
		match self.get_lookahead() {
			TokenType::LeftParenthesis => {
				self.eat_lookahead(TokenType::LeftParenthesis);
				let inner_expression = self.parse_expression();
				self.eat_lookahead(TokenType::RightParenthesis);
				inner_expression
			}
			TokenType::IntegerLiteral => {
				let current_lexeme = self.get_token().get_lexeme();
				self.eat_lookahead(TokenType::IntegerLiteral);
				Ok(ExprNode::init_as_literal(
					current_lexeme,
					DataType::IntegerType,
				))
			}
			TokenType::FloatLiteral => {
				let current_lexeme = self.get_token().get_lexeme();
				self.eat_lookahead(TokenType::FloatLiteral);
				Ok(ExprNode::init_as_literal(
					current_lexeme,
					DataType::FloatType,
				))
			}
			TokenType::Identifier => {
				let current_lexeme = self.get_token().get_lexeme();
				self.eat_lookahead(TokenType::Identifier);
				Ok(ExprNode::init_as_variable(current_lexeme))
			}
			TokenType::StringLiteral => {
				let current_lexeme = self.get_token().get_lexeme();
				self.eat_lookahead(TokenType::StringLiteral);
				Ok(ExprNode::init_as_variable(current_lexeme))
			}
			TokenType::MinusOperator => {
				self.eat_lookahead(TokenType::MinusOperator);
				// TODO: Fix this shit, we're appending contextual information in the parser
				// as if it is scanning... dreadful.
				let current_lexeme = format!("-{}", self.get_token().get_lexeme());
				match self.get_lookahead() {
					TokenType::IntegerLiteral => {
						self.eat_lookahead(TokenType::IntegerLiteral);
						Ok(ExprNode::init_as_literal(
							current_lexeme,
							DataType::IntegerType,
						))
					}
					TokenType::FloatLiteral => {
						self.eat_lookahead(TokenType::FloatLiteral);
						Ok(ExprNode::init_as_literal(
							current_lexeme,
							DataType::FloatType,
						))
					}
					TokenType::Identifier => {
						self.eat_lookahead(TokenType::IntegerLiteral);
						Ok(ExprNode::init_as_variable(current_lexeme))
					}
					_ => Err(self.get_static_compile_error(
						"Negative subexpression not matched here",
					)),
				}
			}
			_ => Err(self.get_static_compile_error(
				"Subexpression did not start with id, left paren, or numeric constant",
			)),
		}
	}
}
