use std::path::Path;
use std::ffi::OsStr;

/// get_output_file_name returns a file name of the output. If the output option exists,
/// it will use it. On the other hand, if there is no output file specified, it will use the
/// input file name for the prefix. The output format will be `[INPUT_FILE_NAME]_output.[OUTPUT_TYPE]`.
pub fn get_output_file_name(input_path_str: &str, output_file_str: Option<&str>) -> (&Path, &Path) {
    let output_path: &Path;
    
    let input_path = Path::new(input_path_str);
    
    if let Some(output_file_str) = output_file_str {;
        output_path = Path::new(output_file_str);
        return (input_path, output_path);
    }

    let input_file_ancestors = match input_path.ancestors().next() {
        None => panic!("there is no ancestors"),
        Some(ancestor) => ancestor
    };

    let input_file_name = match input_path.file_stem(){
        None => panic!("there is no input file"),
        Some(file) => file
    };



    return (input_path, output_path);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_output_file_name(){
        let input_path = "./this/is/test/input/path/OUTCAR";
        let output_path = Some(".");
        assert!(get_output_file_name(input_path, output_path));
    }
}