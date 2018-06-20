#[derive(Debug, Clone)]
pub struct CompileError {
    location: usize,
    line: usize,
    column: usize,
    error_message: String
}

impl CompileError {
    pub fn init(location: usize, line: usize, column: usize, error_message: String) -> CompileError {
        CompileError {
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