#[macro_use]
extern crate clap;

use std::process::exit;

fn main() {
	let matches = clap_app!(mistc =>
		(version: "1.0")
		(about: "Compiler for the Mist programming language.")
		(@arg INPUT: +required "Mist file to compile")
	).get_matches();

	let input_file = matches.value_of("INPUT").expect("No file argument passed?");
	let _parsed = stream::ast::File::read(input_file).unwrap_or_else(|err| {
		println!("Failed to parse file:\n{}", err);
		exit(1);
	});
	dbg!(_parsed);
}
