use clap::{
	crate_version,
	App,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("bytes")
		.about("Pretty print byte values.")
		.after_help("Input can be supplied by piping to this command.")
		.version(crate_version!());

	let raw = Arg::new("raw")
		.about("Display the values in bytes, do not add the 'b' suffix.")
		.short('r')
		.long("raw")
		.short_alias('b');

	let precision = Arg::new("precision")
		.about("Floating point precision used while displaying.")
		.short('p')
		.long("precision")
		.default_value("2")
		.validator(|s| {
			s.parse::<usize>()
				.map(|_| {})
				.map_err(|_| String::from("the value must be a non-negative integer"))
		});

	let input = Arg::new("input").value_name("N").multiple_values(true);

	app.arg(raw).arg(precision).arg(input)
}
