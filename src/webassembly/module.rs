use super::*;

impl Module {
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
								Param(Index::Name(""), ValType::Num(NumType::I32)),
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
