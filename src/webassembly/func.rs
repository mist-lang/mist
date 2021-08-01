use super::*;

impl WasmItem for Func {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(func ");
		builder.push_str(format!("{}", self.name).as_str());
		builder.push_str(self.typeuse.to_wat(1).as_str());
		self.locals.iter().for_each(|_local| todo!("locals"));
		self.instrs.iter().for_each(|instr| {
			builder.push('\n');
			builder.push_str(instr.to_wat(indent + 2).as_str());
		});
		builder.push(')');
		builder
	}
}

impl WasmItem for FuncType {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = String::new();
		self.params.iter().for_each(|param| builder.push_str(param.to_wat(indent).as_str()));
		self.ret_ty.iter().for_each(|result| builder.push_str(result.to_wat(indent).as_str()));
		builder
	}
}

impl WasmItem for Param {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(param");
		builder.push_str(format!(" {}", self.0).as_str());
		builder.push_str(format!(" {})", self.1.to_wat(0)).as_str());
		builder
	}
}
