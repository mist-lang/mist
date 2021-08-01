use super::*;

impl WasmItem for Import {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(import \"");
		builder.push_str(self.module_name);
		builder.push_str("\" \"");
		builder.push_str(self.name);
		builder.push_str("\" ");
		builder.push_str(self.desc.to_wat(0).as_str());
		builder.push(')');
		builder
	}
}

impl WasmItem for ImportDesc {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		match self {
			ImportDesc::Func(name, ty) => {
				builder.push_str(format!("(func {}", name).as_str());
				ty.params.iter().for_each(|param| builder.push_str(param.to_wat(1).as_str()));
				ty.ret_ty.iter().for_each(|result| builder.push_str(result.to_wat(1).as_str()));
				builder.push(')');
			},
			ImportDesc::Table(_) => todo!(),
			ImportDesc::Mem(_, _) => todo!(),
			ImportDesc::Global(_) => todo!(),
		}
		builder
	}
}
