mod builtins;

use crate::hir::Item;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Scope(Arc<RwLock<InnerScope>>);

#[derive(Debug)]
struct InnerScope {
  vals: HashMap<String, Item>,
  upstream: Option<Scope>,
}

impl Scope {
  pub fn with_builtins() -> Scope {
    // builtins::CORE.child()
		Scope(Arc::new(RwLock::new(InnerScope {
			vals: HashMap::new(),
			upstream: None,
		})))
  }

  pub fn insert(&self, name: String, item: Item) {
    let mut inner = self.0.write().expect("poison");
    inner.vals.insert(name.to_string(), item);
  }

  pub fn get(&self, key: impl AsRef<str>) -> Option<Item> {
    let inner = self.0.read().expect("poison");
    match (inner.vals.get(key.as_ref()), inner.upstream.as_ref()) {
      (Some(Item::Fun(fun)), _) => Some(Item::Fun(fun.clone())),
      (None, Some(upstream)) => upstream.get(key),
      (None, None) => None,
    }
  }

  pub fn remove(&self, key: impl AsRef<str>) -> Option<Item> {
    let mut inner = self.0.write().expect("poison");
    inner.vals.remove(key.as_ref())
  }

  pub fn child(&self) -> Scope {
    Scope(Arc::new(RwLock::new(InnerScope {
      vals: HashMap::new(),
      upstream: Some(Scope(self.0.clone())),
    })))
  }
}

impl Scope {
	pub fn keys(&self) -> <Vec<String> as IntoIterator>::IntoIter {
		let inner = self.0.read().expect("poison");
		inner.vals.keys().map(String::to_owned).collect::<Vec<String>>().into_iter()
	}
}
