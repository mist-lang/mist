use super::*;

impl WasmItem for Export {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(export \"");
		builder.push_str(self.name);
		builder.push_str("\" ");
		builder.push_str(self.desc.to_wat(indent).as_str());
		builder.push_str(")");
		builder
	}
}

impl WasmItem for ExportDesc {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = String::new();
		match self {
			ExportDesc::Func(Either::Left(name)) => builder.push_str(format!("{}", name).as_str()),
			ExportDesc::Func(Either::Right(func)) => {
				builder.push('\n');
				builder.push_str(func.to_wat(indent + 2).as_str())
			},
			ExportDesc::Table(name) => builder.push_str(format!("{}", name).as_str()),
			ExportDesc::Memory(Either::Left(name)) => builder.push_str(format!("{}", name).as_str()),
			ExportDesc::Memory(Either::Right(mem)) => builder.push_str(mem.to_wat(0).as_str()),
			ExportDesc::Global(name) => builder.push_str(format!("{}", name).as_str()),
		}
		builder
	}
}
