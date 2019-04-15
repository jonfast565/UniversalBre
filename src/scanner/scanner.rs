use scanner::atom_status::AtomStatus;
use scanner::token::{Token, TokenType};
use utilities::error::CompileError;

macro_rules! seq_fn {
    ($seq_fn:ident, $context_var:ident, $sequence:expr, $enum:expr) => {
        fn $seq_fn (&mut $context_var) -> Result<Token, CompileError> {
            $context_var.scan_sequence($sequence, $enum)
        }
    };
}

macro_rules! op_fn {
    ($op_fn:ident, $context_var:ident, $op:expr, $enum:expr) => {
        fn $op_fn (&mut $context_var) -> Result<Token, CompileError> {
            $context_var.scan_single_char_operator($op, $enum)
        }
    };
}

macro_rules! scan_result {
    ($fn:ident, $context_var:ident) => {
        if let Ok(token) = $context_var.state.$fn() {
            return Ok(token);
        }
    };
}

#[derive(Debug, Clone)]
struct ScanState {
    location: usize,
    line: usize,
    column: usize,
    input: Vec<char>,
}

impl ScanState {
    fn init(input: String) -> ScanState {
        ScanState {
            location: 0,
            line: 0,
            column: 0,
            input: ScanState::input_to_char_vector(input),
        }
    }

    fn input_to_char_vector(input: String) -> Vec<char> {
        let stopped_input = input + "\0";
        return stopped_input.chars().collect();
    }

    fn get_line(&self) -> usize {
        self.line + 1
    }

    fn get_column(&self) -> usize {
        self.column + 1
    }

    fn increment_location(&mut self, increment: usize) {
        self.location += increment;
        self.column += increment; // TODO: This is wrong?
    }

    fn decrement_location(&mut self, decrement: usize) {
        self.location -= decrement;
        self.column -= decrement;
    }

    // utilities

    fn char_at(&self) -> char {
        if self.out_of_range() {
            return '\0';
        }

        // TODO: Enable code for debugging, but make it switchable
        // log::log_debug(&format!("Current location is '{}'", self.location));
        // log::log_debug(&format!("Current char is '{}'", self.input[self.location]));

        self.input[self.location]
    }

    fn char_at_index(&self, index: usize) -> char {
        if index >= self.input.len() {
            return '\0';
        }
        self.input[index]
    }

    fn char_at_offset(&self, offset: usize) -> char {
        if self.location + offset >= self.input.len() {
            return '\0';
        }
        self.input[self.location + offset]
    }

    fn get_atom_at_offset(&self, offset: usize) -> AtomStatus {
        AtomStatus::init(self.char_at_offset(offset))
    }

    fn get_atom(&self) -> AtomStatus {
        AtomStatus::init(self.char_at_index(self.location))
    }

    fn increment_line(&mut self) {
        self.line += 1;
    }

    fn zero_column(&mut self) {
        self.column = 0;
    }

    fn skip_whitespace(&mut self) {
        // range check is critical if we're
        // moving over array bounds indiscriminantly
        if self.out_of_range() {
            return;
        }

        let mut temp_ctr: usize = 0;
        loop {
            let c = self.get_atom_at_offset(temp_ctr);
            if c.is_whitespace() {
                temp_ctr += 1;
                if c.is_newline_char() {
                    self.increment_line();
                    self.zero_column();
                }
            } else {
                break;
            }
        }
        self.increment_location(temp_ctr);
    }

    fn get_scan_error_details(&self, message: String) -> CompileError {
        CompileError::init(self.location, self.line, self.column, message)
    }

    fn get_token(&self, lexeme: String, token_type: TokenType) -> Token {
        Token::init(self.line, self.column, token_type, lexeme)
    }

    // scan helpers

    fn push_increment_scan(
        &mut self,
        result: &mut String,
        increment_counter: &mut usize,
        char_atom: &AtomStatus,
    ) {
        result.push(char_atom.get_atom());
        self.increment_location(1);
        *increment_counter += 1;
    }

    fn scan_single_char_operator(
        &mut self,
        operator_char: char,
        token_type: TokenType,
    ) -> Result<Token, CompileError> {
        if self.char_at() != operator_char {
            return Err(
                self.get_scan_error_details(format!("{} not scanned", operator_char).to_string())
            );
        }
        self.increment_location(1);
        Ok(self.get_token(format!("{}", operator_char).to_string(), token_type))
    }

