use ordering::Ordering;

pub struct Statement {
	statement_order: usize
}

impl Statement {
	pub fn init(statement_order: usize) -> Statement {
		Statement {
			statement_order: statement_order
		}
	}
}

impl Ordering for Statement {
	fn get_order(&self) -> usize {
		self.statement_order
	}

	fn set_order(&mut self, order: usize) {
		self.statement_order = order;
	}
}