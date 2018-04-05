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
    // operators for special operations
    AssignmentOperator,
    // program delimiters
    Semicolon,
    ListDelimiter,
    // keywords
    FunctionKeyword,
    InfiniteKeyword,
    BreakKeyword,
    FeatureKeyword,
    AutobreakKeyword,
    OnKeyword,
    OffKeyword,
    // error state
    // used for initialization only
    Invalid
}