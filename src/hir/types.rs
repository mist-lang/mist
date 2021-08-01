use crate::webassembly;
use super::Type;

impl Type {
  pub fn constrain(&self, constraint: &Type) -> Result<Option<Box<Type>>, String> {
    Ok(match (self, constraint) {
      (Type::Bool, Type::Bool) => None,
      (Type::Int, Type::Int) => None,
      (Type::Arrow(s_param, s_ret), Type::Arrow(c_param, c_ret)) => {
				s_param.constrain(c_param)?;
				s_ret.constrain(c_ret)?
			},
      (Type::Tuple(tys), _) if tys.len() == 1 => tys[0].constrain(constraint)?,
      (_, Type::Tuple(tys)) if tys.len() == 1 => self.constrain(&tys[0])?,
      (Type::Tuple(l_tys), Type::Tuple(r_tys)) => {
				l_tys.into_iter()
					.zip(r_tys)
					.map(|(lt, rt)| lt.constrain(rt))
					// TODO: I don't believe there's a reson to look at the vecs,
					// but if something goes wrong then this will have to be double-checked
					.collect::<Result<Vec<_>, String>>()?;
				None
			}
      (left, right) => return Err(format!("Type {:?} could not be constrained to type {:?}", left, right)),
    })
  }
}

impl Type {

	pub fn to_wasm(&self) -> webassembly::ValType {
		use webassembly::ValType;
		use webassembly::NumType;
		match self {
			Type::Bool => ValType::Num(NumType::I32),
			Type::Int => ValType::Num(NumType::I32),
			_ => todo!()
		}
	}

	// pub fn to_wasm(&self) -> wasm::Type {
	// 	match self {
	// 		Type::Bool => wasm::Type::I32,
	// 		Type::Int => wasm::Type::I32,
	// 		Type::Tuple(tys) if tys.len() == 0 => wasm::Type::None,
	// 		Type::Tuple(tys) if tys.len() == 1 => tys[0].to_wasm(),
	// 		_ => todo!(),
	// 	}
	// }
}

