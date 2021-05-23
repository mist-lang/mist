// use super::*;

// pub type TypeError = String;
// type Result<T> = std::result::Result<T, TypeError>;

// impl Type {
// 	pub fn constrain(&mut self, other: &Type) -> Result<()> {
// 		use Type::*;
// 		match (self, other) {
// 			(Int, Int) | (Bool, Bool) => Ok(()),
// 			// if every item in this can be constrained to every item in that, then the tuples are compatible
// 			(Tuple(my_types), Tuple(other_types)) => my_types
// 				.iter_mut()
// 				.zip(other_types.iter())
// 				.map(|(mine, other)| mine.constrain(other))
// 				.collect(),
// 			(this, that) => Err(format!("{} is not compatible with {}", this, that)),
// 		}
// 	}
// }

// impl std::fmt::Display for Type {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		use Type::*;
// 		match self {
// 			Int => write!(f, "int"),
// 			Bool => write!(f, "bool"),
// 			Tuple(items) => write!(f, "({})", items.iter().map(|item| format!("{}", item)).collect::<Vec<_>>().join(", ")),
// 		}
// 	}
// }

// impl File {
// 	pub fn type_check(&mut self) -> Result<()> {
// 		self.items.iter_mut().map(Item::type_check).collect()
// 	}
// }

// impl Item {
// 	fn type_check(&mut self) -> Result<()> {
// 		use Item::*;
// 		match self {
// 			Fun(fun) => fun.write().unwrap().type_check(),
// 		}
// 	}
// }

// impl Fun {
// 	fn type_check(&mut self) -> Result<()> {
// 		let eval_ty = match &mut self.eval {
// 			Either::Left(expr) => expr.type_check(),
// 			Either::Right(block) => block.type_check(),
// 		}?;
// 		self.out_ty.constrain(&eval_ty)
// 	}
// }

// impl Block {
// 	fn type_check(&mut self) -> Result<Type> {
// 		// TODO: type-check statements
// 		match &mut self.expr {
// 			Some(expr) => expr.type_check(),
// 			// (), or Unit, or void, or whatever you want to call it
// 			None => Ok(Type::Tuple(vec![])),
// 		}
// 	}
// }

// impl Expr {
// 	fn type_check(&mut self) -> Result<Type> {
// 		use Expr::*;
// 		match self {
// 			Bool(_) => Ok(Type::Bool),
// 			Int(_) => Ok(Type::Int),
// 			If(if_expr) => if_expr.type_check(),
// 			_ => todo!(),
// 		}
// 	}
// }

// impl If {
// 	pub fn type_check(&mut self) -> Result<Type> {
// 		// x
// 		self.0.type_check()?.constrain(&Type::Bool)?;
// 		// y
// 		let mut then_ty = self.1.type_check()?;
// 		// z
// 		let mut else_ty = match self.2.as_mut() {
// 			Either::Left(block) => block.type_check(),
// 			Either::Right(if_expr) => if_expr.type_check(),
// 		}?;
// 		// if y > z then y' else y
// 		then_ty.constrain(&else_ty)?;
// 		// if z > y then z' else z
// 		else_ty.constrain(&then_ty)?;
// 		// y'
// 		Ok(then_ty)
// 	}
// }
