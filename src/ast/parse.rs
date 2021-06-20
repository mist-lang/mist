use super::*;

use std::fs;
use std::path::Path;

use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;

named!(pub semicolon<&str, &str>, recognize!(tuple!(multispace0, tag!(";"))));

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
			map!(Fun::parse, Item::Fun)
		)
	);
}

impl Type {
	// TODO: `parse` is reserved for parsing type declarations
	// TODO: | map!(separated_list1!(delimited!(multispace0, tag!(","), multispace0), Type::name), Type::Tuple)
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
			delimited!(multispace0, tag!(":"), multispace0),
			Fun::ty,
			delimited!(multispace0, tag!("="), multispace0),
			Expr::parse,
			preceded!(multispace0, semicolon)
		), |(_, name, _, arrow, _, expr, _)| Fun {
			name,
			arrow,
			eval: Either::Left(expr),
		})
	);

	// name1 Type1 name2 Type2 Type3 Type4 Type5
	// ^========= pt 1 ======^ ^====2====^ ^=3=^
	// (name1: Type1) -> (name2: Type2) -> Type3 -> Type4 -> Type5
	// (name1: Type1) ->
	// Type5
	// TODO: ensure that parts 1 and 2 function correctly,
	// we know that the last part works because `fun main: int` parses
	// correctly but we don't know if parameters work
	named!(ty<&str, Vec<(Option<Ident>, Type)>>,
		map!(tuple!(
			separated_list0!(
				delimited!(multispace0, tag!("->"), multispace0),
				Fun::param
			),
			many0!(terminated!(
				Type::name,
				delimited!(multispace0, tag!("->"), multispace0)
			)),
			Type::name
		), |(params, arrow_tys, final_ret)| {
			params.into_iter()
				.map(|(name, ty)| (Some(name), ty))
				.chain(arrow_tys.into_iter().map(|ty| (None, ty)))
				.chain(Some((None, final_ret)))
				.collect()
		})
	);

	// (name1: Type1)
	// (name2: Type2, name3: Type3)
	// (name4: Type2,)
	named!(param<&str, (Ident, Type)>,
		map!(
			delimited!(
				tuple!(tag!("("), multispace0),
				separated_list0!(
					tag!(","),
					delimited!(
						multispace0,
						tuple!(
							Ident::parse,
							delimited!(multispace0, tag!(":"), multispace0),
							Type::name
						),
						multispace0
					)
				),
				tuple!(multispace0, tag!(")")
			)
		), |_| todo!())
	);

	named!(fn_arrow<&str, Vec<Type>>,
		separated_list1!(
			delimited!(multispace0, tag!("->"), multispace0),
			Type::name
		)
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
			| map!(Expr::call, |(name, args)| Expr::Call(name, args))
			| map!(Ident::parse, Expr::VarRef)
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

	named!(call<&str, (Ident, Vec<Expr>)>,
		tuple!(
			Ident::parse,
			many0!(
				preceded!(
					multispace1,
					Expr::parse
				)
			)
		)
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
