use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;

/// `Source` is a struct for interracting with external sources such as [Materials Project](https://materialsproject.org/).
/// 
/// It is used to get sources such as POSCAR, and other informations using hyper, a HTTP client for rust.
#[derive(Copy, Clone)]
pub struct Source<'a>{
    target_path: &'a Path,
}

impl<'a> Source<'a> {
     /// `new()` initializes the `Source` struct with user input.
     // TODO: Not using target input now. 
    pub fn new(target: &str) -> Self {
        Source {
            target_path: Path::new("~/vasputils/mp")
        }
    }

    /// `initialize_token` actually writes token into `~/vasputils/[TARGET_SOURCE]`.
    /// This token will be used for interacting with external source.
    pub fn initialize_token(&self, token: &str) {
        let file = match File::open(self.target_path) {
            Err(why) => panic!("couldn`t find :{}", why.description()),
            Ok(file) => file,
        };

        let mut writer = BufWriter::new(file);

        writer.write(token.as_bytes()).unwrap();
    }
}

/// `Target` enum is for types of external source.
pub enum Target {
    // MaterialProject is widely used open source database.
    // https://materialsproject.org/
    MaterialsProject,
}

impl FromStr for Target {
    type Err = &'static str;

    /// `from_str` trait converts `&str` into enum field.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MaterialProjects" | "mp" => Ok(Target::MaterialsProject),
            _ => Err("No feature to match"),
        }
    }
}
