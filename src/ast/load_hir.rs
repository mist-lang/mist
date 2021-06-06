use std::sync::Arc;
use std::sync::RwLock;

use super::*;
use crate::hir;

type Result<T> = std::result::Result<T, String>;

impl File {
	pub fn compile_hir(&self) -> Result<hir::Program> {
		let hir_scope = hir::Scope::with_builtins();
		self.items.iter()
			.map(|item| Ok(match item {
				Item::Fun(fun) => hir_scope.insert(
					fun.name.0.clone(),
					hir::Item::Fun(Arc::new(RwLock::new(fun.compile_hir(&vec![], &hir_scope)?)))
				),
			}))
			.collect::<Result<Vec<()>>>()?;
		Ok(hir::Program::new(hir_scope))
	}
}

impl Var {
	pub fn to_hir_dec(&self) -> hir::Dec {
		hir::Dec {
			name: self.0.0.clone(),
			ty: self.1.to_hir_type(),
		}
	}
}

impl Type {
	pub fn to_hir_type(&self) -> hir::Type {
		match self {
			Type::Int => hir::Type::Int,
			Type::Bool => hir::Type::Bool,
		}
	}
}

impl Fun {
	// vars is for once we add lambdas, which is soon!
	pub fn compile_hir(&self, vars: &Vec<Arc<hir::Dec>>, scope: &hir::Scope) -> Result<hir::Fun> {
		let params = self.params.iter().map(Var::to_hir_dec).map(Arc::new).collect::<Vec<_>>();
		let vars: Vec<_> = params.iter().map(Arc::clone).chain(vars.into_iter().map(Arc::clone)).collect();
		let expr = match self.eval.as_ref() {
			Either::Left(expr) => expr.compile_hir(&vars, scope),
			Either::Right(block) => block.compile_hir(&vars, scope),
		}?;
		Ok(hir::Fun::new(params, Box::new(expr), self.out_ty.to_hir_type()))
	}
}

impl Block {
	pub fn compile_hir(&self, vars: &Vec<Arc<hir::Dec>>, scope: &hir::Scope) -> Result<hir::Expr> {
		// TODO: more complex blocks
		match self.expr.as_ref() {
			Some(expr) => expr.compile_hir(vars, scope),
			None => Err("blocks not fully implemented yet".to_string()),
		}
	}
}

impl Expr {
	pub fn compile_hir(&self, vars: &Vec<Arc<hir::Dec>>, scope: &hir::Scope) -> Result<hir::Expr> {
		Ok(match self {
			Expr::Bool(b) => hir::Expr::Const(hir::Const::Bool(*b)),
			Expr::Int(i) => hir::Expr::Const(hir::Const::Int(*i)),
			Expr::VarRef(ident) => hir::Expr::Var(vars.iter().find(|dec| dec.name == ident.0).ok_or_else(|| format!("{} isn't defined", ident.0))?.clone()),
			Expr::If(if_expr) => if_expr.compile_hir(vars, scope)?,
			Expr::Call(fun_name, args) => {
				let fun = match vars.iter().find(|var| var.name == fun_name.0) {
					Some(_lam) => todo!(),
					None => scope.get(&fun_name.0).ok_or_else(|| format!("function {} not found", fun_name.0)),
				}?;
				let args = args.iter().map(|arg| arg.compile_hir(vars, scope)).collect::<Result<Vec<_>>>()?;
				hir::Expr::Call(fun, args)
			}
		})
	}
}

impl If {
	pub fn compile_hir(&self, vars: &Vec<Arc<hir::Dec>>, scope: &hir::Scope) -> Result<hir::Expr> {
		Ok({
			let cond = self.0.compile_hir(vars, scope)?;
			let then = self.1.compile_hir(vars, scope)?;
			let els = match self.2.as_ref() {
				Either::Left(block) => block.compile_hir(vars, scope),
				Either::Right(elif) => elif.compile_hir(vars, scope),
			}?;
			hir::Expr::If(Box::new(cond), None, Box::new(then), Box::new(els))
		})
	}
}
