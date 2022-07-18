use std::ops::Range;

#[derive (Debug)]
pub enum Regex {
    Concat(Vec<Regex>),
    Union(Vec<Regex>),
    Repeat(Box<Regex>),
    Terminal(u8)
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
            Regex::Terminal(s) => {
                Regex::Terminal(s.clone())
            },
        }
    }
}


pub fn from_set(set: Range<u8>) -> Regex {
    let mut vec_char: Vec<Regex> = Vec::new();
    for char in set {
        vec_char.push(Regex::Terminal(char));
    }
    Regex::Union(vec_char)
}

pub fn all() -> Regex {
    let mut vec_char: Vec<Regex> = Vec::new();
    for char in 0..=255 {
        vec_char.push(Regex::Terminal(char));
    }
    Regex::Union(vec_char)
}

pub fn from_char(char: u8) -> Regex {
    Regex::Terminal(char)
}

pub fn from_word(word: &str) -> Regex { 
    let mut vec_char: Vec<Regex> = Vec::new();
    for char in word.as_bytes() {
        vec_char.push(Regex::Terminal(*char));
    }
    Regex::Concat(vec_char)
}

pub fn concat(regexs: Vec<&Regex>) -> Regex {
    let mut rc = Vec::new();
    for re in regexs {
        rc.push(re.clone());
    }
    Regex::Concat(rc)
}

pub fn union(regexs: Vec<&Regex>) -> Regex {
    let mut rc = Vec::new();
    for re in regexs {
        rc.push(re.clone());
    }
    Regex::Union(rc)
}

pub fn repeat(re: &Regex) -> Regex {
    Regex::Repeat(Box::new(re.clone()))
}