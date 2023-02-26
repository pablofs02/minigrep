use minigrep::procesar;
use minigrep::Config;
use std::{env, process};

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::crear(&args).unwrap_or_else(|err| {
		eprintln!("{err}");
		process::exit(1);
	});
	if let Err(e) = procesar(config) {
		eprintln!("{e}");
		process::exit(1);
	}
}
