use crate::wasm;

use super::*;

type Result<T> = std::result::Result<T, String>;

impl Fun {
	pub fn new(params: Vec<Arc<Dec>>, expr: Box<Expr>, ret_ty: Type) -> Fun {
		let cloned_params = params.clone();
		Fun {
			params,
			expr,
			fn_ty: if cloned_params.len() == 0 {
				Type::Arrow(Box::new(Type::Tuple(vec![])), Box::new(ret_ty))
			} else {
				cloned_params.iter().rev().fold(ret_ty, |acc, el| Type::Arrow(Box::new(el.ty.clone()), Box::new(acc)))
			},
		}
	}
}

impl Fun {
	pub fn type_check(&mut self) -> Result<Type> {
		let expr_ty = self.expr.type_check()?;
		let constrain_ty = if self.params.len() == 0 {
			Type::Arrow(Box::new(Type::Tuple(vec![])), Box::new(expr_ty))
		} else {
			self.params.iter().rev().fold(expr_ty, |acc, el| Type::Arrow(Box::new(el.ty.clone()), Box::new(acc)))
		};
		self.fn_ty.constrain(&constrain_ty)?;
		Ok(self.fn_ty.clone())
	}
}

impl Fun {
	pub fn to_wasm(&self, name: String) -> Result<wasm::Func> {
		let mut ret_ty = &self.fn_ty;
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
