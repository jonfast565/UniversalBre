use statement::Statement;

pub struct Program {
	statements: Vec<Statement>
}

impl Program {
	pub fn init(statements: Vec<Statement>) -> Program {
		Program {
			statements: statements
		}
	}
}