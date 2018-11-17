use std::path::Path;

/// `Formatter` represents broker for user and  this command line interface.
/// 
/// This is like a handler, if you are familier with web application. It contains informations such as
/// `input file`, `output file` and the `action` from the user input.
pub struct Formatter<'a> {
    /// `input` represents field for user input file path.
    pub input: &'a Path,

    // `output` represent field for user output file path.
    pub output: &'a Path
}

impl<'a> Formatter<'a> {
    /// `new()` initializes the `Formatter` struct with user input.
    pub fn new(input_path: &'a Path, output_path: &'a Path) -> Self {
        Formatter {
            input: input_path,
            output: output_path
        }
    }
}