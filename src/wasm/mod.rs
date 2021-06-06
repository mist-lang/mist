mod output;

#[derive(Default)]
pub struct Program {
	pub funcs: Vec<Func>,
}

pub struct Func {
	pub name: String,
	pub result: Type,
	pub expr: Expr,
}

pub enum Type {
	None,
	I32,
}

pub enum Expr {
	Return(Box<Expr>),
	Const(Type, String),
	If(Box<Expr>, Type, Box<Expr>, Box<Expr>),
}

pub enum Lit {
	I32(i32),
}