    fn scan_sequence(
        &mut self,
        keyword_id: &str,
        token_type: TokenType,
    ) -> Result<Token, CompileError> {
        let keyword_chars: Vec<char> = keyword_id.chars().collect();
        let mut increment_counter = 0;
        let mut result = String::new();

        for keyword_char in &keyword_chars {
            let lowercase_char = keyword_char.to_lowercase().next().unwrap();

            // log::log_debug(&format!("Sequence char is '{}'", lowercase_char));
            // log::log_debug(&format!("Current char is '{}'", self.char_at()));

            if self.char_at() != lowercase_char {
                self.decrement_location(increment_counter);
                return Err(
                    self.get_scan_error_details(format!("{} not scanned", keyword_id).to_string())
                );
            }

            let mut current_atom = self.get_atom();
            self.push_increment_scan(&mut result, &mut increment_counter, &current_atom);
        }

        let last_char = self.get_atom();
        if !last_char.breaks_any() {
            return Err(self.get_scan_error_details(
                format!("{} not scanned, id candidate", keyword_id).to_string(),
            ));
        }

        Ok(self.get_token(format!("{}", keyword_id).to_string(), token_type))
    }

    // literals
    fn scan_integer_literal(&mut self) -> Result<Token, CompileError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_digit() {
            // must 'lock in' the first character
            return Err(self.get_scan_error_details(format!("{} not scanned", "digit").to_string()));
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any_integer() {
                break;
            } else if !next_char.is_digit() {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("{} not scanned", "integer digit").to_string(),
                ));
            } else {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
        }

        Ok(self.get_token(result, TokenType::IntegerLiteral))
    }

    fn scan_string_literal(&mut self) -> Result<Token, CompileError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.breaks_any_string() {
            // must 'lock in' the first quote mark
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "string delimiter").to_string(),
            ));
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any_string() {
                break;
            } else if self.out_of_range() {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("{} runs off of code file", "string").to_string(),
                ));
            } else {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
        }

        let next_char = self.get_atom();
        result.push(next_char.get_atom());
        self.increment_location(1);

        Ok(self.get_token(result, TokenType::StringLiteral))
    }

    fn scan_identifier(&mut self) -> Result<Token, CompileError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_identifier_char() {
            return Err(
                self.get_scan_error_details(format!("{} not scanned", "identifier").to_string())
            );
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any() {
                break;
            } else if next_char.is_identifier_char() {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            } else {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("unrecognized char {}", self.char_at()).to_string(),
                ));
            }
        }

        Ok(self.get_token(result, TokenType::Identifier))
    }

    fn scan_float_literal(&mut self) -> Result<Token, CompileError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_digit() {
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "floating-point number").to_string(),
            ));
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);
        let mut precision_part = false;

        // TODO: Collect scale and precision in both of these sections
        loop {
            let next_char = self.get_atom();
            if next_char.is_digit() {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            } else if next_char.is_dot() && precision_part == false {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
                precision_part = true;
            } else if next_char.breaks_any_integer() {
                break;
            } else {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("unrecognized char {}", self.char_at()).to_string(),
                ));
            }
        }

        Ok(self.get_token(result, TokenType::FloatLiteral))
    }

    // other structured literals
    seq_fn!(
        scan_boolean_true_literal,
        self,
        "true",
        TokenType::BooleanTrueLiteral
    );
    seq_fn!(
        scan_boolean_false_literal,
        self,
        "false",
        TokenType::BooleanFalseLiteral
    );

    // sequences
    seq_fn!(
        scan_function_keyword,
        self,
        "fn",
        TokenType::FunctionKeyword
    );
    // loop keywords
    seq_fn!(
        scan_infinite_keyword,
        self,
        "infinite",
        TokenType::InfiniteKeyword
    );
    seq_fn!(scan_break_keyword, self, "break", TokenType::BreakKeyword);
    // feature keywords
    seq_fn!(
        scan_feature_keyword,
        self,
        "feature",
        TokenType::FeatureKeyword
    );
    seq_fn!(
        scan_autobreak_keyword,
        self,
        "autobreak",
        TokenType::AutobreakKeyword
    );
    seq_fn!(
        scan_return_keyword,
        self,
        "return",
        TokenType::ReturnKeyword
    );
    seq_fn!(scan_on_keyword, self, "on", TokenType::OnKeyword);
    seq_fn!(scan_off_keyword, self, "off", TokenType::OffKeyword);
    // boolean equality operators
    seq_fn!(
        scan_boolean_eq_operator,
        self,
        "==",
        TokenType::BooleanEqOperator
    );
    seq_fn!(
        scan_boolean_ne_operator,
        self,
        "!=",
        TokenType::BooleanNeOperator
    );
    // scoping operators
    op_fn!(
        scan_scope_begin_operator,
        self,
        '{',
        TokenType::ScopeBeginOperator
    );
    op_fn!(
        scan_scope_end_operator,
        self,
        '}',
        TokenType::ScopeEndOperator
    );
    // boolean and/or operators
    seq_fn!(
        scan_boolean_and_operator,
        self,
        "&&",
        TokenType::BooleanAndOperator
    );
    seq_fn!(
        scan_boolean_or_operator,
        self,
        "||",
        TokenType::BooleanOrOperator
    );
    // boolean comparison operators
    op_fn!(
        scan_boolean_gt_operator,
        self,
        '>',
        TokenType::BooleanGtOperator
    );
    op_fn!(
        scan_boolean_lt_operator,
        self,
        '<',
        TokenType::BooleanLtOperator
    );
    seq_fn!(
        scan_boolean_gte_operator,
        self,
        ">=",
        TokenType::BooleanGteOperator
    );
    seq_fn!(
        scan_boolean_lte_operator,
        self,
        "<=",
        TokenType::BooleanLteOperator
    );
    // mathematical operators
    op_fn!(scan_plus_operator, self, '+', TokenType::PlusOperator);
    op_fn!(scan_minus_operator, self, '-', TokenType::MinusOperator);
    op_fn!(
        scan_multiply_operator,
        self,
        '*',
        TokenType::MultiplyOperator
    );
    op_fn!(scan_divide_operator, self, '/', TokenType::DivideOperator);
    op_fn!(scan_concat_operator, self, '~', TokenType::ConcatOperator);
    op_fn!(
        scan_assignment_operator,
        self,
        '=',
        TokenType::AssignmentOperator
    );
    // parenthesis / indexers
    op_fn!(scan_left_parenthesis, self, '(', TokenType::LeftParenthesis);
    op_fn!(
        scan_right_parenthesis,
        self,
        ')',
        TokenType::RightParenthesis
    );
    op_fn!(scan_left_indexer, self, '[', TokenType::LeftIndexer);
    op_fn!(scan_right_indexer, self, ']', TokenType::RightIndexer);
    // program delimiters
    op_fn!(scan_semicolon, self, ';', TokenType::Semicolon);
    op_fn!(scan_list_delimiter, self, ',', TokenType::ListDelimiter);
    op_fn!(scan_type_specifier, self, ':', TokenType::TypeSpecifier);
    // special
    op_fn!(scan_end_of_file, self, '\0', TokenType::EndOfFile);

    // TODO: Is this method needed? can it be moved to the top?
    fn out_of_range(&self) -> bool {
        self.location >= self.input.len()
    }
}

