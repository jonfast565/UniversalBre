use token::Token;
use token_type::TokenType;
use atom_status::AtomStatus;
use log;

#[derive(Debug, Clone)]
pub struct ScanError {
    location: usize,
    line: usize,
    column: usize,
    error_message: String
}

impl ScanError {
    fn init(location: usize, line: usize, column: usize, error_message: String) -> ScanError {
        ScanError {
            location: location,
            line: line,
            column: column,
            error_message: error_message
        }
    }

    pub fn get_error_message(&self) -> String {
        return self.error_message.clone();
    }
}

#[derive(Debug, Clone)]
struct ScanState {
    location: usize,
    line: usize,
    column: usize,
    input: Vec<char>
}

impl ScanState {
    fn init(input: String) -> ScanState {
        ScanState {
            location: 0,
            line: 0,
            column: 0,
            input: ScanState::input_to_char_vector(input)
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

    fn get_location(&self) -> usize {
        self.location
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
            return '\0'
        }
        
        // log::log_debug(&format!("Current location is '{}'", self.location));
        // log::log_debug(&format!("Current char is '{}'", self.input[self.location]));
        
        self.input[self.location]
    }

    fn char_at_index(&self, index: usize) -> char {
        if index >= self.input.len() {
            return '\0'
        }
        self.input[index]
    }

    fn char_at_offset(&self, offset: usize) -> char {
        if self.location + offset >= self.input.len() {
            return '\0'
        }
        self.input[self.location + offset]
    }

    fn get_atom_at_index(&self, location: usize) -> AtomStatus {
        AtomStatus::init(self.char_at_index(location))
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
            return
        }

        let mut temp_ctr : usize = 0;
        loop
        {
            let c = self.get_atom_at_offset(temp_ctr);
            if c.is_whitespace() {
                temp_ctr += 1;
                if c.is_newline_char() {
                    self.increment_line();
                    self.zero_column();
                }
            }
            else {
                break;
            }
        }
        self.increment_location(temp_ctr);
    }

    fn get_scan_error_details(&self, message: String) -> ScanError {
        // TODO: Implement as constructor method
        ScanError {
            location: self.location,
            line: self.line,
            column: self.column,
            error_message: message
        }
    }

    fn get_token(&self, lexeme: String, token_type: TokenType) -> Token {
        Token::init(self.line, self.column, token_type, lexeme)
    }

    // scan helpers

    fn push_increment_scan(&mut self, result: &mut String, increment_counter: &mut usize, char_atom: &AtomStatus) {
        result.push(char_atom.get_atom());
        self.increment_location(1);
        *increment_counter += 1;
    }

    fn scan_single_char_operator(&mut self, operator_char: char, token_type: TokenType) -> Result<Token, ScanError> {
        if self.char_at() != operator_char {
            return Err(self.get_scan_error_details(
                format!("{} not scanned", operator_char).to_string()))
        }
        self.increment_location(1);
        Ok(self.get_token(
            format!("{}", operator_char).to_string(), token_type))
    }

    fn scan_sequence(&mut self, keyword_id: &str, token_type: TokenType) -> Result<Token, ScanError>  {
        let keyword_chars : Vec<char> = keyword_id.chars().collect();
        let mut increment_counter = 0;
        let mut result = String::new();

        for keyword_char in &keyword_chars {
            let lowercase_char = keyword_char.to_lowercase().next().unwrap();

            // log::log_debug(&format!("Sequence char is '{}'", lowercase_char));
            // log::log_debug(&format!("Current char is '{}'", self.char_at()));

            if self.char_at() != lowercase_char {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                format!("{} not scanned", keyword_id).to_string()))
            }

