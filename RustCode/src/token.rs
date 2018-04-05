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

	pub fn get_token_type(&self) -> &TokenType {
		&self.token_type
	}

	pub fn get_lexeme(&self) -> &String {
		&self.lexeme
	}

	pub fn get_line(&self) -> usize {
		self.line
	}

	pub fn get_column(&self) -> usize {
		self.column
	}
}