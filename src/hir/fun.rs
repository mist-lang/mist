use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Fun {
	pub fn type_check(&mut self) -> Result<Type> {
		let expr_ty = self.expr.type_check()?;
		self.ret_ty.constrain(&expr_ty)?;
		Ok(*self.ret_ty.clone())
	}
}

impl Fun {
	pub fn to_wasm(&self, name: String) -> Result<wasm::Func> {
		let mut ret_ty = self.ret_ty.as_ref();
		while let Type::Arrow(_, ret) = ret_ty {
			ret_ty = ret;
		}
		Ok(wasm::Func {
			name,
			result: ret_ty.to_wasm(),
			expr: self.expr.to_wasm()?,
		})
	}
}
