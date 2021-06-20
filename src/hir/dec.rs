use super::Dec;
use super::DecInner;
use super::Type;

use std::sync::Arc;

impl Dec {
	pub fn new(name: impl ToString, ty: Type) -> Self {
		Dec(Arc::new(DecInner {
			name: name.to_string(),
			ty,
		}))
	}

	pub fn name(&self) -> &str {
		self.0.name.as_str()
	}

	pub fn ty(&self) -> &Type {
		&self.0.ty
	}
}
