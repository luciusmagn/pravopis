//! https://www.rust-lang.org/
//! spouštět přes:
//! ```
//! cargo run --release
//! ```
//! jinak se bude pouštět debug build a to je výrazně pomalejší
extern crate levenshtein; // Levenshteinova vzdálenost
extern crate rayon; // paralelní iterátory

use rayon::prelude::*;
use levenshtein::levenshtein;

use std::fs::File;
use std::env::args;
use std::process::exit;
use std::io::{Write, BufReader, BufRead};

// funkce pro správné zabití když se něco nepovede
fn error(msg: &str) -> ! {
	eprintln!("{}", msg);
	exit(-1)
}

fn main() {
	let mut args = args().skip(1);

	// program bere 0-2 argumenty
	// 1. název souboru se slovníkem, jinak slovnik.txt
	// 2. název souboru s levopisem, jinak levopis.txt
	// opravy program také vypisuje, takže lze přesměrovat stdout do osuboru,
	// pokud by byla jiný výstup než opravy.txt žádoucí
	let slovnik_name =
		if let Some(s) = args.next() { s }
		else { "slovnik.txt".to_string() }
	;

	let text_name =
		if let Some(s) = args.next() { s }
		else { "levopis.txt".to_string() }
	;

	// Soubory
	let slovnik = File::open(slovnik_name)
		.unwrap_or_else(|_| error("nepodařilo se vytvořit soubor se slovníkem"));

	let text = File::open(text_name)
		.unwrap_or_else(|_| error("nepodařilo se otevřít soubor s levopisem"));

	let mut opravy = File::create("opravy.txt")
		.unwrap_or_else(|_| error("nepodařilo se vytvořit soubor pro opravy"));

	let slovnik: Box<Vec<String>> = Box::new(BufReader::new(slovnik)
		.lines()
		// Iterator<Result<String>>
		.filter(|x| x.is_ok())
		// Iterator<String>
		.map(|x| x.unwrap())
		// Vec<String>
		.collect());


	BufReader::new(text)
		.lines()
		// Iterator<Result<String>>
		// (např. protože by obsahovala znaky, které nelze zkonvertovat do UTF-8)
		// tak je potichu vynecháme, to samé u předchozího BufReaderu
		.filter(|x| x.is_ok())
		// Iterator<String>, pokud nelze přečíst nějakou řádku
		.map(|x| x.unwrap())
		// Iterator<Vec<String>, uvnitř paralelní iterátor pro hledání podobných slov (viz par_iter z rayonu)
		.map(|x| slovnik.par_iter().filter(|w| levenshtein(&x, w) == 1).cloned().collect::<Vec<String>>())
		// Iterator<String>
		.map(|x| x.join(" "))
		// Aby se to vypisovalo i do stdout
		.inspect(|x| println!("{}", x))
		// Zapsání do souboru opravy.txt
		.for_each(|x| { let _ = writeln!(opravy, "{}", x); });
}
