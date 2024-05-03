pub struct RGBColorSpace {}

pub static sRGB: RGBColorSpace = RGBColorSpace {};

pub fn get_named(name: &String) -> Option<&'static RGBColorSpace> {
    match name.as_str() {
        "sRGB" => Some(&sRGB),
        _ => None,
    }
}
