pub trait Ordering {
	fn get_order(&self) -> usize;
	fn set_order(&mut self, order: usize);
}