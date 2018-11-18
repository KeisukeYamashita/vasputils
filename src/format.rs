use std::any::Any;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::str::FromStr;

/// `Formatter` represents broker for user and  this command line interface.
///
/// This is like a handler, if you are familier with web application. It contains informations such as
/// `input file`, `output file` and the `action` from the user input.
#[derive(Copy, Clone)]
pub struct Formatter<'a> {
    /// `input` represents field for user input file path.
    input: &'a Path,

    // `output` represent field for user output file path.
    output: &'a Path,

    // `output_type` is the output file format.
    output_type: &'a str,
}

impl<'a> Formatter<'a> {
    /// `new()` initializes the `Formatter` struct with user input.
    pub fn new(input_path: &'a Path, output_path: &'a Path, output_type: &'a str) -> Self {
        Formatter {
            input: input_path,
            output: output_path,
            output_type: output_type,
        }
    }

    /// `extract_feature` creates a new file of extracted infomation from the input file.
    pub fn extract_feature(&self, what: &str) {
        let feature_type: Feature = match Feature::from_str(what) {
            Ok(v) => v,
            Err(why) => panic!("failed to get type of feature, err: {}", why),
        };

        self.exec_extraction(feature_type);
    }

    /// `exec_extraction` routes the action.
    fn exec_extraction(&self, feature_type: Feature) {
        let input_file = match File::open(self.input) {
            Err(why) => panic!("couldn`t find :{}", why.description()),
            Ok(file) => file,
        };

        let output_file_path = match File::create(self.output) {
            Err(why) => panic!("couldn`t find : {}", why.description()),
            Ok(file) => file,
        };

        let mut output_file = BufWriter::new(output_file_path);
        let buffered = BufReader::new(input_file);

        let grep_text = Self::get_grep_text(feature_type);

        for line in buffered.lines() {
            if let Ok(line) = line {
                if line.contains(grep_text) {
                    let line = self.process(line);
                    output_file.write(line.as_bytes()).unwrap();
                }
            }
        }
    }

    /// `process` actually processes the line from the input.
    fn process(&self, line: String) -> String {
        return match self.output_type {
            "plainText" => self.process_plain_text(line),
            "csv" => self.process_csv(line),
            _ => panic!("not match"),
        };
    }

    /// `process_plain_text` processes greps for plain text.
    fn process_plain_text(&self, line: String) -> String {
        let line_with_new_line = line + "\n";
        return line_with_new_line;
    }

    /// `process_csv` processes grep and converts into csv.
    fn process_csv(&self, line: String) -> String {
        let mut result: String = "".to_string();
        let elements: Vec<&str> = line.split_whitespace().collect();

        for element in &elements {
            if *element != " " {
                if *element == elements[4] {
                    result = result + *element + "\n";
                }
            }
        }

        return result;
    }

    /// `get_grep_text` returns text use for grep. The alternative method from bash is, for example,
    ///
    /// ```shell
    /// grep "free " OUTCAR > OUTCAR_o
    /// ```
    ///
    fn get_grep_text<'b>(feature_type: Feature) -> &'b str {
        // `extract feature` promised that it will match, so there is no default _ in match.
        return match feature_type {
            Feature::FreeEnergy => "free",
        };
    }

    /// `is_validate_input` checks wether the input file exists.
    fn is_validate_input(&self) -> bool {
        return match File::open(self.input) {
            Err(_) => false,
            Ok(_) => true,
        };
    }
}

/// `Feature` enum is for types of feature which is used for formatting.
pub enum Feature {
    // FreeEnergy is usally used for checking convergence.
    FreeEnergy,
}

impl FromStr for Feature {
    type Err = &'static str;

    /// `from_str` trait converts `&str` into enum field.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FreeEnergy" | "fe" => Ok(Feature::FreeEnergy),
            _ => Err("No feature to match"),
        }
    }
}
