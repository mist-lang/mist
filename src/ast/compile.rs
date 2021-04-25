use super::*;
use crate::wasm;

impl File {
	pub fn to_wasm_program(&self) -> wasm::Program {
		let mut funcs = Vec::new();

		for item in self.items.iter() {
			match item {
				Item::Fun(fun) => funcs.push(fun.to_wasm_func()),
			}
		}

		wasm::Program {
			funcs,
		}
	}
}

impl Fun {
	pub fn to_wasm_func(&self) -> wasm::Func {
		let name = self.name.0.clone();
		let expr = match &self.eval {
			Either::Left(expr) => wasm::Expr::Return(Box::new(expr.to_wasm_expr())),
			Either::Right(_block) => todo!(),
		};
		wasm::Func {
			name,
			result: wasm::Type::I32,
			expr,
		}
	}
}

impl Expr {
	pub fn to_wasm_expr(&self) -> wasm::Expr {
		use Expr::*;
		match self {
			Num(num) => wasm::Expr::Const(wasm::Type::I32, format!("{}", num)),
		}
	}
}
