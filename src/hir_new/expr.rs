use super::*;

type Result<T> = std::result::Result<T, String>;

impl Expr {
	pub fn type_check(&self) -> Result<Type> {
		Ok(match self {
			Expr::Const(Const::Bool(_)) => Type::Bool,
			Expr::Const(Const::Int(_)) => Type::Int,
			Expr::Let(dec, assign, cont) => todo!(),
			Expr::Call(Item::Fun(fun), args) => todo!(),
			Expr::Call(_, _) => todo!(),
			Expr::Var(dec) => todo!(),
			Expr::If(cond, then, els) => todo!(),
		})
	}
}
