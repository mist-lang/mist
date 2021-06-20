mod dec;
mod expr;
mod fun;
mod program;
mod scope;
pub use scope::Scope;
mod types;

use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Program(Scope);

#[derive(Debug, Clone)]
pub enum Item {
	Fun(Arc<RwLock<Fun>>),
}

#[derive(Debug)]
pub struct Fun {
	pub params: Vec<Dec>,
	pub expr: Box<Expr>,
	pub ret_ty: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
	Int,
	Bool,
	Arrow(Box<Type>, Box<Type>),
	Tuple(Vec<Type>),
}

#[derive(Debug, Clone)]
pub struct Dec(Arc<DecInner>);

#[derive(Debug)]
struct DecInner {
	pub name: String,
	pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Const(Const),
	/// LISP-style let binding
	Let {
		dec: Dec,
		assign: Box<Expr>,
		cont: Box<Expr>,
	},
	Call {
		fun: Item,
		args: Vec<Expr>,
	},
	Var(Dec),
	If {
		cond: Box<Expr>,
		out_ty: Option<Type>,
		then: Box<Expr>,
		els: Box<Expr>,
	},
}

#[derive(Debug, Clone)]
pub enum Const {
	Bool(bool),
	Int(u64),
}
