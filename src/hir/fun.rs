use crate::webassembly;

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
	pub fn to_wasm(&self) -> Result<webassembly::Func> {
		let params: Vec<_> = self.params.iter()
			.map(|p| webassembly::Param(
				webassembly::Index::Name(Box::leak(String::from(p.name()).into_boxed_str())),
				p.ty().to_wasm(),
			))
			.collect();
		let ret_ty = vec![webassembly::Result(self.ret_ty.to_wasm())];
		let instrs = self.expr.to_wasm()?;
		Ok(webassembly::Func {
			name: webassembly::Index::Name(Box::leak(self.name.clone().into_boxed_str())),
			typeuse: webassembly::FuncType {
				params,
				ret_ty,
			},
			locals: vec![],
			instrs,
		})
		// let mut ret_ty = self.ret_ty.as_ref();
		// while let Type::Arrow(_, ret) = ret_ty {
		// 	ret_ty = ret;
		// }
		// Ok(wasm::Func {
		// 	name,
		// 	result: ret_ty.to_wasm(),
		// 	expr: self.expr.to_wasm()?,
		// })
	}
}
