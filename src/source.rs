use std::path::Path;
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
    pub fn new(target: &str) -> Self {
        Source {
            target_path: Path::new("~/vasputils/mp")
        }
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
