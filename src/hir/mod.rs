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
	pub params: Vec<Arc<Dec>>,
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
pub struct Dec {
	pub name: String,
	pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Const(Const),
	Let(Arc<Dec>, Box<Expr>, Box<Expr>),
	Call(Item, Vec<Expr>),
	Var(Arc<Dec>),
	If(Box<Expr>, Option<Type>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Const {
	Bool(bool),
	Int(u64),
}
