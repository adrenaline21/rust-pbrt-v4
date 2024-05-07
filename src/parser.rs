use std::mem::replace;

use crate::{
    paramdict::{ParsedParameter, ParsedParameterVector},
    scene::INTERNED_STRINGS,
    util::{
        error::{error_exit, FileLoc},
        file::read_file_contents,
    },
    Float,
};

pub trait ParserTarget {
    fn scale(&mut self, sx: Float, sy: Float, sz: Float, loc: FileLoc);

    fn shape(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);

    fn option(&mut self, name: &String, value: &String, loc: FileLoc);

    fn identity(&mut self, loc: FileLoc);
    fn translate(&mut self, dx: Float, dy: Float, dz: Float, loc: FileLoc);
    fn rotate(&mut self, angle: Float, ax: Float, ay: Float, az: Float, loc: FileLoc);
    fn look_at(
        &mut self,
        ex: Float,
        ey: Float,
        ez: Float,
        lx: Float,
        ly: Float,
        lz: Float,
        ux: Float,
        uy: Float,
        uz: Float,
        loc: FileLoc,
    );
    fn concat_transform(&mut self, transform: [Float; 16], loc: FileLoc);
    fn transform(&mut self, transform: [Float; 16], loc: FileLoc);
    fn coordinate_system(&mut self, name: &String, loc: FileLoc);
    fn coord_sys_transform(&mut self, name: &String, loc: FileLoc);
    fn active_transform_all(&mut self, loc: FileLoc);
    fn active_transform_end_time(&mut self, loc: FileLoc);
    fn active_transform_start_time(&mut self, loc: FileLoc);
    fn transform_times(&mut self, start: Float, end: Float, loc: FileLoc);

