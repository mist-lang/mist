use super::*;

use std::fs;
use std::path::Path;

use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;

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

impl Item {
	named!(pub parse<&str, Item>,
		alt!(
			map!(Fun::parse, |fun| Item::Fun(Box::new(fun)))
		)
	);
}

impl Type {
	// TODO: `parse` is reserved for parsing type declarations

	named!(pub name<&str, Type>,
		alt!(
			value!(Type::Int, tag!("int"))
			| value!(Type::Bool, tag!("bool"))
		)
	);
}

impl Fun {
	named!(pub parse<&str, Fun>,
		map!(tuple!(
			delimited!(multispace0, tag!("fun"), multispace0),
			Ident::parse,
			preceded!(pair!(tag!(":"), multispace0), Type::name),
			delimited!(multispace0, tag!("="), multispace0),
			Expr::parse,
			recognize!(tuple!(multispace0, tag!(";")))
		), |(_, name, out_ty, _, expr, _)| Fun {
			name,
			out_ty,
			eval: Either::Left(expr),
		})
	);
}

impl Block {
	named!(pub parse<&str, Block>,
		map!(
			tuple!(
				tag!("{"),
				multispace0,
				Expr::parse,
				multispace0,
				tag!("}")
			),
			|(_, _, expr, _, _)| Block {
				stmts: Vec::new(),
				expr: Some(expr),
			}
		)
	);
}

impl Expr {
	named!(pub parse<&str, Expr>,
		alt!(
			map!(Expr::boolean, Expr::Bool)
			| map!(Expr::int, Expr::Int)
			| map!(If::parse, Expr::If)
		)
	);

	named!(boolean<&str, bool>,
		alt!(
			value!(true, tag!("true"))
			| value!(false, tag!("false"))
		)
	);

	named!(int<&str, u64>,
		map!(digit1, |input| input.parse::<u64>().unwrap())
	);
}

impl If {
	named!(parse<&str, If>,
		map!(
			tuple!(
				tag!("if"),
				multispace1,
				Expr::parse,
				delimited!(multispace0, Block::parse, multispace0),
				tag!("else"),
				multispace1,
				alt!(
					map!(If::parse, Either::Right)
					| map!(Block::parse, Either::Left)
				)
			), |(_, _, cond, then, _, _, el)| If(
				Box::new(cond),
				Box::new(then),
				Box::new(el)
			)
		)
	);
}
