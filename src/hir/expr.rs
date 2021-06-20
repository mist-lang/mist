use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Expr {
	pub fn type_check(&mut self) -> Result<Type> {
		Ok(match self {
			Expr::Const(Const::Bool(_)) => Type::Bool,
			Expr::Const(Const::Int(_)) => Type::Int,
			Expr::Let { dec, assign, cont } => {
				dec.ty().constrain(&assign.type_check()?)?;
				cont.type_check()?
			},
			Expr::Call { fun: Item::Fun(fun), args } => {
				{ // do this in an inner block to ensure there aren't any RWLock shenanigans
					// ensure that the function is validly typed
					let mut fun_w = fun.write().expect("poisoned");
					// use the ret_ty field of the fun for the ret type of the expr,
					// not the result of constraining the fn's ret type to its expr's type
					fun_w.type_check()?;
				}

				let fun = fun.read().expect("poisoned");
				// TODO: methodology for once we have function currying:
				// 1. apply args to function's parameters
				// 2. apply args to function's ret type
				// 3. return rest of function's ret type
				if args.len() != fun.params.len() {
					return Err("function currying not yet supported".to_string());
				}
				args.into_iter()
					.zip(&fun.params)
					.map(|(expr, param)| expr.type_check()?.constrain(param.ty()))
					.collect::<Result<Vec<_>>>()?;
				*fun.ret_ty.clone()
			},
			Expr::Var(dec) => dec.ty().clone(),
			Expr::If {
				cond: _,
				out_ty: Some(ty),
				then: _,
				els: _,
			} => ty.clone(),
			Expr::If { cond, out_ty, then, els } => {
				cond.type_check()?.constrain(&Type::Bool)?;
				let then_ty = then.type_check()?;
				let els_ty = els.type_check()?;
				then_ty.constrain(&els_ty)?;
				els_ty.constrain(&then_ty)?;
				out_ty.insert(then_ty.clone());
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
			Expr::Let { dec, assign, cont } => todo!(),
			Expr::Call { fun, args } => todo!(),
			Expr::Var(_var) => todo!(),
			Expr::If { cond, out_ty, then, els } => wasm::Expr::If(Box::new(cond.to_wasm()?), out_ty.as_ref().map(Type::to_wasm).unwrap_or(wasm::Type::None), Box::new(then.to_wasm()?), Box::new(els.to_wasm()?)),
		})
	}
}
