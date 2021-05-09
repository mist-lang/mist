use super::*;
use crate::wasm;

impl Type {
	pub fn to_wasm_type(&mut self) -> Option<wasm::Type> {
		use Type::*;
		match self {
			Int => Some(wasm::Type::I32),
			Bool => Some(wasm::Type::I32),
			Tuple(types) if types.len() == 0 => None,
			Tuple(_types) => todo!("tuples"),
		}
	}
}

impl File {
	pub fn to_wasm_program(&mut self) -> wasm::Program {
		let mut funcs = Vec::new();

		for item in self.items.iter_mut() {
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
	pub fn to_wasm_func(&mut self) -> wasm::Func {
		let name = self.name.0.clone();
		let expr = match &mut self.eval {
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

impl Block {
	pub fn to_wasm_expr(&mut self) -> wasm::Expr {
		// TODO: build up the wasm::Block https://webassembly.github.io/spec/core/text/instructions.html#control-instructions
		match &mut self.expr {
			Some(expr) => expr.to_wasm_expr(),
			None => todo!("wasm blocks"),
		}
	}
}

impl Expr {
	pub fn to_wasm_expr(&mut self) -> wasm::Expr {
		use Expr::*;
		match self {
			Int(num) => wasm::Expr::Const(wasm::Type::I32, format!("{}", num)),
			Bool(false) => wasm::Expr::Const(wasm::Type::I32, "0".to_string()),
			Bool(true) => wasm::Expr::Const(wasm::Type::I32, "1".to_string()),
			If(if_expr) => if_expr.to_wasm_expr(),
		}
	}
}

impl If {
	pub fn to_wasm_expr(&mut self) -> wasm::Expr {
		let mut out_ty = self.type_check().unwrap();
		let cond = self.0.to_wasm_expr();
		let then = self.1.to_wasm_expr();
		let el = match self.2.as_mut() {
			Either::Left(block) => block.to_wasm_expr(),
			Either::Right(if_expr) => if_expr.to_wasm_expr(),
		};
		wasm::Expr::If(Box::new(cond), out_ty.to_wasm_type(), Box::new(then), Box::new(el))
	}
}
