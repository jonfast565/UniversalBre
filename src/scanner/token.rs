use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // file markers
    EndOfFile,
    // scoping
    ScopeBeginOperator,
    ScopeEndOperator,
    // literals
    IntegerLiteral,
    StringLiteral,
    FloatLiteral,
    Identifier,
    BooleanTrueLiteral,
    BooleanFalseLiteral,
    // operators
    PlusOperator,
    MinusOperator,
    MultiplyOperator,
    DivideOperator,
    ConcatOperator,
    // boolean equality operators
    // equal to
    BooleanEqOperator,
    // not equal to
    BooleanNeOperator,
    // boolean comparison operators
    BooleanGtOperator,
    // greater than
    BooleanLtOperator,
    // less than
    BooleanLteOperator,
    // less than or equal to
    BooleanGteOperator,
    // greater than or equal to
    // boolean combinators
    BooleanAndOperator,
    BooleanOrOperator,
    // parenthesis
    LeftParenthesis,
    RightParenthesis,
    LeftIndexer,
    RightIndexer,
    // operators for special operations
    AssignmentOperator,
    // program delimiters
    Semicolon,
    ListDelimiter,
    TypeSpecifier,
    // keywords
    FunctionKeyword,
    InfiniteKeyword,
    BreakKeyword,
    FeatureKeyword,
    AutobreakKeyword,
    OnKeyword,
    OffKeyword,
    ReturnKeyword,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    line: usize,
    column: usize,
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn init(line: usize, column: usize, token_type: TokenType, lexeme: String) -> Token {
        Token {
            line: line,
            column: column,
            token_type: token_type,
            lexeme: lexeme,
        }
    }

    pub fn get_token_type(&self) -> &TokenType {
        self.token_type
    }

    pub fn get_lexeme(&self) -> &String {
        self.lexeme
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}
