// use tokentype;

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

	fn increment_location(&mut self) {
		self.location += 1;
	}

	fn char_at_location(&self) -> char {
		self.input[self.location]
	}
}

struct Scanner {

}

impl Scanner {

}