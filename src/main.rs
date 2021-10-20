mod app;
mod bytes;

use std::io::{
	self,
	BufRead,
};

use atty::Stream;

fn run() -> Result<(), &'static str> {
	let m = app::new().get_matches();
	let raw = m.is_present("raw");
	if !atty::is(Stream::Stdin) {
		let stdin = io::stdin();
		for s in stdin.lock().lines() {
			match s {
				Ok(s) => conv(&s, raw),
				Err(e) => eprintln!("error: {}", e),
			};
		}
	} else if !m.is_present("input") {
		return Err("missing required argument: input");
	} else {
		for s in m.values_of("input").unwrap() {
			conv(s, raw);
		}
	}

	Ok(())
}

fn conv(s: &str, raw: bool) {
	match bytes::parse(s) {
		Ok(b) if raw => println!("{}", b.0),
		Ok(b) => println!("{}", b),
		Err(e) => eprintln!("error: {}", e),
	};
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {}", e);
		std::process::exit(2);
	}
}
