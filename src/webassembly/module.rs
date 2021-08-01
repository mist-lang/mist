use super::*;

impl WasmItem for Module {
	fn to_wat(&self, _indent: usize) -> String {
		let mut wat_out = String::new();
		wat_out.push_str("(module\n");
		wat_out.push_str(format!("  (start {})\n", self.start).as_str());
		self.imports.iter().for_each(|import| {
			wat_out.push_str(import.to_wat(2).as_str());
			wat_out.push('\n');
		});
		self.exports.iter().for_each(|export| {
			wat_out.push_str(export.to_wat(2).as_str());
			wat_out.push('\n');
		});
		self.mems.iter().for_each(|mem| {
			wat_out.push_str(mem.to_wat(2).as_str());
			wat_out.push('\n');
		});
		self.funcs.iter().for_each(|func| {
			wat_out.push_str(func.to_wat(2).as_str());
			wat_out.push('\n');
		});
		wat_out.push_str(")\n");
		wat_out
	}
}

impl Default for Module {
  fn default() -> Self {
		Module {
			start: Index::Name("_start"),
			imports: vec![
				Import {
					module_name: "wasi_snapshot_preview1",
					name: "proc_exit",
					desc: ImportDesc::Func(
						Index::Name("proc_exit"),
						FuncType {
							params: vec![
								Param(Index::None, ValType::Num(NumType::I32)),
							],
							ret_ty: vec![],
						},
					),
				},
			],
			exports: vec![
				Export {
					name: "memory",
					desc: ExportDesc::Memory(Either::Right(Memory(Limits(0, None)))),
				},
			],
			mems: vec![
				Memory(Limits(1, None)),
			],
			funcs: vec![
				Func {
					name: Index::Name("_start"),
					typeuse: FuncType {
						params: vec![],
						ret_ty: vec![],
					},
					locals: vec![],
					instrs: vec![
						Instr::Plain(PlainInstr::call(Index::Name("main"))),
						Instr::Plain(PlainInstr::call(Index::Name("proc_exit"))),
					],
				}
			],
		}
  }
}
