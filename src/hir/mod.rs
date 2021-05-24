mod builtin;
mod scope;
mod type_check;

use std::sync::Arc;
use std::sync::RwLock;

pub use scope::Scope;

use once_cell::sync::Lazy;

pub static PROGRAM: Lazy<Program> = Lazy::new(|| todo!());

pub struct Ident(String);

pub struct Program {
	root_scope: Scope,
	pub main_scope: Scope,
}

pub struct Block(Scope, Vec<Stmt>);

pub enum Const {
	Int(u64),
	Bool(bool),
}

pub enum Type {
	Int,
	Bool,
}

pub enum Item {
	Fun(Fun),
	Var(Dec),
}

pub struct Fun(Arc<RwLock<(Vec<Dec>, Block)>>);

impl Fun {
	pub fn new(scope: Scope, params: Vec<Dec>, stmts: Vec<Stmt>) -> Fun {
		Fun(Arc::new(RwLock::new((params, Block(scope, stmts)))))
	}
}

pub struct Dec(Arc<(Option<String>, RwLock<Type>)>);

impl Dec {
	pub fn new(name: String, ty: Type) -> Dec {
		Dec(Arc::new((Some(name), RwLock::new(ty))))
	}

	pub fn gen(ty: Type) -> Dec {
		Dec::new("".to_string(), ty)
	}
}

pub enum Stmt {
	Let(Dec, Expr),
	Reassign(Dec, Expr),
}

pub enum Expr {
	Call(Fun, Vec<Ref>),
	If(Vec<(Ref, Block)>),
	Ref(Ref),
}

pub struct Ref();
