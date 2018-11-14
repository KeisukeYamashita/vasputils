extern crate clap;

use clap::{App, Arg, SubCommand};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = App::new("vasp-utils")
        .version("v1.0.0-beta")
        .author("KeisukeYamashita <19yamashita15@gmail.com>")
        .about("A useful VASP tools to help your research accelerate")
        .args(&[Arg::with_name("debug")
            .help("turn on debug")
            .short("v")
            .long("debug")]).subcommand(
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
                        .long("output"),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("format") {
        let input_file_path = matches.value_of("input").unwrap();
        let path = Path::new(input_file_path);
        let display = path.display();

        let mut file = match File::open(input_file_path) {
          Err(why) => panic!("couldn`t find {}: {}", display, why.description()),
          Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
          Err(why) => panic!("couldn`t read {}: {}",display, why.description()),
          Ok(_) => println!("{}", s)
        };
    }
}
