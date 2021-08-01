use super::*;

impl WasmItem for Instr {
	fn to_wat(&self, indent: usize) -> String {
		match self {
			Instr::Plain(plain_instr) => plain_instr.to_wat(indent),
			Instr::Block(block_instr) => block_instr.to_wat(indent),
		}
	}
}

impl WasmItem for PlainInstr {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		match self {
			PlainInstr::I32(i32_instr) => builder.push_str(i32_instr.to_wat(0).as_str()),
			PlainInstr::call(name) => builder.push_str(format!("call {}", name).as_str()),
			instr @ _ => todo!("{:?}", instr),
		}
		builder
	}
}

impl WasmItem for I32Instr {
	fn to_wat(&self, indent: usize) -> String {
		use I32Instr::*;
		let mut builder = self.indented(indent);
		match self {
			r#const(val) => builder.push_str(format!("i32.const {}", val).as_str()),
			_ => todo!(),
		}
		builder
	}
}

impl WasmItem for BlockInstr {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		match self {
			BlockInstr::r#if(ty, then, els) => {
				builder.push_str("if");
				builder.push_str(ty.to_wat(1).as_str());
				builder.push('\n');
				then.into_iter().for_each(|instr| {
					builder.push_str(instr.to_wat(indent + 2).as_str());
					builder.push('\n');
				});
				self.indent(indent, &mut builder).push_str("else\n");
				els.into_iter().for_each(|instr| {
					builder.push_str(instr.to_wat(indent + 2).as_str());
					builder.push('\n');
				});
				self.indent(indent, &mut builder).push_str("end");
			},
			_ => todo!(),
		}
		builder
	}
}
