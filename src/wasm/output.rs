use super::*;

use std::collections::BTreeMap;
use std::fmt;

use handlebars::Handlebars;
use once_cell::sync::Lazy;

impl Program {
	pub fn to_wat(&self) -> String {
		static TEMPLATES: Lazy<Handlebars> = Lazy::new(|| {
			let mut bars = Handlebars::new();
			bars.register_template_string("program", "\
(module
	(import \"wasi_snapshot_preview1\" \"proc_exit\" (func $proc_exit (param i32)))

	(memory 1)
	(export \"memory\" (memory 0))

	(func $_start (export \"_start\")
		(call $proc_exit (call $main))
	)

	{{funcs}}
)").unwrap();
			bars
		});

		let mut tree = BTreeMap::new();
		tree.insert("funcs", self.funcs.iter().map(Func::to_wat).collect::<Vec<_>>().join(""));
		TEMPLATES.render("program", &tree).unwrap()
	}
}

impl Func {	
	pub fn to_wat(&self) -> String {
		static TEMPLATES: Lazy<Handlebars> = Lazy::new(|| {
			let mut bars = Handlebars::new();
			bars.register_template_string("func", "
(func ${{name}} (result {{result}})
	{{expr}}
)
").unwrap();
			bars
		});

		let mut tree = BTreeMap::new();
		tree.insert("name", self.name.clone());
		tree.insert("result", format!("{}", self.result));
		tree.insert("expr", self.expr.to_wat());
		TEMPLATES.render("func", &tree).unwrap()
	}
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Type::*;
		match self {
			I32 => write!(f, "i32"),
		}
	}
}

impl Expr {
	pub fn to_wat(&self) -> String {
		static TEMPLATES: Lazy<Handlebars> = Lazy::new(|| {
			let mut bars = Handlebars::new();
			bars.register_template_string("return", "(return {{expr}})").unwrap();
			bars.register_template_string("const", "({{ty}}.const {{val}})").unwrap();
			bars.register_template_string("if-expr", "\
(if (result {{ty}})
	{{cond}}
	(then {{then}})
	(else {{el}})
)
").unwrap();
			bars.register_template_string("if-stmt", "\
(if
	({{cond}})
	(then {{then}})
	(else {{el}})
)
").unwrap();
			bars
		});

		let mut data = BTreeMap::new();
		use Expr::*;
		let template = match self {
			Return(expr) => {
				data.insert("expr", expr.to_wat());
				"return"
			},
			Const(ty, val) => {
				data.insert("ty", format!("{}", ty));
				data.insert("val", val.clone());
				"const"
			},
			If(cond, ty, then, el) => {
				let fmt = if let Some(ty) = ty {
					data.insert("ty", format!("{}", ty));
					"if-expr"
				} else {
					"if-stmt"
				};
				data.insert("cond", cond.to_wat());
				data.insert("then", then.to_wat());
				data.insert("el", el.to_wat());
				fmt
			},
		};
		TEMPLATES.render(template, &data).unwrap()
	}
}

impl Lit {
	pub fn to_wat(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Lit::*;
		match self {
			I32(num) => write!(f, "(i32.const {})", *num),
		}
	}

	pub fn _to_wat(&self) -> String {
		todo!()
	}
}
