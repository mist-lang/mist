use std::sync::Arc;
use std::sync::RwLock;

use super::*;
use crate::hir;

type Result<T> = std::result::Result<T, String>;

impl File {
	pub fn load_to_hir(&self) -> Result<()> {
		self.items.iter().map(|item| match item {
			Item::Fun(fun) => fun.load_to_hir()
		}).collect()
	}
}

impl Type {
	fn to_hir(&self, scope: &hir::Scope) -> Result<hir::Type> {
		match self {
			Type::Int => Ok(hir::Type::Int),
			Type::Bool => Ok(hir::Type::Bool),
		}
	}
}

impl Fun {
	fn load_to_hir(&self) -> Result<()> {
		let fun_scope = hir::PROGRAM.main_scope.child();
		let params: Vec<_> = self.params.iter().map(|param| param.to_hir(&fun_scope)).collect();
		todo!()
	}
}

impl Var {
	fn to_hir(&self, scope: &hir::Scope) -> Result<hir::Dec> {
		let name = self.0.0.clone();
		let ty = self.1.to_hir(scope)?;
		Ok(hir::Dec::new(name, ty))
	}
}

impl Expr {
	fn to_hir(&self, scope: &hir::Scope) -> Result<(Vec<hir::Stmt>, hir::Expr)> {
		let mut stmts = Vec::new();
		let expr = match self {
			Expr::Bool(bb) => hir::Expr::Ref(scope.define_global(hir::Const::Bool(*bb))),
			Expr::Int(ii) => hir::Expr::Ref(scope.define_global(hir::Const::Int(*ii))),
			// Expr::VarRef(var_name) => hir::Expr::Ref(scope.get(var_name).unwrap()),
			_ => todo!()
		};
		Ok((stmts, expr))
	}
}

/*

fun main: int {
	if true { return 0; }
	let x = 3 + 1;
	x
}

fun main: int {
	let x = if true { return 0; } else { 3 };
	x + 1
}

fun main: int {
	let x = 3;
	if true {
		return 0;
	} else if true {
		x = x + 3;
	}
	x
}

*/
