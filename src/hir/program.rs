use crate::webassembly;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Program {
	pub fn new(scope: Scope) -> Program {
		Program(scope)
	}
}

impl Program {
	pub fn type_check(&self) -> Result<()> {
		let main_ret_ty = Box::new(Type::Int);
		for key in self.0.keys() {
			match (key.as_str(), self.0.get(&key).unwrap()) {
				("main", Item::Fun(fun)) => {
					fun.write().expect("poison").type_check()?.constrain(&main_ret_ty)?;
				},
				(_, Item::Fun(fun)) => {
					fun.write().expect("poison").type_check()?;
				},
				// ("main", _) => return Err("main must be a function".to_string()),
			};
		}
		Ok(())
	}
}

impl Program {
	pub fn to_wasm(&self) -> Result<webassembly::Program> {
		let mut main_module = webassembly::Module::default();
		for key in self.0.keys() {
			match self.0.get(&key).expect("key that was from keys() doesn't exist") {
				Item::Fun(fun) => main_module.funcs.push(fun.read().expect("poison").to_wasm()?),
			}
		}
		Ok(webassembly::Program(vec![main_module]))
	}
}
