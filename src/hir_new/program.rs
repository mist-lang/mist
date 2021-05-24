use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Program {
	pub fn new(scope: Scope) -> Program {
		Program(scope)
	}
}

impl Program {
	pub fn type_check(&self) -> Result<()> {
		let poisoned = |_| "lock to function was poisoned before I could type-check".to_string();
		self.0.iter()
			.map(|(item_name, item)| match (item_name.as_str(), item) {
				("main", Item::Fun(fun)) => fun.write().map_err(poisoned)?.type_check().and_then(|main_ty| todo!()),
				("main", _) => Err("main must be a function".to_string()),
				(_, Item::Fun(fun)) => fun.write().map_err(poisoned)?.type_check().and(Ok(())),
			})
			.collect::<Result<Vec<()>>>()?;
		Ok(())
	}
}

impl Program {
	pub fn to_wasm(&self) -> Result<wasm::Program> {
		todo!()
	}
}