    fn color_space(&mut self, name: &String, loc: FileLoc);
    fn pixel_filter(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn film(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn accelerator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn integrator(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn camera(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn make_named_medium(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn medium_interface(&mut self, inside_name: &String, outside_name: &String, loc: FileLoc);
    fn sampler(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);

    fn world_begin(&mut self, loc: FileLoc);
    fn attribute_begin(&mut self, loc: FileLoc);
    fn attribute_end(&mut self, loc: FileLoc);
    fn attribute(&mut self, target: &str, params: ParsedParameterVector, loc: FileLoc);
    fn texture(
        &mut self,
        name: &String,
        typename: &String,
        texname: &String,
        params: ParsedParameterVector,
        loc: FileLoc,
    );
    fn material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn make_named_material(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn named_material(&mut self, name: &String, loc: FileLoc);
    fn light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn area_light_source(&mut self, name: &String, params: ParsedParameterVector, loc: FileLoc);
    fn reverse_orientation(&mut self, loc: FileLoc);
    fn object_begin(&mut self, name: &String, loc: FileLoc);
    fn object_end(&mut self, loc: FileLoc);
    fn object_instance(&mut self, name: &String, loc: FileLoc);

    fn end_of_files(&mut self);

    // fn error_exit_deferred(&mut self);
    // error_exit
}

#[derive(Default)]
struct Token<'a> {
    loc: FileLoc,
    token: &'a [u8],
}

impl ParsedParameter {
    fn add_float(&mut self, v: Float) {
        assert!(self.ints.is_empty() && self.strings.is_empty() && self.bools.is_empty());
        self.floats.push(v);
    }

    fn add_int(&mut self, i: i32) {
        assert!(self.floats.is_empty() && self.strings.is_empty() && self.bools.is_empty());
        self.ints.push(i);
    }

    fn add_string(&mut self, s: String) {
        assert!(self.floats.is_empty() && self.strings.is_empty() && self.strings.is_empty());
        self.strings.push(s.to_string());
    }

    fn add_bool(&mut self, v: bool) {
        assert!(self.floats.is_empty() && self.ints.is_empty() && self.strings.is_empty());
        self.bools.push(v as u8);
    }
}

// fn parse_int(t: Token) -> i32 {
//     let negate = t.token[0] == b'-';
//     let mut index = 0;
//     if t.token[0] == b'-' || t.token[0] == b'+' {
//         index += 1;
//     }
//     let mut value: i32 = 0;
//     while index < t.token.len() {
//         let n = t.token[index] as i32 - 48;
//         if !(n >= 0 && n <= 9) {
//             error_exit(Some(&t.loc), &format!("\"{}\": expected a number", n));
//         }
//         value = 10 * value + n;
//     }
//     if negate {
//         -value
//     } else {
//         value
//     }
// }

#[inline]
fn is_quoted_string(str: &[u8]) -> bool {
    str.len() >= 2 && str[0] == b'"' && str[str.len() - 1] == b'"'
}

fn dequote_string(t: &Token) -> String {
    if !is_quoted_string(t.token) {
        error_exit(
            Some(&t.loc),
            &format!(
                "\"{}\": expected quoted string",
                std::str::from_utf8(t.token).unwrap()
            ),
        );
    }
    String::from_utf8(t.token[1..t.token.len() - 1].to_vec()).unwrap()
}

const TOKEN_OPTIONAL: i32 = 0;
const TOKEN_REQUIRED: i32 = 1;

#[inline]
fn unget<'a>(unget_token: &mut Option<Token<'a>>, t: Token<'a>) {
    *unget_token = Some(t);
}

// All next_token are the same?
fn parse_parameters<'a>(
    file_stack: &mut Vec<Tokenizer<'a>>,
    unget_token: &mut Option<Token<'a>>,
) -> ParsedParameterVector {
    let parse_error = |msg: &str, loc: &FileLoc| error_exit(Some(loc), &format!("{}", msg));
    let mut parameter_vector = ParsedParameterVector::new();
    let error_call_back = |t: &Token, msg: &str| {
        parse_error(
            &format!("{}:{}", std::str::from_utf8(t.token).unwrap(), msg),
            &t.loc,
        )
    };
    loop {
        let t = next_token(file_stack, unget_token, TOKEN_OPTIONAL);

        if t.is_none() {
            return parameter_vector;
        }
        let t = t.unwrap();
        if !is_quoted_string(t.token) {
            unget(unget_token, t);
            return parameter_vector;
        }

        let mut param = ParsedParameter::new();
        let decl = dequote_string(&t);
        let mut iter = decl.split_whitespace();
        param.type_name = INTERNED_STRINGS.lookup(&iter.next().unwrap().to_string());
        param.name = INTERNED_STRINGS.lookup(&iter.next().unwrap().to_string());

        // println!("{},{}", param.type_name, param.name);

        #[derive(PartialEq)]
        enum ValType {
            Unknown,
            String,
            Bool,
            Float,
            Int,
        }

        let mut val_type = ValType::Unknown;
        if *param.type_name == "integer" {
            val_type = ValType::Int;
        }

        let val = next_token(file_stack, unget_token, TOKEN_REQUIRED).unwrap();

        let mut add_val = |t: Token| {
            if is_quoted_string(t.token) {
                match val_type {
                    ValType::Unknown => val_type = ValType::String,
                    ValType::String => {}
                    ValType::Float => error_call_back(&t, "expected floating-point value"),
                    ValType::Int => error_call_back(&t, "expected integer value"),
                    ValType::Bool => error_call_back(&t, "expected Boolean value"),
                }
                param.add_string(dequote_string(&t));
            } else if t.token[0] == b't' && t.token == b"true" {
                match val_type {
                    ValType::Unknown => val_type = ValType::Bool,
                    ValType::String => error_call_back(&t, "expected string value"),
                    ValType::Float => error_call_back(&t, "expected floating-point value"),
                    ValType::Int => error_call_back(&t, "expected integer value"),
                    ValType::Bool => {}
                }
                param.add_bool(true);
            } else if t.token[0] == b'f' && t.token == b"false" {
                match val_type {
                    ValType::Unknown => val_type = ValType::Bool,
                    ValType::String => error_call_back(&t, "expected string value"),
                    ValType::Float => error_call_back(&t, "expected floating-point value"),
                    ValType::Int => error_call_back(&t, "expected integer value"),
                    ValType::Bool => {}
                }
                param.add_bool(false);
            } else {
                match val_type {
                    ValType::Unknown => val_type = ValType::Float,
                    ValType::String => error_call_back(&t, "expected string value"),
                    ValType::Float => {}
                    ValType::Int => {}
                    ValType::Bool => error_call_back(&t, "expected Boolean value"),
                }

                if val_type == ValType::Int {
                    param.add_int(
                        std::str::from_utf8(t.token)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    );
                } else {
                    param.add_float(
                        std::str::from_utf8(t.token)
                            .unwrap()
                            .parse::<Float>()
                            .unwrap(),
                    );
                }
            }
        };

        if val.token == &[b'['] {
            loop {
                let val = next_token(file_stack, unget_token, TOKEN_REQUIRED).unwrap();
                if val.token == &[b']'] {
                    break;
                }
                add_val(val);
            }
        } else {
            add_val(val);
        }

        param.loc = t.loc;
        parameter_vector.push(param);
    }
}

fn basic_param_list_entrypoint<'a>(
    file_stack: &mut Vec<Tokenizer<'a>>,
    unget_token: &mut Option<Token<'a>>,
) -> (String, ParsedParameterVector) {
    let t = next_token(file_stack, unget_token, TOKEN_REQUIRED).unwrap();
    let dequoted = dequote_string(&t);
    let parameter_vector = parse_parameters(file_stack, unget_token);
    (dequoted, parameter_vector)
}

fn next_token<'a>(
    file_stack: &mut Vec<Tokenizer<'a>>,
    unget_token: &mut Option<Token<'a>>,
    flags: i32,
) -> Option<Token<'a>> {
    if unget_token.is_some() {
        return replace(unget_token, None);
    }
    if file_stack.is_empty() {
        if flags & TOKEN_REQUIRED != 0 {
            error_exit(None, &"premature end of file".to_string());
        }
        return None;
    }
    let tok: Option<Token> = file_stack.last_mut().unwrap().next();
    match tok {
        None => {
            println!(
                "Finished parsing {}",
                file_stack.last().unwrap().loc.filename
            );
            file_stack.pop();
            return next_token(file_stack, unget_token, flags);
        }
        // Some(t) if t.token.starts_with("#") => next_token(file_stack, flags),
        _ => tok,
    }
}

