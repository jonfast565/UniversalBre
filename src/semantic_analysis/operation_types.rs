
#[derive(Debug, Clone, PartialEq)]
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
}