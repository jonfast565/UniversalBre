use std::fmt;

#[derive(Debug, Clone)]
pub struct CompileError {
    location: usize,
    line: usize,
    column: usize,
    error_message: String,
}

impl CompileError {
    pub fn init(
        location: usize,
        line: usize,
        column: usize,
        error_message: String,
    ) -> CompileError {
        CompileError {
            location: location,
            line: line,
            column: column,
            error_message: error_message,
        }
    }

    pub fn get_error_message(&self) -> String {
        return self.error_message.clone();
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}: {}", self.line, self.column, self.error_message)
    }
}
