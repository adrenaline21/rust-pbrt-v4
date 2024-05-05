#[inline]
pub fn red(s: &str) -> String {
    "\x1b[1m\x1b[31m".to_owned() + s + "\x1b[0m"
}
