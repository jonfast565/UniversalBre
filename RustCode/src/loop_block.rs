use ordering::Ordering;

pub struct LoopBlock {
	statement_order: usize
}

impl LoopBlock {
	pub fn init(statement_order: usize) -> LoopBlock {
		LoopBlock {
			statement_order: statement_order
		}
	}
}

impl Ordering for LoopBlock {
	fn get_order(&self) -> usize {
		self.statement_order
	}

	fn set_order(&mut self, order: usize) {
		self.statement_order = order;
	}
}