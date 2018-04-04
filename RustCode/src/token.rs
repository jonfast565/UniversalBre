use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
	line: usize,
	column: usize,
	token_type: TokenType,
	lexeme: String
}

impl Token {
	pub fn init(line: usize, column: usize, token_type: TokenType, lexeme: String) -> Token {
		Token {
            line: line,
            column: column,
            token_type: token_type,
            lexeme: lexeme
        }
	}
}