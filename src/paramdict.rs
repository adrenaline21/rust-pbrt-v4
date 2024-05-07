use std::sync::Arc;

use crate::scene::INTERNED_STRINGS;
use crate::util::colorspace::RGBColorSpace;
use crate::util::spectrum::Spectrum;
use crate::util::string::InternedString;
use crate::util::vecmath::{Point2f, Point3f, Tuple2, Tuple3, Vector3f};
use crate::{util::error::FileLoc, Float};

pub struct ParsedParameter {
    pub type_name: InternedString,
    pub name: InternedString,
    pub loc: FileLoc,
    pub floats: Vec<Float>,
    pub ints: Vec<i32>,
    pub strings: Vec<String>,
    pub bools: Vec<u8>,
    looked_up: bool,
}

impl ParsedParameter {
    pub fn new() -> Self {
        Self {
            type_name: INTERNED_STRINGS.lookup(&String::from("")),
            name: INTERNED_STRINGS.lookup(&String::from("")),
            loc: FileLoc::default(),
            floats: Vec::new(),
            ints: Vec::new(),
            strings: Vec::new(),
            bools: Vec::new(),
            looked_up: false,
        }
    }
}

pub type ParsedParameterVector = Vec<ParsedParameter>;

enum SpectrumType {
    Illuminant,
    Albedo,
    Unbounded,
}

pub trait ParameterTypeTraits {
    const TYPE_NAME: &'static str;
    const N_PER_ITEM: usize;
    type ValueType;
    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized;
    fn convert(v: &[Self::ValueType]) -> Self;
}

impl ParameterTypeTraits for bool {
    const TYPE_NAME: &'static str = "bool";
    const N_PER_ITEM: usize = 1;
    type ValueType = u8;
    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.bools
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        match v[0] {
            0 => false,
            _ => true,
        }
    }
}

impl ParameterTypeTraits for Float {
    const TYPE_NAME: &'static str = "float";

    const N_PER_ITEM: usize = 1;

    type ValueType = Float;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.floats
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        v[0]
    }
}

impl ParameterTypeTraits for i32 {
    const TYPE_NAME: &'static str = "int";

    const N_PER_ITEM: usize = 1;

    type ValueType = i32;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.ints
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        v[0]
    }
}

impl ParameterTypeTraits for Point2f {
    const TYPE_NAME: &'static str = "point2";

    const N_PER_ITEM: usize = 2;

    type ValueType = Float;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.floats
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        Point2f::new(v[0], v[1])
    }
}

impl ParameterTypeTraits for Point3f {
    const TYPE_NAME: &'static str = "point3";

    const N_PER_ITEM: usize = 3;

    type ValueType = Float;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.floats
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        Point3f::new(v[0], v[1], v[2])
    }
}

impl ParameterTypeTraits for Vector3f {
    const TYPE_NAME: &'static str = "vector3";

    const N_PER_ITEM: usize = 3;

    type ValueType = Float;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.floats
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        Vector3f::new(v[0], v[1], v[2])
    }
}

impl ParameterTypeTraits for String {
    const TYPE_NAME: &'static str = "string";

    const N_PER_ITEM: usize = 1;

    type ValueType = String;

    fn get_values(param: &ParsedParameter) -> &[Self::ValueType]
    where
        Self: Sized,
    {
        &param.strings
    }

    fn convert(v: &[Self::ValueType]) -> Self {
        v[0].clone()
    }
}

pub struct ParameterDictionary {
    params: ParsedParameterVector,
    color_space: Option<&'static RGBColorSpace>,
}

impl Default for ParameterDictionary {
    fn default() -> Self {
        Self {
            params: Default::default(),
            color_space: None,
        }
    }
}

pub fn return_array<R, V>(
    values: &[V],
    param: &ParsedParameter,
    n_per_item: usize,
    convert: fn(&[V]) -> R,
) -> Vec<R> {
    if values.is_empty() {
        panic!("No values provided");
    }
    if values.len() % n_per_item != 0 {
        panic!("Number of values provided not a multiple.");
    }
    let n = values.len() / n_per_item;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v[i] = convert(&values[n_per_item * i..n_per_item * (i + 1)])
    }
    v
}

impl ParameterDictionary {
    pub fn new(params: ParsedParameterVector, color_space: &'static RGBColorSpace) -> Self {
        Self {
            params: params,
            color_space: Some(color_space),
        }
    }

    fn get_one_float(&self, name: &str, default: Float) -> Float {
        self.lookup_single(name, default)
    }
    fn get_one_int(&self, name: &str, default: i32) -> i32 {
        self.lookup_single(name, default)
    }
    fn get_one_bool(&self, name: &str, default: bool) -> bool {
        self.lookup_single(name, default)
    }
    fn get_one_point2f(&self, name: &str, default: Point2f) -> Point2f {
        self.lookup_single(name, default)
    }
    fn get_one_vector3f(&self, name: &str, default: Vector3f) -> Vector3f {
        self.lookup_single(name, default)
    }
    fn get_one_string(&self, name: &str, default: String) -> String {
        // TODO: default &String
        self.lookup_single(name, default)
    }
    fn get_one_spectrum<T: Spectrum>(
        &self,
        name: &str,
        default: T,
        spectrum_type: SpectrumType,
    ) -> T {
        for p in &self.params {
            if *p.name != name {
                continue;
            }
        }
        default
    }

    fn get_float_array(&self, name: &str) -> Vec<Float> {
        self.lookup_array::<Float>(name)
    }

    fn lookup_array<T: ParameterTypeTraits>(&self, name: &str) -> Vec<T> {
        for p in &self.params {
            if *p.name == name && *p.type_name == T::TYPE_NAME {
                return return_array(T::get_values(p), p, T::N_PER_ITEM, T::convert);
            }
        }
        vec![]
    }

    fn lookup_single<T>(&self, name: &str, default: T) -> T
    where
        T: ParameterTypeTraits,
    {
        for p in &self.params {
            if *p.name != name && *p.type_name != T::TYPE_NAME {
                continue;
            }
            let values = T::get_values(p);
            if values.is_empty() {
                panic!("No values!");
            }
            if values.len() != T::N_PER_ITEM {
                panic!("Expected {} values", T::N_PER_ITEM);
            }
            return T::convert(values);
        }
        default
    }

    fn extract_spectrum_array<T: Spectrum>(
        &self,
        param: &ParsedParameter,
        spectrum_type: SpectrumType,
    ) -> &[T] {
        if *param.type_name == "rgb" {
        } else if *param.type_name == "blackbody" {
        } else if *param.type_name == "spectrum" && !param.floats.is_empty() {
        } else if *param.type_name == "spectrum" && !param.strings.is_empty() {
        }
        &[]
    }
}