fn parse(target: &mut dyn ParserTarget, t: Tokenizer) {
    let mut file_stack: Vec<Tokenizer> = Vec::new();
    file_stack.push(t);

    let mut unget_token = None;

    loop {
        let tok = next_token(&mut file_stack, &mut unget_token, TOKEN_OPTIONAL);
        if tok.is_none() {
            break;
        }
        let tok = tok.unwrap();
        let loc = tok.loc;
        let token = std::str::from_utf8(tok.token).unwrap();
        // println!("Token: {}", token);

        match token {
            "AttributeBegin" => {
                // target.attribute_begin(loc);
            }
            "AttributeEnd" => {
                // target.attribute_end(loc);
            }
            "Attribute" => {
                // basic_param_list_entrypoint(target, ParserTarget::attribute, loc);
            }
            "ActiveTransform" => {}
            "AreaLightSource" => {}
            "Accelerator" => {}
            "Camera" => {
                let (name, params) = basic_param_list_entrypoint(&mut file_stack, &mut unget_token);
                target.camera(&name, params, loc);
            }
            "Film" => {
                let (name, params) = basic_param_list_entrypoint(&mut file_stack, &mut unget_token);
                target.film(&name, params, loc);
            }
            "Integrator" => {
                let (name, params) = basic_param_list_entrypoint(&mut file_stack, &mut unget_token);
                target.integrator(&name, params, loc);
            }
            "LookAt" => {
                let mut v = [0 as Float; 9];
                for i in 0..9 {
                    v[i] = std::str::from_utf8(
                        next_token(&mut file_stack, &mut unget_token, TOKEN_REQUIRED)
                            .unwrap()
                            .token,
                    )
                    .unwrap()
                    .parse::<Float>()
                    .unwrap();
                }
                target.look_at(v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], loc);
            }
            "PixelFilter" => {
                let (name, params) = basic_param_list_entrypoint(&mut file_stack, &mut unget_token);
                target.pixel_filter(&name, params, loc);
            }
            "Sampler" => {
                let (name, params) = basic_param_list_entrypoint(&mut file_stack, &mut unget_token);
                target.sampler(&name, params, loc);
            }
            _ => {}
        }
    }
}

