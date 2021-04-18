use std::fs;
use std::path::Path;

use either::Either;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::space0;

#[derive(Debug)]
pub struct File {
	pub items: Vec<Item>
}

impl File {
	pub fn read(path: impl AsRef<Path>) -> Result<Self, String> {
		let contents = match fs::read_to_string(path) {
			Ok(contents) => contents,
			Err(ee) => return Err(format!("Couldn't open file: {}", ee)),
		};
		match File::parse(contents.as_str()) {
			Ok((_rest, file)) => Ok(file),
			Err(ee) => {
				dbg!(&ee);
				Err(format!("Couldn't parse: {}", ee))
			},
		}
	}

	named!(pub parse<&str, File>,
		map!(many0!(complete!(Item::parse)), |items| File { items })
	);
}

#[derive(Debug)]
pub struct Ident(String);

impl Ident {
	named!(pub parse<&str, Ident>, map!(
		recognize!(
			preceded!(
				alt!(tag!("_") | alpha1),
				alt!(tag!("_") | alphanumeric0)
			)
		), |ident| Ident(String::from(ident))
	));
}

#[derive(Debug)]
pub enum Item {
	Fun(Fun),
}

impl Item {
	named!(pub parse<&str, Item>,
		alt!(
			map!(Fun::parse, Item::Fun)
		)
	);
}

#[derive(Debug)]
pub struct Fun {
	name: Ident,
	eval: Either<Expr, Block>,
}

impl Fun {
	named!(pub parse<&str, Fun>,
		map!(tuple!(
			recognize!(tuple!(multispace0, tag!("fun"), space0)),
			Ident::parse,
			recognize!(tuple!(space0, tag!("="), space0)),
			Expr::parse,
			recognize!(tuple!(space0, tag!(";")))
		), Fun::from_assignfn_parts)
	);
}

impl Fun {
	pub fn from_assignfn_parts((_fun, name, _eq, expr, _semi): (&str, Ident, &str, Expr, &str)) -> Self {
		Fun {
			name,
			eval: Either::Left(expr),
		}
	}
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

impl Expr {
	named!(pub parse<&str, Expr>,
		alt!(
			map!(Expr::num, Expr::Num)
		)
	);

	named!(num<&str, u64>,
		map!(digit1, |input| input.parse::<u64>().unwrap())
	);
}
