//use std::result;

use crate::program::Program;
use crate::utils::{GlitchType, List::*};
use std::borrow::Borrow;


trait Parser<I> {
    fn parse(&self, input: GlitchType<I>) -> Result<GlitchType<I>, String>;
}

struct ParserReal<T> {
    parse: Box<fn(GlitchType<T>) -> Result<GlitchType<T>, String>>
}

impl<T> Parser<T> for ParserReal<T> {
    fn parse(&self, input: GlitchType<T>) -> Result<GlitchType<T>, String> {

        return (self.parse.borrow())(input)
    }
}

struct ParseAnyChar {}

impl Parser<char> for ParseAnyChar {
    fn parse(&self, input: GlitchType<char>) -> Result<GlitchType<char>, String> {
        return match input.right {
            None => Err("shitty input. no string".to_string()),
            Some(xs) => {
                match xs {
                    Emp => Err("shitty input. string with length zero".to_string()),
                    NEmp(x, xs) => Ok(GlitchType { left: x, right: Some(*xs) }),
                }
            }
        };
    }
}

struct ParseExactChar {
    pub c: char
}

impl Parser<char> for ParseExactChar {
    fn parse(&self, input: GlitchType<char>) -> Result<GlitchType<char>, String> {
        return match input.right {
            None => Err("shitty input. no string".to_string()),
            Some(xs) => {
                match xs {
                    Emp => Err("shitty input. string with length zero".to_string()),
                    NEmp(x, xs) => {
                        if x == self.c {
                            Ok(GlitchType { left: x, right: Some(*xs) })
                        } else {
                            Err("shit char".to_string())
                        }
                    }
                }
            }
        };
    }
}

fn parse_alter<T: 'static>(left: Box<ParserReal<T>>, right: Box<ParserReal<T>>) -> Box<ParserReal<T>> {
    return Box::new(ParserReal {
        parse: Box::new(|input| {
            return match left.parse(input) {
                Ok(x) => Ok(x),
                Err(err) => right.parse(input),
            };
        })
    });
}
