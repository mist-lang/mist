use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Fun {
	pub fn new(params: Vec<Arc<Dec>>, expr: Box<Expr>) -> Fun {
		Fun {
			params,
			expr,
		}
	}
}

impl Fun {
	pub fn type_check(&mut self) -> Result<Type> {
		todo!()
	}
}

impl Fun {
	pub fn to_wasm(&self) -> Result<wasm::Func> {
		todo!()
	}
}