pub fn parse_files(
    target: &mut dyn ParserTarget,
    filenames: Vec<String>,
) -> Result<(), &'static str> {
    let tok_error = |msg: &'static str, loc: &FileLoc| error_exit(Some(loc), &format!("{}", msg));
    if filenames.is_empty() {
        // TODO: stdin file description
        return Err("No file description given.");
    } else {
        for f in &filenames {
            let contents = read_file_contents(f);
            let t = Tokenizer::new(f, &contents, tok_error);
            parse(target, t);
        }
    }

    println!("{:?}", INTERNED_STRINGS);

    Ok(())
}

// pub struct Tokenizer {
//     contents: String,
//     // end: &char,
//     // s_escaped
// }

struct Tokenizer<'a> {
    contents: &'a Vec<u8>,
    // pos: Peekable<CharIndices>,
    pos: usize,
    end: usize,
    loc: FileLoc,
    error_call_back: ErrorCallBack,
}

type ErrorCallBack = fn(&'static str, &FileLoc);

impl<'a> Tokenizer<'a> {
    pub fn new(filename: &String, contents: &'a Vec<u8>, error_call_back: ErrorCallBack) -> Self {
        Self {
            contents: contents,
            loc: FileLoc::new(String::from(filename)),
            error_call_back,
            // pos: contents.char_indices().peekable(),
            pos: 0,
            end: contents.len(),
        }
    }

    // pub fn new(str: String, filename: &String, error_call_back: ErrorCallBack) -> Self {
    //     Self {
    //         loc: FileLoc::new(String::from(filename)),
    //         contents: str,
    //         error_call_back,
    //         // pos: str.chars(),
    //     }
    // }

    pub fn next(&mut self) -> Option<Token<'a>> {
        loop {
            let token_start = self.pos;
            let start_loc = self.loc.clone();

            let ch = self.get_char();
            if ch.is_none() {
                return None;
            }
            match ch.unwrap() {
                ' ' | '\n' | '\t' | '\r' => {}
                '"' => {
                    let mut have_escaped = false;
                    loop {
                        let mut ch = self.get_char();
                        match ch {
                            Some('"') => break,
                            None => {
                                (self.error_call_back)("premature EOF", &start_loc);
                                return None;
                            }
                            Some('\n') => {
                                (self.error_call_back)("unterminated string", &start_loc);
                                return None;
                            }
                            Some('\\') => {
                                have_escaped = true;
                                ch = self.get_char();
                                if ch.is_none() {
                                    (self.error_call_back)("premature EOF", &start_loc);
                                    return None;
                                }
                            }
                            _ => {}
                        }
                    }
                    if !have_escaped {
                        return Some(Token {
                            loc: start_loc,
                            token: &self.contents[token_start..self.pos],
                        });
                    } else {
                        todo!()
                    }
                }
                '[' | ']' => {
                    return Some(Token {
                        loc: start_loc,
                        token: &self.contents[token_start..token_start + 1],
                    })
                }
                '#' => loop {
                    let ch = self.get_char();
                    match ch {
                        None => break,
                        Some('\n' | '\r') => {
                            self.unget_char();
                            break;
                        }
                        _ => {}
                    }
                },
                _ => {
                    loop {
                        let ch = self.get_char();
                        match ch {
                            None => break,
                            Some(' ' | '\n' | '\t' | '\r' | '"' | '[' | ']') => {
                                self.unget_char();
                                break;
                            }
                            _ => {}
                        }
                    }
                    return Some(Token {
                        loc: start_loc,
                        token: &self.contents[token_start..self.pos],
                    });
                }
            }
        }
    }

    #[inline]
    fn get_char(&mut self) -> Option<char> {
        // let next = self.pos.next();
        if self.pos == self.end {
            return None;
        }
        let ch = self.contents[self.pos] as char;
        self.pos += 1;
        if ch == '\n' {
            self.loc.line += 1;
            self.loc.column = 0;
        } else {
            self.loc.column += 1;
        }
        Some(ch as char)
    }

    #[inline]
    fn unget_char(&mut self) {
        self.pos -= 1;
        if self.contents[self.pos] == b'\n' {
            self.loc.line -= 1;
        }
    }
}
