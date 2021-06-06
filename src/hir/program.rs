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
		let main_ret_ty = Type::Arrow(Box::new(Type::Tuple(vec![])), Box::new(Type::Int));
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
	pub fn to_wasm(&self) -> Result<wasm::Program> {
		let mut program = wasm::Program::default();
		for key in self.0.keys() {
			match self.0.get(&key).unwrap() {
				Item::Fun(fun) => program.funcs.push(fun.read().expect("poison").to_wasm(key)?),
			}
		}
		Ok(program)
	}
}
