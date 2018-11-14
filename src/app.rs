extern crate clap;
use clap::App;

fn main() {
  App::new("vasp-utils")
                    .version("v1.0-beta")
                    .author("KeisukeYamashita <19yamashita15@gmail.com>")
                    .about("A useful VASP tools to help your research accelerate")
                    .get_matches();
  println!("hoeg");
}