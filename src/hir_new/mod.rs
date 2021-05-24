mod expr;
mod fun;
mod program;
mod scope;
pub use scope::Scope;

use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Program(Scope);

#[derive(Debug)]
pub enum Item {
	Fun(Arc<RwLock<Fun>>),
}

#[derive(Debug)]
pub struct Fun {
	pub params: Vec<Arc<Dec>>,
	pub expr: Box<Expr>,
}

#[derive(Debug)]
pub enum Type {
	Int,
	Bool,
	Arrow(Box<Type>, Box<Type>),
	Tuple(Vec<Type>),
}

#[derive(Debug)]
pub struct Dec(pub String, pub Type);

#[derive(Debug)]
pub enum Expr {
	Const(Const),
	Let(Arc<Dec>, Box<Expr>, Box<Expr>),
	Call(Item, Vec<Expr>),
	Var(Arc<Dec>),
	If(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Const {
	Bool(bool),
	Int(u64),
}