use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
	line: usize,
	column: usize,
	token_type: TokenType,
	lexeme: String
}