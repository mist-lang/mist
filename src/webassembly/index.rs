use std::fmt::Display;
use super::*;

impl Display for Index {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Index::None => Ok(()),
			Index::Name(name) => {
				f.write_str("$")?;
				f.write_str(name)
			},
			&Index::Number(num) => write!(f, "{}", num),
		}
	}
}
