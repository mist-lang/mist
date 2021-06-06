use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Expr {
	pub fn type_check(&mut self) -> Result<Type> {
		let res = match self {
			Expr::Const(Const::Bool(_)) => Type::Bool,
			Expr::Const(Const::Int(_)) => Type::Int,
			Expr::Let(dec, assign, cont) => {
				dec.ty.constrain(&assign.type_check()?)?;
				cont.type_check()?
			},
			Expr::Call(Item::Fun(_fun), _args) => todo!(),
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
		};
		Ok(res)
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
