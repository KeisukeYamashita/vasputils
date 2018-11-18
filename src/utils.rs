use std::path::Path;

/// get_output_file_name returns a file name of the output. If the output option exists,
/// it will use it. On the other hand, if there is no output file specified, it will use the
/// input file name for the prefix. The output format will be `[INPUT_FILE_NAME]_output.[OUTPUT_TYPE]`.
pub fn get_file_paths<'a, 'b>(
    input_path_str: &'a str,
    output_file_str: Option<&'b str>,
) -> (&'a Path, &'b Path) {
    let output_path: &Path;

    let input_path = Path::new(input_path_str);

    if let Some(output_file_str) = output_file_str {
        output_path = Path::new(output_file_str);
        return (input_path, output_path);
    }

    // TODO: If there is no output specified, it should return something else.
    output_path = Path::new("./OUTCAR_o");

    return (input_path, output_path);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_output_file_name() {
        let input_path = "./this/is/test/input/path/OUTCAR";
        let output_path = None;
        let (input_file, output_file) = get_file_paths(input_path, output_path);
        assert_eq!(input_file, Path::new(input_path));
        assert_eq!(output_file, Path::new("./OUTCAR_o"));
    }
}
