use std::{env, error::Error, fs};

pub fn procesar(config: Config) -> Result<(), Box<dyn Error>> {
	let contenido = fs::read_to_string(config.ruta)?;
	let resultados = if config.sensible {
		hacer_busqueda_insensible(&config.patron, &contenido)
	} else {
		hacer_busqueda(&config.patron, &contenido)
	};
	for linea in resultados {
		println!("{linea}");
	}
	Ok(())
}

pub struct Config {
	pub patron: String,
	pub ruta: String,
	pub sensible: bool,
}

impl Config {
	pub fn crear(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("faltan argumentos");
		}
		let patron = args[1].clone();
		let ruta = args[2].clone();
		let sensible = env::var("SENSIBLE").is_ok();
		Ok(Config {
			patron,
			ruta,
			sensible,
		})
	}
}

pub fn hacer_busqueda<'a>(patron: &str, contenido: &'a str) -> Vec<&'a str> {
	let mut resultados = Vec::new();
	for linea in contenido.lines() {
		if linea.contains(patron) {
			resultados.push(linea);
		}
	}
	resultados
}

pub fn hacer_busqueda_insensible<'a>(patron: &str, contenido: &'a str) -> Vec<&'a str> {
	let patron = patron.to_lowercase();
	let mut resultado = Vec::new();
	for linea in contenido.lines() {
		if linea.to_lowercase().contains(&patron) {
			resultado.push(linea);
		}
	}
	resultado
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sensible() {
		let patron = "duct";
		let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(
			vec!["safe, fast, productive."],
			hacer_busqueda(patron, contenido)
		);
	}

	#[test]
	fn insensible() {
		let patron = "rUsT";
		let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(
			vec!["Rust:", "Trust me."],
			hacer_busqueda_insensible(patron, contenido)
		);
	}
}
