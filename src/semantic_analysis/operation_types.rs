use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationType {
    BooleanOrOperation,
    BooleanAndOperation,

    BooleanGtOperation,
    BooleanGteOperation,
    BooleanLtOperation,
    BooleanLteOperation,
    BooleanEqOperation,
    BooleanNeOperation,

    ConcatOperation,

    AdditionOperation,
    SubtractionOperation,
    MultiplicationOperation,
    DivisionOperation,

    NoOperation
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
