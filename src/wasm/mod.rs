mod output;

pub struct Program {
	pub funcs: Vec<Func>,
}

pub struct Func {
	pub name: String,
	pub result: Type,
	pub expr: Expr,
}

pub enum Type {
	I32,
}

pub enum Expr {
	Return(Box<Expr>),
	Const(Type, String),
}

pub enum Lit {
	I32(i32),
}