pub struct Scanner {
    state: ScanState,
}

impl Scanner {
    pub fn init(input: String) -> Scanner {
        Scanner {
            state: ScanState::init(input),
        }
    }

    fn scan_one(&mut self) -> Result<Token, CompileError> {
        // skip the whitespace
        self.state.skip_whitespace();

        // end of file
        scan_result!(scan_end_of_file, self);
        // literals
        scan_result!(scan_integer_literal, self);
        scan_result!(scan_float_literal, self);
        scan_result!(scan_string_literal, self);
        scan_result!(scan_boolean_true_literal, self);
        scan_result!(scan_boolean_false_literal, self);
        // scoping operators
        scan_result!(scan_scope_begin_operator, self);
        scan_result!(scan_scope_end_operator, self);
        // mathematical operators
        scan_result!(scan_plus_operator, self);
        scan_result!(scan_minus_operator, self);
        scan_result!(scan_multiply_operator, self);
        scan_result!(scan_divide_operator, self);
        // string operators
        scan_result!(scan_concat_operator, self);
        // assignment operators
        scan_result!(scan_assignment_operator, self);
        // grouping operators
        scan_result!(scan_left_parenthesis, self);
        scan_result!(scan_right_parenthesis, self);
        scan_result!(scan_left_indexer, self);
        scan_result!(scan_right_indexer, self);
        scan_result!(scan_semicolon, self);
        scan_result!(scan_list_delimiter, self);
        scan_result!(scan_type_specifier, self);
        // boolean operators
        scan_result!(scan_boolean_eq_operator, self);
        scan_result!(scan_boolean_ne_operator, self);
        scan_result!(scan_boolean_and_operator, self);
        scan_result!(scan_boolean_or_operator, self);
        scan_result!(scan_boolean_gt_operator, self);
        scan_result!(scan_boolean_lt_operator, self);
        scan_result!(scan_boolean_gte_operator, self);
        scan_result!(scan_boolean_lte_operator, self);
        // language keywords
        scan_result!(scan_function_keyword, self);
        scan_result!(scan_infinite_keyword, self);
        scan_result!(scan_break_keyword, self);
        scan_result!(scan_feature_keyword, self);
        scan_result!(scan_autobreak_keyword, self);
        scan_result!(scan_return_keyword, self);
        scan_result!(scan_on_keyword, self);
        scan_result!(scan_off_keyword, self);
        // identifier: must come afterwards
        scan_result!(scan_identifier, self);

        let error_message = format!(
            "Unrecognized character '{}'\nstarting at line {} column {}",
            self.state.char_at(),
            self.state.get_line(),
            self.state.get_column()
        );

        Err(CompileError::init(
            self.state.location,
            self.state.line,
            self.state.column,
            error_message,
        ))
    }

    pub fn scan_all(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let new_token = self.scan_one();
            match new_token {
                Err(scan_error) => return Err(scan_error),
                Ok(scanned_token) => {
                    if scanned_token.get_token_type() == TokenType::EndOfFile {
                        break;
                    } else {
                        tokens.push(scanned_token)
                    }
                }
            }
        }

        return Ok(tokens);
    }
}
