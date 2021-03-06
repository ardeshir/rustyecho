use clap::{App, Arg};

fn main() {
   let matches = App::new("echor")
	.version("0.0.1")
	.author("Ardeshir <ardeshir.org@gmail.com>")
	.about("Rust toy echo")
	.arg(
	   Arg::with_name("text")
		.value_name("TEXT")
		.help("Input text")
		.required(true)
		.min_values(1),
        )
	.arg(
	   Arg::with_name("omit_newline")
		.short("n")
		.help("Do not print newline")
		.takes_value(false),
	)
	.get_matches();

    println!("{:#?}", matches);
}

