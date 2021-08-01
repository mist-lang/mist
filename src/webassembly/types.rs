use super::*;

impl WasmItem for Result {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(result ");
		builder.push_str(self.0.to_wat(0).as_str());
		builder.push_str(")");
		builder
	}
}

impl WasmItem for BlockType {
	fn to_wat(&self, indent: usize) -> String {
		match self {
			BlockType::None => String::new(),
			BlockType::Result(res) => res.to_wat(indent),
			BlockType::TypeUse(ty) => ty.to_wat(indent),
		}
	}
}

impl WasmItem for ValType {
	fn to_wat(&self, indent: usize) -> String {
		match self {
			ValType::Num(num_ty) => num_ty.to_wat(indent),
		}
	}
}

impl WasmItem for NumType {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		match self {
			NumType::I32 => builder.push_str("i32"),
			_ => todo!(),
		}
		builder
	}
}