            let mut current_atom = self.get_atom();
            self.push_increment_scan(&mut result, &mut increment_counter, &current_atom);
        }

        let last_char = self.get_atom();
        if !last_char.breaks_any() {
            return Err(self.get_scan_error_details(
                format!("{} not scanned, id candidate", keyword_id).to_string()))
        }

        Ok(self.get_token(
            format!("{}", keyword_id).to_string(), token_type))
    }

    // scan methods

    fn scan_function_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("fn", TokenType::FunctionKeyword)
    }

    // loops

    fn scan_infinite_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("infinite", TokenType::InfiniteKeyword)
    }

    fn scan_break_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("break", TokenType::BreakKeyword)
    }

    // language features
    
    fn scan_feature_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("feature", TokenType::FeatureKeyword)
    }

    fn scan_autobreak_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("autobreak", TokenType::AutobreakKeyword)
    }

    fn scan_on_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("on", TokenType::OnKeyword)
    }

    fn scan_off_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("off", TokenType::OffKeyword)
    }

    // brackets

    fn scan_begin_scope_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('{', TokenType::ScopeBeginOperator)
    }

    fn scan_end_scope_operator(&mut self) -> Result<Token, ScanError>  {
        self.scan_single_char_operator('}', TokenType::ScopeEndOperator)
    }

    // literals

    fn scan_integer_literal(&mut self) -> Result<Token, ScanError> {
        let mut result = String::new();
        let mut first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_digit() {
            // must 'lock in' the first character
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "digit").to_string()))
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any_integer() {
                break
            } 
            else if !next_char.is_digit() {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("{} not scanned", "integer digit").to_string()))
            }
            else {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
        }

        Ok(self.get_token(result, TokenType::IntegerLiteral))
    }

    fn scan_string_literal(&mut self) -> Result<Token, ScanError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.breaks_any_string() {
            // must 'lock in' the first quote mark
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "string delimiter").to_string()))
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any_string() {
                break
            }
            else if self.out_of_range() {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                    format!("{} runs off of code file", "string").to_string()))
            } 
            else {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
        }

        let next_char = self.get_atom();
        result.push(next_char.get_atom());
        self.increment_location(1);

        Ok(self.get_token(result, TokenType::StringLiteral))
    }

    fn scan_identifier(&mut self) -> Result<Token, ScanError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_identifier_char() {
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "identifier").to_string()))
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);

        loop {
            let next_char = self.get_atom();
            if next_char.breaks_any() {
                break
            } 
            else if next_char.is_identifier_char() {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
            else {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                format!("unrecognized char {}", self.char_at()).to_string()))
            }
        }

        Ok(self.get_token(result, TokenType::Identifier))
    }

    fn scan_float_literal(&mut self) -> Result<Token, ScanError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = 0;

        if !first_char.is_digit() {
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "floating-point number").to_string()))
        }

        self.push_increment_scan(&mut result, &mut increment_counter, &first_char);
        let mut precision_part = false;

        // TODO: Collect scale and precision in both of these sections
        loop {
            let next_char = self.get_atom();
            if next_char.is_digit() {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
            }
            else if next_char.is_dot() && precision_part == false {
                self.push_increment_scan(&mut result, &mut increment_counter, &next_char);
                precision_part = true;
            }
            else if next_char.breaks_any_integer() {
                break
            } 
            else {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                format!("unrecognized char {}", self.char_at()).to_string()))
            }
        }

        Ok(self.get_token(result, TokenType::FloatLiteral))
    }

    // boolean equality operators 

    fn scan_boolean_eq_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("==", TokenType::BooleanEqOperator)
    }

    fn scan_boolean_ne_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("!=", TokenType::BooleanNeOperator)
    }

    // boolean and/or operators

    fn scan_boolean_and_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("&&", TokenType::BooleanAndOperator)
    }

    fn scan_boolean_or_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("||", TokenType::BooleanOrOperator)
    }

    // boolean comparison operators

    fn scan_boolean_gt_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('>', TokenType::BooleanGtOperator)
    }

    fn scan_boolean_lt_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('<', TokenType::BooleanLtOperator)
    }

    fn scan_boolean_gte_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence(">=", TokenType::BooleanGteOperator)
    }

    fn scan_boolean_lte_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("<=", TokenType::BooleanLteOperator)
    }

    // operators

    fn scan_plus_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('+', TokenType::PlusOperator)
    }

    fn scan_minus_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('-', TokenType::MinusOperator)
    }

    fn scan_multiply_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('*', TokenType::MultiplyOperator)
    }

    fn scan_divide_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('/', TokenType::DivideOperator)
    }

    fn scan_concat_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('~', TokenType::ConcatOperator)
    }

    fn scan_assignment_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('=', TokenType::AssignmentOperator)
    }

    // parenthesis

    fn scan_left_parenthesis(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('(', TokenType::LeftParenthesis)
    }

    fn scan_right_parenthesis(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator(')', TokenType::RightParenthesis)
    }

    // program delimiters

    fn scan_semicolon(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator(';', TokenType::Semicolon)
    }

    fn scan_list_delimiter(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator(',', TokenType::ListDelimiter)
    }

    fn scan_type_specifier(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator(':', TokenType::TypeSpecifier)
    }

    // file delimiters

    fn scan_end_of_file(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('\0', TokenType::EndOfFile)
    }

    // scan delimiters
    // TODO: Is this method needed? can it be moved to the top?
    fn out_of_range(&self) -> bool {
        self.location >= self.input.len()
    }
}

