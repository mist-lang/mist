use std::collections::hash_map::Iter;

use crate::hir::Dec;

use super::Item;

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Scope();

// basically libcore
pub static CORE: Lazy<Scope> = Lazy::new(|| todo!());

impl Scope {
	pub fn with_builtins() -> Scope {
		todo!()
	}

	pub fn insert(&mut self, name: String, item: Item) {
		todo!()
	}

	pub fn get(&self, key: impl AsRef<str>) -> Option<Item> {
		todo!()
	}

	pub fn remove(&self, key: impl AsRef<str>) -> Option<Item> {
		todo!()
	}

	pub fn child(&self) -> Scope {
		todo!()
	}
}

impl Scope {
	pub fn iter<'scope>(&'scope self) -> Iter<'scope, String, Item> {
		self.into_iter()
	}
}

impl<'scope> IntoIterator for &'scope Scope {
    type Item = (&'scope String, &'scope Item);
    type IntoIter = Iter<'scope, String, Item>;

    fn into_iter(self) -> Self::IntoIter {
			todo!()
    }
}
