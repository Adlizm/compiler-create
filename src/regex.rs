use std::ops::Range;

#[derive (Debug)]
pub enum Regex {
    Concat(Vec<Regex>),
    Union(Vec<Regex>),
    Repeat(Box<Regex>),
    Word(Vec<u8>),
    Char(u8),
    Empty
}
impl Clone for Regex {
    fn clone(&self) -> Self {
        match self {
            Regex::Concat(regexs) => {
                let mut vec = Vec::new();
                for re in regexs {
                    vec.push(re.clone());
                }
                Regex::Concat(vec)
            },
            Regex::Union(regexs) => {
                let mut vec = Vec::new();
                for re in regexs {
                    vec.push(re.clone());
                }
                Regex::Union(vec)
            },
            Regex::Repeat(re) => {
                Regex::Repeat(re.clone())
            },
            Regex::Word(s) => {
                Regex::Word(s.clone())
            },
            Regex::Char(c) => Regex::Char(*c),
            Regex::Empty => Regex::Empty,
        }
    }
}

pub fn empty() -> Regex {
    Regex::Empty
}

pub fn from_set(set: Range<u8>) -> Regex {
    if set.is_empty() {
        return Regex::Empty;
    }
    let mut vec_char = Vec::new();
    for char in set {
        vec_char.push(Regex::Char(char));
    }
    Regex::Union(vec_char)
}

pub fn all() -> Regex {
    let mut vec_char: Vec<Regex> = Vec::new();
    for char in 0..=255 {
        vec_char.push(Regex::Char(char));
    }
    Regex::Union(vec_char)
}

pub fn from_char(char: u8) -> Regex {
    Regex::Char(char)
}

pub fn from_word(word: &str) -> Regex { 
    let mut vec_char: Vec<u8> = word.as_bytes().to_vec();
    if vec_char.is_empty() {
        return Regex::Empty;
    }
    Regex::Word(vec_char)
}

pub fn concat(regexs: Vec<&Regex>) -> Regex {
    if regexs.is_empty() {
        return Regex::Empty;
    }
    let mut rc = Vec::new();
    for re in regexs {
        rc.push(re.clone());
    }
    Regex::Concat(rc)
}

pub fn union(regexs: Vec<&Regex>) -> Regex {
    if regexs.is_empty() {
        return Regex::Empty;
    }
    let mut rc = Vec::new();
    for re in regexs {
        rc.push(re.clone());
    }
    Regex::Union(rc)
}

pub fn repeat(re: &Regex) -> Regex {
    Regex::Repeat(Box::new(re.clone()))
}