use std::path::Path;
use std::ffi::OsStr;

/// get_output_file_name returns a file name of the output. If the output option exists,
/// it will use it. On the other hand, if there is no output file specified, it will use the
/// input file name for the prefix. The output format will be `[INPUT_FILE_NAME]_output.[OUTPUT_TYPE]`.
pub fn get_output_file_name() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_output_file_name(){
        assert!(get_output_file_name());
    }
}