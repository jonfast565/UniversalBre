use utilities::error::CompileError;
use utilities::log;
use utilities::utility;

use semantic_analysis::data_types::DataType;
use semantic_analysis::expressions::ExprNode;
use semantic_analysis::functions::{ArgumentBlock, FunctionBlock};
use semantic_analysis::loops::{LoopBlock, LoopType};
use semantic_analysis::operation_types::OperationType;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::SemanticBlock;
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};

use scanner::token::{Token, TokenType};

macro_rules! eat_token {
    ($self_var: expr, $e:expr) => {
        if let Some(error) = $self_var.eat_lookahead($e) {
            return Err(error);
        }
    };
}

macro_rules! parse_error {
    ($e:expr) => {
        match $e {
            Ok(result) => result,
            Err(contents) => return Err(contents),
        };
    };
}

macro_rules! get_identifier {
    ($context_var:ident) => {
        if $context_var.get_lookahead() == TokenType::Identifier {
            $context_var.get_token().get_lexeme()
        } else {
            panic!("Invalid lookahead");
        };
    };
}

fn or_many(many_ors: Vec<bool>) -> bool {
    let mut result = false;
    for or in many_ors {
        result = result || or;
    }
    return result;
}

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
        if self.location == self.tokens.len() {
            return TokenType::EndOfFile;
        }
        self.tokens[self.location].get_token_type().clone()
    }

    fn eat_lookahead(&mut self, token_type: TokenType) -> Option<CompileError> {
        let lookahead = self.get_lookahead();
        self.eat_token(lookahead, token_type)
    }

    fn get_token(&self) -> Token {
        if self.location == self.tokens.len() {
            return Token::init(0, 0, TokenType::EndOfFile, String::new());
        }
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
            self.location += 1;
            return None;
        } else {
            let error_message = format!("Expected {:?} but got {:?}", expected, actual);
            compile_error = self.get_compile_error(error_message);
            Some(compile_error)
        }
    }

    pub fn parse(&mut self) -> Result<Program, CompileError> {
        log::log_debug("Parse program");
        let mut program_blocks = Vec::<SemanticBlock>::new();
        while self.get_lookahead() != TokenType::EndOfFile {
            let semantic_block = self.parse_unit();
            match semantic_block {
                Ok(block) => program_blocks.push(block),
                Err(err) => return Err(err),
            }
        }
        eat_token!(self, TokenType::EndOfFile);
        Ok(Program::init(program_blocks))
    }

    fn parse_unit(&mut self) -> Result<SemanticBlock, CompileError> {
        match self.get_lookahead() {
            TokenType::Identifier => {
                let assignment_statement = parse_error!(self.parse_assignment_statement());
                Ok(SemanticBlock::init_assignment(assignment_statement))
            }
            TokenType::InfiniteKeyword => {
                let infinite_loop = parse_error!(self.parse_infinite_loop());
                Ok(SemanticBlock::init_loop(infinite_loop))
            }
            TokenType::BreakKeyword => {
                let breaker = parse_error!(self.parse_break_statement());
                Ok(SemanticBlock::init_break(breaker))
            }
            TokenType::ReturnKeyword => {
                let returner = parse_error!(self.parse_return_statement());
                Ok(SemanticBlock::init_return(returner))
            }
            TokenType::FunctionKeyword => {
                let function_block = parse_error!(self.parse_function_block());
                Ok(SemanticBlock::init_function(function_block))
            }
            _ => {
                return Err(self.get_compile_error(format!(
                    "Unit cannot begin with {}",
                    self.get_lookahead()
                )));
            }
        }
    }

    fn parse_assignment_statement(&mut self) -> Result<AssignmentBlock, CompileError> {
        log::log_debug("Parse assignment statement");
        let id = get_identifier!(self);

        eat_token!(self, TokenType::Identifier);
        eat_token!(self, TokenType::AssignmentOperator);

        let expression = parse_error!(self.parse_expression());

        eat_token!(self, TokenType::Semicolon);
        Ok(AssignmentBlock::init(id, expression))
    }

    fn parse_break_statement(&mut self) -> Result<BreakBlock, CompileError> {
        log::log_debug("Parse break statement");

        eat_token!(self, TokenType::BreakKeyword);
        eat_token!(self, TokenType::Semicolon);

        Ok(BreakBlock::init())
    }

    fn parse_return_statement(&mut self) -> Result<ReturnBlock, CompileError> {
        log::log_debug("Parse return statement");
        eat_token!(self, TokenType::ReturnKeyword);

        let expression = parse_error!(self.parse_expression());
        eat_token!(self, TokenType::Semicolon);

        Ok(ReturnBlock::init(expression))
    }

    fn parse_infinite_loop(&mut self) -> Result<LoopBlock, CompileError> {
        log::log_debug("Parse infinite loop");
        let mut loop_blocks = Vec::<SemanticBlock>::new();

        eat_token!(self, TokenType::InfiniteKeyword);
        eat_token!(self, TokenType::ScopeBeginOperator);

        while self.get_lookahead() != TokenType::ScopeEndOperator {
            let loop_block = self.parse_unit();
            match loop_block {
                Ok(block) => loop_blocks.push(block),
                Err(err) => return Err(err),
            }
        }

        eat_token!(self, TokenType::ScopeEndOperator);
        Ok(LoopBlock::init(LoopType::InfiniteLoop, loop_blocks))
    }

    fn parse_argument_list(&mut self) -> Result<Vec<ArgumentBlock>, CompileError> {
        let mut argument_list = Vec::<ArgumentBlock>::new();
        if self.get_lookahead() == TokenType::LeftParenthesis {
            eat_token!(self, TokenType::LeftParenthesis);
            while self.get_lookahead() == TokenType::Identifier {
                let id = self.get_token().get_lexeme();
                eat_token!(self, TokenType::Identifier);
                argument_list.push(ArgumentBlock::init(id));

                if self.get_lookahead() != TokenType::RightParenthesis {
                    eat_token!(self, TokenType::ListDelimiter);
                }
            }
            eat_token!(self, TokenType::RightParenthesis);
        }
        Ok(argument_list)
    }

    fn parse_function_block(&mut self) -> Result<FunctionBlock, CompileError> {
        log::log_debug("Parse function");
        eat_token!(self, TokenType::FunctionKeyword);
        let id = get_identifier!(self);

        eat_token!(self, TokenType::Identifier);
        let argument_list = parse_error!(self.parse_argument_list());
        let mut function_body = Vec::<SemanticBlock>::new();
        eat_token!(self, TokenType::ScopeBeginOperator);

        while self.get_lookahead() != TokenType::ScopeEndOperator {
            let result_statement_body = self.parse_unit();
            match result_statement_body {
                Ok(body_statement) => function_body.push(body_statement),
                Err(err) => return Err(err),
            }
        }

        eat_token!(self, TokenType::ScopeEndOperator);
        Ok(FunctionBlock::init(id, argument_list, function_body))
    }

    fn parse_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse expression");
        self.parse_boolean_or_expression()
    }

    fn parse_boolean_or_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse boolean-or subexpr");
        let mut left_node = parse_error!(self.parse_boolean_and_expression());

        while self.get_lookahead() == TokenType::BooleanOrOperator {
            match self.get_lookahead() {
                TokenType::BooleanOrOperator => {
                    eat_token!(self, TokenType::BooleanOrOperator);
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_boolean_and_expression());
            left_node =
                ExprNode::init_binary(left_node, right_node, OperationType::BooleanOrOperation);
        }
        Ok(left_node)
    }

    fn parse_boolean_and_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse boolean-and subexpr");
        let mut left_node = parse_error!(self.parse_boolean_comparison_expression());

        while self.get_lookahead() == TokenType::BooleanAndOperator {
            match self.get_lookahead() {
                TokenType::BooleanAndOperator => {
                    eat_token!(self, TokenType::BooleanAndOperator);
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_boolean_comparison_expression());
            left_node =
                ExprNode::init_binary(left_node, right_node, OperationType::BooleanAndOperation);
        }
        Ok(left_node)
    }

    fn parse_boolean_comparison_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse boolean comparison subexpr");
        let mut left_node = parse_error!(self.parse_boolean_equality_expression());

        while self.get_lookahead() == TokenType::BooleanGtOperator
            || self.get_lookahead() == TokenType::BooleanGteOperator
            || self.get_lookahead() == TokenType::BooleanLtOperator
            || self.get_lookahead() == TokenType::BooleanLteOperator
        {
            let operation_type: OperationType;
            match self.get_lookahead() {
                TokenType::BooleanGtOperator => {
                    eat_token!(self, TokenType::BooleanGtOperator);
                    operation_type = OperationType::BooleanGtOperation;
                }
                TokenType::BooleanGteOperator => {
                    eat_token!(self, TokenType::BooleanGteOperator);
                    operation_type = OperationType::BooleanGteOperation;
                }
                TokenType::BooleanLtOperator => {
                    eat_token!(self, TokenType::BooleanLtOperator);
                    operation_type = OperationType::BooleanLtOperation;
                }
                TokenType::BooleanLteOperator => {
                    eat_token!(self, TokenType::BooleanLteOperator);
                    operation_type = OperationType::BooleanLteOperation;
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_boolean_equality_expression());
            left_node = ExprNode::init_binary(left_node, right_node, operation_type);
        }
        Ok(left_node)
    }

    fn parse_boolean_equality_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse boolean equality subexpr");
        let mut left_node = parse_error!(self.parse_concat_expression());

        while self.get_lookahead() == TokenType::BooleanEqOperator
            || self.get_lookahead() == TokenType::BooleanNeOperator
        {
            let operation_type: OperationType;
            match self.get_lookahead() {
                TokenType::BooleanEqOperator => {
                    eat_token!(self, TokenType::BooleanEqOperator);
                    operation_type = OperationType::BooleanEqOperation;
                }
                TokenType::BooleanNeOperator => {
                    eat_token!(self, TokenType::BooleanNeOperator);
                    operation_type = OperationType::BooleanNeOperation;
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_concat_expression());
            left_node = ExprNode::init_binary(left_node, right_node, operation_type);
        }
        Ok(left_node)
    }

    fn parse_concat_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse concat subexpr");
        let mut left_node = parse_error!(self.parse_mathematical_expression());

        while self.get_lookahead() == TokenType::ConcatOperator {
            match self.get_lookahead() {
                TokenType::ConcatOperator => {
                    eat_token!(self, TokenType::ConcatOperator);
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_mathematical_expression());
            left_node =
                ExprNode::init_binary(left_node, right_node, OperationType::ConcatOperation);
        }
        Ok(left_node)
    }

    fn parse_mathematical_expression(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse mathematical subexpr");
        let mut left_node = parse_error!(self.parse_term());

        while self.get_lookahead() == TokenType::PlusOperator
            || self.get_lookahead() == TokenType::MinusOperator
        {
            let operation_type: OperationType;
            match self.get_lookahead() {
                TokenType::PlusOperator => {
                    eat_token!(self, TokenType::PlusOperator);
                    operation_type = OperationType::AdditionOperation;
                }
                TokenType::MinusOperator => {
                    eat_token!(self, TokenType::MinusOperator);
                    operation_type = OperationType::SubtractionOperation;
                }
                _ => break,
            }

            let right_node = parse_error!(self.parse_term());
            left_node = ExprNode::init_binary(left_node, right_node, operation_type);
        }
        Ok(left_node)
    }

    fn parse_term(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse term subexpr");
        let mut left_node = parse_error!(self.parse_factor());

        while self.get_lookahead() == TokenType::MultiplyOperator
            || self.get_lookahead() == TokenType::DivideOperator
        {
            let operation_type: OperationType;
            match self.get_lookahead() {
                TokenType::MultiplyOperator => {
                    eat_token!(self, TokenType::MultiplyOperator);
                    operation_type = OperationType::MultiplicationOperation;
                }
                TokenType::DivideOperator => {
                    eat_token!(self, TokenType::DivideOperator);
                    operation_type = OperationType::DivisionOperation;
                }
                _ => break,
            }
            let right_node = parse_error!(self.parse_factor());
            left_node = ExprNode::init_binary(left_node, right_node, operation_type);
        }
        Ok(left_node)
    }

    fn parse_nested_expression(&mut self) -> Result<ExprNode, CompileError> {
        eat_token!(self, TokenType::LeftParenthesis);
        let inner_expression = self.parse_expression();
        eat_token!(self, TokenType::RightParenthesis);
        inner_expression
    }

    fn parse_integer_literal(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::IntegerLiteral);
        Ok(ExprNode::init_literal(
            current_lexeme,
            DataType::IntegerType,
        ))
    }

    fn parse_float_literal(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::FloatLiteral);
        Ok(ExprNode::init_literal(current_lexeme, DataType::FloatType))
    }

    fn parse_identifier(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::Identifier);
        Ok(ExprNode::init_variable(current_lexeme))
    }

    fn parse_string_literal(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::StringLiteral);
        Ok(ExprNode::init_variable(current_lexeme))
    }

    fn parse_boolean_true_literal(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::BooleanTrueLiteral);
        Ok(ExprNode::init_literal(
            current_lexeme,
            DataType::BooleanType,
        ))
    }

    fn parse_boolean_false_literal(&mut self) -> Result<ExprNode, CompileError> {
        let current_lexeme = self.get_token().get_lexeme();
        eat_token!(self, TokenType::BooleanFalseLiteral);
        Ok(ExprNode::init_literal(
            current_lexeme,
            DataType::BooleanType,
        ))
    }

    fn parse_negated_expression(&mut self) -> Result<ExprNode, CompileError> {
        eat_token!(self, TokenType::MinusOperator);
        // TODO: Fix this shit, we're appending contextual information in the parser
        // as if it is scanning... dreadful.
        let current_lexeme = format!("-{}", self.get_token().get_lexeme());
        match self.get_lookahead() {
            TokenType::IntegerLiteral => self.parse_integer_literal(),
            TokenType::FloatLiteral => self.parse_float_literal(),
            TokenType::Identifier => self.parse_identifier(),
            _ => Err(self.get_static_compile_error("Negative subexpression not matched here")),
        }
    }

    fn parse_factor(&mut self) -> Result<ExprNode, CompileError> {
        log::log_debug("Parse factor subexpr");
        match self.get_lookahead() {
            TokenType::LeftParenthesis => self.parse_nested_expression(),
            TokenType::IntegerLiteral => self.parse_integer_literal(),
            TokenType::FloatLiteral => self.parse_float_literal(),
            TokenType::Identifier => self.parse_identifier(),
            TokenType::StringLiteral => self.parse_string_literal(),
            TokenType::BooleanTrueLiteral => self.parse_boolean_true_literal(),
            TokenType::BooleanFalseLiteral => self.parse_boolean_false_literal(),
            TokenType::MinusOperator => self.parse_negated_expression(),
            _ => Err(self.get_static_compile_error(
                "Subexpression needs id, left paren, or numeric constant",
            )),
        }
    }
}
