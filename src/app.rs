extern crate clap;

mod format;
mod utils;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("vasp-utils")
        .version("v0.1.0-beta")
        .author("KeisukeYamashita <19yamashita15@gmail.com>")
        .about("A useful VASP tools to help your research accelerate")
        .args(&[Arg::with_name("debug")
            .help("turn on debug")
            .short("v")
            .long("verbose")]).subcommand(
            SubCommand::with_name("format")
                .about("format input file of vasp into specific type of file")
                .arg(
                    Arg::with_name("input")
                        .help("the input file to format")
                        .short("i")
                        .long("input")
                        .takes_value(true)
                        .required(true),
                ).arg(
                    Arg::with_name("output")
                        .help("specify output file and path")
                        .short("o")
                        .long("output")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("feature")
                        .help("type of feature you what to format into")
                        .short("f")
                        .long("feature")
                        .possible_values(&["FreeEnegry", "fe"])
                        .takes_value(true)
                        .required(true),
                ).arg(
                    Arg::with_name("output type")
                        .help("specify file format of output. Default is plain text.")
                        .short("t")
                        .long("type")
                        .possible_values(&["plainText", "csv"])
                        .takes_value(true),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("format") {
        let input_path_str = matches.value_of("input").unwrap();
        let feature_type = matches.value_of("feature").unwrap();

        let output_path_str = matches.value_of("output");

        let output_type = match matches.value_of("output type") {
            Some(output_type) => output_type,
            None => "plainText",
        };

        let (input_path, output_path) = utils::get_file_paths(input_path_str, output_path_str);

        let formatter = format::Formatter::new(input_path, output_path, output_type);

        formatter.extract_feature(feature_type);
    }
}