pub struct Scanner {
    state: ScanState
}

impl Scanner {
    
    pub fn init (input: String) -> Scanner {
        Scanner {
            state: ScanState::init(input)
        }
    }

    fn scan_one(&mut self) -> Result<Token, ScanError> {
        // skip the whitespace

        self.state.skip_whitespace();
        
        // end of file

        if let Ok(token) = self.state.scan_end_of_file() {
            return Ok(token)
        }

        // numeric literals

        if let Ok(token) = self.state.scan_integer_literal() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_float_literal() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_string_literal() {
            return Ok(token)
        }

        // scoping operators

        if let Ok(token) = self.state.scan_begin_scope_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_end_scope_operator() {
            return Ok(token)
        }

        // mathematical operators

        if let Ok(token) = self.state.scan_plus_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_minus_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_multiply_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_divide_operator() {
            return Ok(token)
        }

        // string operators

        if let Ok(token) = self.state.scan_concat_operator() {
            return Ok(token)
        }

        // assingment operators

        if let Ok(token) = self.state.scan_assignment_operator() {
            return Ok(token)
        }

        // grouping operators

        if let Ok(token) = self.state.scan_left_parenthesis() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_right_parenthesis() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_semicolon() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_list_delimiter() {
            return Ok(token)
        }

        // boolean operators

        if let Ok(token) = self.state.scan_boolean_eq_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_ne_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_and_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_or_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_gt_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_lt_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_gte_operator() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_boolean_lte_operator() {
            return Ok(token)
        }

        // language keywords

        if let Ok(token) = self.state.scan_function_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_infinite_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_break_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_feature_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_autobreak_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_on_keyword() {
            return Ok(token)
        }

        if let Ok(token) = self.state.scan_off_keyword() {
            return Ok(token)
        }

        // language type keywords



        // identifier

        if let Ok(token) = self.state.scan_identifier() {
            return Ok(token)
        }

        let error_message = format!("Unrecognized character '{}'\nstarting at line {} column {}", 
            self.state.char_at(), self.state.get_line(), self.state.get_column());

        Err(ScanError::init(self.state.location, self.state.line, self.state.column, error_message))
    }

    pub fn scan_all(&mut self) -> Result<Vec<Token>, ScanError> {
        let mut tokens : Vec<Token> = Vec::new();

        loop {
            let new_token = self.scan_one();
            match new_token {
                Err(scan_error) => return Err(scan_error),
                Ok(scanned_token) => {
                    if *scanned_token.get_token_type() == TokenType::EndOfFile {
                        break
                    } else {
                        tokens.push(scanned_token)
                    }
                }
            }
        }

        return Ok(tokens);
    }
}
