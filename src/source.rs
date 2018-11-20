extern crate reqwest;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::path::{PathBuf};
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, Read};
use std::str::FromStr;
use self::serde_json::{Value};

/// `Source` is a struct for interracting with external sources such as [Materials Project](https://materialsproject.org/).
/// 
/// It is used to get sources such as POSCAR, and other informations using hyper, a HTTP client for rust.
#[derive(Serialize, Deserialize, Debug)] 
pub struct Source{
    target_path: PathBuf
}

impl Source{
     /// `new()` initializes the `Source` struct with user input.
    pub fn new(target: &str) -> Self {
        let mut path = dirs::home_dir().unwrap();
        path.push("vasputils");
        path.push(target);

        Source {
            target_path: path
        }
    }

    /// `initialize_token` actually writes token into `~/vasputils/[TARGET_SOURCE]`.
    /// This token will be used for interacting with external source.
    pub fn initialize_token(self, token: &str) {
        let file = match File::create(&self.target_path) {
            Err(why) => panic!("couldn`t find {:?}, err: {}", self.target_path, why.description()),
            Ok(file) => file,
        };

        let mut writer = BufWriter::new(file);

        writer.write(token.as_bytes()).unwrap();
    }

    /// `fetch_file()` method strout responce from external resource.
    pub fn fetch_file(self, material: &str){
        let file = match File::open(&self.target_path) {
            Ok(file) => file,
            Err(why) => panic!("cannot open {:?}, err: {}", self.target_path, why.description())
        };

        let ref mut token = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(token).unwrap();
        
        let url: String = "https://www.materialsproject.org/rest/v2/materials/".to_string() + "/" + material + "/vasp" + "?API_KEY=" + token; 
        let body = match reqwest::get(url.as_str()) {
            Ok(mut req) => req.text().unwrap(),
            Err(why) => panic!("error while requesting to the API, err: {}", why.description())
        };

        let v: Value = match serde_json::from_str(body.as_str()) {
            Ok(v) => v,
            Err(why) => panic!("error while parsing, err: {}", why.description())
        };

        println!("{:?}", v);
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