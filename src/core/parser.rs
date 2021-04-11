use super::primitives::*;
use super::lexer::{Lexer};

use std::collections::HashMap;

macro_rules! primitive {
    ($token:expr) => { $token.ok_or_else(|| ParserError(line!()))? };
}

#[derive(Debug)]
pub struct ParserError(pub u32);


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    allow_streams: bool,
    buf1: Option<Primitives>,
    buf2: Option<Primitives>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, allow_streams: bool) -> Self {
        let mut p = Parser {
            lexer: lexer,
            allow_streams: allow_streams,
            buf1: None,
            buf2: None,
        };
        p.refill();
        p
    }

    pub fn lexer(&self) -> &'a Lexer {
        &self.lexer
    }

    pub fn buf1(&self) -> Option<Primitives> {
        self.buf1.clone()
    }

    pub fn buf2(&self) -> Option<Primitives> {
        self.buf2.clone()
    }

    fn refill(&mut self) {
        self.buf1 = self.lexer.get_obj().ok();
        self.buf2 = self.lexer.get_obj().ok();
    }

    fn shift(&mut self) -> Option<Primitives> {
        let gone = self.buf1.take();
        if self.buf2 == Primitives::cmd("ID") {
            self.buf1 = self.buf2.take();
            self.buf2 = None;
        } else {
            self.buf1 = self.buf2.take();
            self.buf2 = self.lexer.get_obj().ok();
        }
        gone
    }

    /*
    fn trye_shift() {
    }
    */

    pub fn get_obj(&mut self) -> Result<Primitives, ParserError> {
        let buf1 = primitive!(self.shift());

        if let Primitives::Cmd(ref cmd) = buf1 {
            /*
            if cmd == b"BI" { // inline image
                returns self.make_inline_image();
            }*/
            if cmd == b"[" { // array
                let mut array = Vec::new();
                while self.buf1 != Primitives::cmd("]") && self.buf1 != Primitives::EOF {
                    array.push(self.get_obj()?);
                }
                if self.buf1 == Primitives::EOF {
                    eprintln!("End of file inside array");
                    return Ok(Primitives::Array(array));
                }
                self.shift();
                return Ok(Primitives::Array(array));
            } else if cmd == b"<<" {
                let mut dict = HashMap::<Name, Primitives>::new();

                let mut i = 0;
                while self.buf1 != Primitives::cmd(">>") && self.buf1 != Primitives::EOF {
                    if primitive!(self.buf1.as_ref()).is_name() {
                        if let Primitives::Name(name) = primitive!(self.buf1.take()) {
                            self.shift();
                            if self.buf1 == Primitives::EOF {
                                break;
                            }
                            dict.insert(Name(name.0), self.get_obj()?);
                        }
                    } else {
                        eprintln!("Malformed dictionary: key must be a name object");
                        self.shift();
                        continue;
                    }
                }

                if self.buf1 == Primitives::EOF {
                    eprintln!("End of file inside dictionary");
                    return Ok(Primitives::Dict(Dict::new(dict)));
                }

                // Stream objects are not allowed inside content streams or object streams.
                if self.buf2.as_ref().unwrap().is_cmd("stream") {
                    if self.allow_streams {
                        return self.make_stream(dict)
                    } else {
                        return Ok(Primitives::Dict(Dict::new(dict)));
                    }
                }
                self.shift();
                return Ok(Primitives::Dict(Dict::new(dict)));
            } else {
                return Ok(buf1);
            }
        }

        if let Some(num1) = buf1.get_integer() {
            if primitive!(self.buf1.as_ref()).is_integer() && primitive!(self.buf2.as_ref()).is_cmd("R") {

                if let Primitives::Int(num2) = primitive!(self.buf1.take()) {
                    self.shift();
                    self.shift();
                    return Ok(Primitives::Ref(Ref::new(num1 as i32, num2 as i32)));
                }
            }
            return Ok(Primitives::Int(num1));
        }

        if buf1.is_string() {
            // if (cipher_transform) {
            //     // cipherTransform.decrypt_string(buf1)
            // }
            return Ok(buf1);
        }

        // simple object
        Ok(buf1)
    }

    fn make_stream(&self, dict: HashMap<Name, Primitives>) -> Result<Primitives, ParserError> {
        Err(ParserError(line!()))
    }

}


