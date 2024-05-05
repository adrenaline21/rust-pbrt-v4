use std::fs;

pub fn read_file_contents(filename: &String) -> String {
    // TODO: error_exit
    fs::read_to_string(filename).unwrap()
}
