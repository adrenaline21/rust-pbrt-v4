use std::str;

#[derive(Default)]
pub struct FileLoc {
    filename: &'static str,
    line: i32,
    column: i32,
}
