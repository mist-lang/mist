use super::*;

impl WasmItem for Program {
	fn to_wat(&self, _indent: usize) -> String {
		self.0
			.iter()
			.map(|module| module.to_wat(0))
			.collect::<Vec<_>>()
			.join("\n")
	}
}
