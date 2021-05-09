mod compile;
mod parse;
mod types;

use either::Either;

#[derive(Debug)]
pub struct File {
	pub items: Vec<Item>
}

#[derive(Debug)]
pub struct Ident(String);

#[derive(Debug)]
pub enum Item {
	Fun(Box<Fun>),
}

#[derive(Clone, Debug)]
pub enum Type {
	Int,
	Bool,
	Tuple(Vec<Type>),
}

#[derive(Debug)]
pub struct Fun {
	name: Ident,
	out_ty: Type,
	eval: Either<Expr, Block>,
}

/// Rust-style blocks
#[derive(Debug)]
pub struct Block {
	stmts: Vec<Stmt>,
	expr: Option<Expr>,
}

#[derive(Debug)]
pub enum Stmt {
	// TODO: statements
}

#[derive(Debug)]
pub enum Expr {
	Bool(bool),
	Int(u64),
	If(If),
}

#[derive(Debug)]
pub struct If(Box<Expr>, Box<Block>, Box<Either<Block, If>>);
