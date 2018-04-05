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

    fn decrement_location(&mut self, decrement: usize) {
        self.location -= decrement;
        self.column -= decrement;
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

    fn get_atom_at_index(&self, location: usize) -> AtomStatus {
        AtomStatus::init(self.char_at_index(location))
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

        let mut temp_ctr : usize = self.location;
        let mut c : AtomStatus = self.get_atom_at_index(temp_ctr);

        if !c.is_whitespace() { 
            return
        }

        loop
        {
            c = self.get_atom_at_index(temp_ctr);
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

        for keyword_char in &keyword_chars {
            let lowercase_char = keyword_char.to_lowercase().next().unwrap();
            
            if self.char_at() != lowercase_char {
                self.decrement_location(increment_counter);
                return Err(self.get_scan_error_details(
                format!("{} not scanned", keyword_id).to_string()))
            }
            
            increment_counter += 1;
            self.increment_location(1);
        }

        Ok(self.get_token(
            format!("{}", keyword_id).to_string(), token_type))
    }

    // scan methods

    fn scan_function_keyword(&mut self) -> Result<Token, ScanError> {
        self.scan_sequence("function", TokenType::FunctionKeyword)
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

    // brackets

    fn scan_begin_scope_operator(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('{', TokenType::ScopeBeginOperator)
    }

    fn scan_end_scope_operator(&mut self) -> Result<Token, ScanError>  {
        self.scan_single_char_operator('{', TokenType::ScopeEndOperator)
    }

    // literals

    fn scan_integer_literal(&mut self) -> Result<Token, ScanError> {
        let mut result = String::new();
        let first_char = self.get_atom();
        let mut increment_counter = self.location;

        if !first_char.is_digit() {
            // must 'lock in' the first character
            return Err(self.get_scan_error_details(
                format!("{} not scanned", "digit").to_string()))
        }

        self.increment_location(1);
        increment_counter += 1;

        loop {
            let mut next_char = self.get_atom();
            if next_char.breaks_any_integer() {
                break
            } 
            else if !next_char.is_digit() {
                return Err(self.get_scan_error_details(
                    format!("{} not scanned", "integer digit").to_string()))
            }
            else {
                result.push(next_char.get_atom());
                self.increment_location(1);
                increment_counter += 1;
            }
        }

        Ok(self.get_token(result, TokenType::IntegerLiteral))
    }

    // TODO: Translate and validate these remaining methods
    fn scan_string_literal(&mut self) {
/*
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.breaks_any_string())
    {
        throw exceptions::scan_failure(get_char(), L"quotation");
    }

    auto next_char = get_char_atom();
    do
    {
        result += get_char();
        increment_location(1);
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any_string());

    result += get_char();
    increment_location(1);
    return token(token_type::string_literal, result);
*/
    }

    fn scan_identifier(&mut self) {
/*
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.is_identifier_char())
    {
        throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
    }

    auto next_char = get_char_atom();
    do
    {
        if (next_char->is_identifier_char())
        {
            result += get_char();
            increment_location(1);
        }
        else
        {
            throw exceptions::scan_failure(get_char(), L"letter, digit, underscore");
        }
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any());
    return token(token_type::identifier, result);
*/
    }

    fn scan_float_literal(&mut self) {
/*
    std::wstring result;

    auto first_char = atom_status(get_char());
    if (!first_char.is_digit())
    {
        throw exceptions::scan_failure(get_char(), L"digit");
    }

    auto next_char = get_char_atom();
    bool precision_part = false;
    do
    {
        if (next_char->is_digit())
        {
            result += get_char();
            increment_location(1);
        }
        else if (next_char->is_dot() && precision_part == false)
        {
            result += get_char();
            increment_location(1);
            precision_part = true;
        }
        else
        {
            throw exceptions::scan_failure(get_char(), L"digit");
        }
        next_char = get_char_atom();
    }
    while (!next_char->breaks_any_integer());

    return token(token_type::float_literal, result);
*/
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
        self.scan_single_char_operator(')', TokenType::ListDelimiter)
    }

    // file delimiters

    fn scan_end_of_file(&mut self) -> Result<Token, ScanError> {
        self.scan_single_char_operator('\0', TokenType::EndOfFile)
    }

    // scan delimiters
    // TODO: Is this method needed? can it be moved to the top?
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
