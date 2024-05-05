use std::fs;

pub fn read_file_contents(filename: &String) -> Vec<u8> {
    // TODO: error_exit
    fs::read(filename).unwrap()
}
