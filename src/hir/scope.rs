use super::*;

pub struct Scope();

impl Scope {
	pub fn child(&self) -> Scope {
		todo!()
		// Scope(Arc::new((Default::default(), Some(self.0.clone()))))
	}

	pub fn insert(&self, name: String, item: Item) {
		todo!()
	}

	pub fn get(&self, name: String) -> Option<&Item> {
		todo!()
	}

	pub fn define_global(&self, val: Const) -> Ref {
		todo!()
	}
}
