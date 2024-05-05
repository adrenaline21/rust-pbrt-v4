use std::str;

use super::print::red;

#[derive(Default, Clone)]
pub struct FileLoc {
    pub filename: String,
    pub line: i32,
    pub column: i32,
}

impl FileLoc {
    pub fn new(filename: String) -> Self {
        let mut loc = FileLoc::default();
        loc.filename = filename;
        loc
    }

    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.filename, self.line, self.column)
    }
}

fn process_error(error_type: &str, loc: Option<&FileLoc>, message: &String) {
    let mut error_string = red(error_type);
    if loc.is_some() {
        error_string += &(": ".to_owned() + &loc.unwrap().to_string());
    }
    error_string += &(": ".to_owned() + message);
    // TODO: multi-thread mutex
    print!("{}", error_string);
    std::process::exit(1)
}

pub fn error_exit(loc: Option<&FileLoc>, message: &String) {
    process_error("Error", loc, message);
}
