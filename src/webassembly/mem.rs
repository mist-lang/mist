use super::*;

impl WasmItem for Memory {
	fn to_wat(&self, indent: usize) -> String {
		let mut builder = self.indented(indent);
		builder.push_str("(memory ");
		builder.push_str(format!("{}", self.0.0).as_str());
		if let Some(num) = self.0.1 {
			builder.push_str(format!(" {}", num).as_str());
		}
		builder.push(')');
		builder
	}
}
