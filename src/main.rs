#[macro_use]
extern crate clap;

use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

use mist::webassembly::WasmItem;

fn main() {
	let matches = clap_app!(mistc =>
		(version: "1.0")
		(about: "Compiler for the Mist programming language.")
		(@arg INPUT: +required "Mist file to compile")
		(@arg OUTPUT: +required "WebAssembly (.wat) file to output")
	).get_matches();

	let input_file = matches.value_of("INPUT").expect("No input file argument passed?");
	let parsed = mist::ast::File::read(input_file).unwrap_or_else(|err| {
		println!("Failed to parse file:\n{}", err);
		exit(1);
	});
	let hir = parsed.compile_hir().unwrap_or_else(|err| {
		println!("Failed to load HIR:\n{}", err);
		exit(1);
	});
	hir.type_check().unwrap_or_else(|err| {
		println!("Failed to type-check:\n{}", err);
		exit(1);
	});
	let wat_ast = hir.to_wasm().unwrap_or_else(|err| {
		println!("Failed to load WebAssembly:\n{}", err);
		exit(1);
	});
	let output_file = matches.value_of("OUTPUT").expect("No output file argument passed?");
	let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(output_file).unwrap_or_else(|err| {
		println!("Failed to open file {}:\n{}", output_file, err);
		exit(1);
	});
	let wat_string = wat_ast.to_wat(0);
	file.write_all(wat_string.as_bytes()).unwrap_or_else(|err| {
		println!("Failed to write to file {}:\n{}", output_file, err);
		exit(1);
	});
}
