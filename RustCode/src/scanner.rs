use token::Token;
use token_type::TokenType;
use atom_status::AtomStatus;

#[derive(Debug, Clone)]
struct ScanError {
    location: usize,
    line: usize,
    column: usize,
    error_message: String
}

impl ScanError {
    
}

#[derive(Debug, Clone)]
struct ScanState {
    location: usize,
    line: usize,
    column: usize,
    input: Vec<char>,
    failed: bool,
    error: Option<ScanError>
}

impl ScanState {
    fn init(input: String) -> ScanState {
        ScanState {
            location: 0,
            line: 0,
            column: 0,
            input: ScanState::input_to_char_vector(input),
            failed: false,
            error: None
        }
    }

    fn input_to_char_vector(input: String) -> Vec<char> {
        return input.chars().collect();
    }

    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }

    fn get_location(&self) -> usize {
        self.location
    }

    fn increment_location(&mut self, increment: usize) {
        self.location += increment;
        self.column += increment; // TODO: This is wrong
    }

    // utilities

    fn char_at(&self) -> char {
        if self.location >= self.input.len() {
            return '\0'
        }
        self.input[self.location]
    }

    fn char_at_index(&self, index: usize) -> char {
        if index > self.input.len() {
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

    fn get_char_atom(&self, location: usize) -> AtomStatus {
        AtomStatus::init(self.char_at_index(location))
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

        let mut temp_ctr = 0;
        let mut c = self.get_char_atom(temp_ctr);

        if !c.is_whitespace() { 
            return
        }

        loop
        {
            c = self.get_char_atom(temp_ctr);
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

    fn scan_single_char_operator(&mut self, operator_char: char, token_type: TokenType) -> Result<Token, ScanError> {
        if self.char_at() != operator_char {
            return Err(self.get_scan_error_details(
                format!("{} not scanned", operator_char).to_string()))
        }
        self.increment_location(1);
        Ok(self.get_token(
            format!("{}", operator_char).to_string(), token_type))
    }

    // scan methods
    fn scan_function_keyword(&mut self) {

    }

    // loops
    // TODO: Implement, copied from the CPP

    fn scan_infinite_keyword(&mut self) {

    }

    fn scan_break_keyword(&mut self) {

    }

    // language features
    // TODO: Implement, copied from the CPP
    
    fn scan_feature_keyword(&mut self) {

    }

    fn scan_autobreak_keyword(&mut self) {

    }

    // brackets
    fn scan_begin_scope_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('{', TokenType::ScopeBeginOperator)
    }

    fn scan_end_scope_operator(&mut self) -> Result<Token, ScanError>  {
        self.scan_single_char_operator('{', TokenType::ScopeEndOperator)
    }

    // literals

    fn scan_integer_literal(&mut self) {

    }

    fn scan_string_literal(&mut self) {

    }

    fn scan_identifier(&mut self) {

    }

    fn scan_float_literal(&mut self) {

    }

    // boolean equality operators 

    fn scan_boolean_eq_operator(&mut self) {

    }

    fn scan_boolean_ne_operator(&mut self) {

    }

    // boolean and/or operators

    fn scan_boolean_and_operator(&mut self) {

    }

    fn scan_boolean_or_operator(&mut self) {

    }

    // boolean comparison operators

    fn scan_boolean_gt_operator(&mut self) {

    }

    fn scan_boolean_lt_operator(&mut self) {

    }

    fn scan_boolean_gte_operator(&mut self) {

    }

    fn scan_boolean_lte_operator(&mut self) {

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
        self.scan_single_char_operator(')', TokenType::ListDelimiter)
    }

    // file delimiters
    fn scan_end_of_file(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('\0', TokenType::EndOfFile)
    }

    // scan delimiters
    fn out_of_range(&self) -> bool {
        self.location >= self.input.len() - 1
    }
}

struct Scanner {

}

impl Scanner {

    fn scan_one() {

    }

    fn scan_all() {

    }
}
