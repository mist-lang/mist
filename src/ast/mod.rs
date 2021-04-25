mod compile;
mod parse;

use either::Either;

#[derive(Debug)]
pub struct File {
	pub items: Vec<Item>
}

#[derive(Debug)]
pub struct Ident(String);

#[derive(Debug)]
pub enum Item {
	Fun(Fun),
}

#[derive(Debug)]
pub struct Fun {
	name: Ident,
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
	Num(u64),
}
