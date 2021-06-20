use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Expr {
	pub fn type_check(&mut self) -> Result<Type> {
		Ok(match self {
			Expr::Const(Const::Bool(_)) => Type::Bool,
			Expr::Const(Const::Int(_)) => Type::Int,
			Expr::Let(dec, assign, cont) => {
				dec.ty.constrain(&assign.type_check()?)?;
				cont.type_check()?
			},
			Expr::Call(Item::Fun(fun), args) => {
				// ensure that the function is validly typed
				let mut fun_w = fun.write().expect("poisoned");
				// use the ret_ty field of the fun for the ret type of the expr,
				// not the result of constraining the fn's ret type to its expr's type
				fun_w.type_check()?;
				// reset the rwlock
				drop(fun_w);

				// 1. apply args to function's parameters
				let fun_r = fun.read().expect("poisoned");
				let rest_args_ii = for (ii, arg) in args.into_iter().enumerate() {
					// TODO: add type fields to all of the Exprs and make a getter
					// so that we don't have to always type-check the same expr many times
				};
				// 2. apply args to function's ret type
				// 3. return rest of function's ret type
				todo!()
			},
			Expr::Var(dec) => dec.ty.clone(),
			Expr::If(_, Some(ty), _, _) => ty.clone(),
			Expr::If(cond, ty, then, els) => {
				cond.type_check()?.constrain(&Type::Bool)?;
				let then_ty = then.type_check()?;
				let els_ty = els.type_check()?;
				then_ty.constrain(&els_ty)?;
				els_ty.constrain(&then_ty)?;
				ty.insert(then_ty.clone());
				then_ty
			},
		})
	}
}

impl Expr {
	pub fn to_wasm(&self) -> Result<wasm::Expr> {
		Ok(match self {
			Expr::Const(Const::Bool(true)) => wasm::Expr::Const(wasm::Type::I32, "1".to_string()),
			Expr::Const(Const::Bool(false)) => wasm::Expr::Const(wasm::Type::I32, "0".to_string()),
			Expr::Const(Const::Int(i)) => wasm::Expr::Const(wasm::Type::I32, format!("{}", i)),
			Expr::Let(_dec, _assign, _cont) => todo!(),
			Expr::Call(_fun, _args) => todo!(),
			Expr::Var(_var) => todo!(),
			Expr::If(cond, ty, then, els) => wasm::Expr::If(Box::new(cond.to_wasm()?), ty.as_ref().map(Type::to_wasm).unwrap_or(wasm::Type::None), Box::new(then.to_wasm()?), Box::new(els.to_wasm()?)),
		})
	}
}
