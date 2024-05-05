use std::{
    cell::RefCell,
    fs::File,
    iter::Peekable,
    mem::replace,
    ptr::null,
    str::{CharIndices, Chars},
};

use dashmap::mapref::one::Ref;

use crate::{
    paramdict::ParsedParameterVector,
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
    fn attribute(&mut self, target: &String, params: ParsedParameterVector, loc: FileLoc);
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

// impl<'a> Token<'a> {
//     pub fn new(token:&str, loc:FileLoc)
// }

const TOKEN_OPTIONAL: i32 = 0;
const TOKEN_REQUIRED: i32 = 1;

fn parse(target: &mut dyn ParserTarget, t: Tokenizer) {
    let mut file_stack: Vec<Tokenizer> = Vec::new();
    file_stack.push(t);

    let parse_error = |msg: &'static str, loc: &FileLoc| error_exit(Some(loc), &format!("{}", msg));

    // let mut unget_token = Some(Token::default());

    fn next_token<'a>(file_stack: &mut Vec<Tokenizer<'a>>, flags: i32) -> Option<Token<'a>> {
        // if unget_token.is_some() {
        //     return replace(&mut unget_token, None);
        // }
        if file_stack.is_empty() {
            if flags & TOKEN_REQUIRED != 0 {
                error_exit(None, &"premature end of file".to_string());
            }
            return None;
        }
        let tok = file_stack.last_mut().unwrap().next();
        match tok {
            None => {
                println!(
                    "Finished parsing {}",
                    file_stack.last().unwrap().loc.filename
                );
                file_stack.pop();
                return next_token(file_stack, flags);
            }
            // Some(t) if t.token.starts_with("#") => next_token(file_stack, flags),
            _ => tok,
        }
    }

    loop {
        let tok = next_token(&mut file_stack, TOKEN_OPTIONAL);
        if tok.is_none() {
            break;
        }
        let token = std::str::from_utf8(tok.unwrap().token).unwrap();
        // println!("Token: {}", token);
        match token {
            "AttributeBegin" => {
                println!("YYY!")
            }
            _ => {}
        }
    }
}

pub fn parse_files(target: &mut dyn ParserTarget, filenames: Vec<String>) -> Result<(), &str> {
    let tok_error = |msg: &'static str, loc: &FileLoc| error_exit(Some(loc), &format!("{}", msg));
    if filenames.is_empty() {
        // TODO: stdin file description
        return Err("No file description given.");
    } else {
        for f in &filenames {
            let contents = read_file_contents(f);
            let t = Tokenizer::new(f, contents.as_bytes(), tok_error);
            parse(target, t);
        }
    }
    Ok(())
}

// pub struct Tokenizer {
//     contents: String,
//     // end: &char,
//     // s_escaped
// }

struct Tokenizer<'a> {
    contents: &'a [u8],
    // pos: Peekable<CharIndices<'a>>,
    pos: usize,
    end: usize,
    loc: FileLoc,
    error_call_back: ErrorCallBack,
}

type ErrorCallBack = fn(&'static str, &FileLoc);

impl<'a> Tokenizer<'a> {
    pub fn new(filename: &String, contents: &'a [u8], error_call_back: ErrorCallBack) -> Self {
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
