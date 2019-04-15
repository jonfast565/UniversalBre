use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    StringType,
    IntegerType,
    FloatType,
    BooleanType,
    AnyType,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
