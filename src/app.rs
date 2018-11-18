extern crate clap;

mod format;
mod utils;

use clap::{App, Arg, SubCommand};

use std::error::Error;
use std::fs::File;
use std::io::{ BufWriter, Write, BufRead, BufReader };

fn main() {
    let matches = App::new("vasp-utils")
        .version("v1.0.0-beta")
        .author("KeisukeYamashita <19yamashita15@gmail.com>")
        .about("A useful VASP tools to help your research accelerate")
        .args(&[Arg::with_name("debug")
            .help("turn on debug")
            .short("v")
            .long("verbose")]
        )
        .subcommand(
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
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("format") { 
        // Option `.required(true)` promises that input has a value
        let input_path_str = matches.value_of("input").unwrap();
        let output_path_str = matches.value_of("output");

        let (input_path, output_path) = utils::get_file_paths(input_path_str, output_path_str);

        let formatter = format::Formatter::new(
            input_path, 
            output_path,
        );

        let mut file = match File::open(formatter.input) {
          Err(why) => panic!("couldn`t find :{}", why.description()),
          Ok(file) => file,
        };

        let buffered = BufReader::new(file);

        for line in buffered.lines() {
          if let Ok(s) = line {
            println!("{}", s);
          }
        }

        let mut output_file_path = match File::create(formatter.output) {
          Err(why) => panic!("couldn`t find : {}", why.description()),
          Ok(file) => file,
        };

        let mut output_file = BufWriter::new(output_file_path);
        
        let b = b"hoge";


        output_file.write(b).unwrap();
    }
}
