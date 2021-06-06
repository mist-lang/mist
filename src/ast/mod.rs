mod parse;
mod load_hir;

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

#[derive(Clone, Debug)]
pub enum Type {
	Int,
	Bool,
}

#[derive(Debug)]
pub struct Fun {
	name: Ident,
	params: Vec<Var>,
	out_ty: Type,
	eval: Either<Expr, Block>,
}

#[derive(Debug)]
pub struct Var(Ident, Type);

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
	VarRef(Ident),
	If(If),
	Call(Ident, Vec<Expr>),
}

#[derive(Debug)]
pub struct If(Box<Expr>, Box<Block>, Box<Either<Block, If>>);
