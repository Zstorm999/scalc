use regex::Regex;
use std::collections::VecDeque;

pub struct Parser<A> {
    elements: Vec<ParserElement<A>>,
}

struct ParserElement<A> {
    reg: Regex,
    on_found: Box<dyn Fn(&str) -> A>,
}

impl<A> Parser<A> {
    pub fn new() -> Parser<A> {
        Parser {
            elements: Vec::new(),
        }
    }

    pub fn push(mut self, reg: &str, on_found: impl Fn(&str) -> A + 'static) -> Parser<A> {
        let reg = String::from("^") + reg + "$";
        let element = ParserElement {
            reg: Regex::new(&reg).unwrap(),
            on_found: Box::new(on_found),
        };

        self.elements.push(element);
        self
    }

    pub fn parse<'a>(&'a self, content: &'a str) -> ParserIter<'a, A> {
        ParserIter {
            parser: self,
            content,
            pos: 0,
        }
    }
}

pub struct ParserIter<'a, A> {
    parser: &'a Parser<A>,
    content: &'a str,
    pos: usize,
}

impl<A> Iterator for ParserIter<'_, A> {
    type Item = Result<A, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.content.len() == self.pos {
            return None; // end of the string, iterator is complete
        }

        for i in ((self.pos + 1)..=(self.content.len())).rev() {
            if let Some(val) = (&self.parser.elements).into_iter().fold(None, |acc, elt| {
                if !acc.is_none() {
                    return acc;
                } else {
                    if elt.reg.is_match(&self.content[self.pos..i]) {
                        return Some((elt.on_found)(&self.content[self.pos..i]));
                    }
                    return None;
                }
            }) {
                self.pos = i;
                return Some(Ok(val));
            }
        }

        Some(Err(()))
    }
}
