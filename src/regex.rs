use std::{ops::{Add, BitOr, RangeInclusive}, vec};

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

impl Add for Regex {
    type Output = Regex;

    fn add(self, other: Self) -> Regex {
        match self {
            Regex::Union(mut vec) => { 
                vec.push(other);
                Regex::Union(vec)
            },
            _ => Regex::Union(vec![self, other])
        }
    }
}

impl BitOr for Regex {
    type Output = Regex;

    fn bitor(self, other: Self) -> Regex {
        match self {
            Regex::Concat(mut vec) => { 
                vec.push(other);
                Regex::Concat(vec)
            },
            _ => Regex::Concat(vec![self, other])
        }
    }
}

impl Regex {
    pub fn new<A>(args: A) -> Regex
        where A: IntoRegex {
        args.into()
    }
}
pub trait IntoRegex{
    fn into(self) -> Regex;
}

impl IntoRegex for u8 {
    fn into(self) -> Regex {
        from_byte(self)
    }
}
impl IntoRegex for &str {
    fn into(self) -> Regex {
        from_word(self)
    }
}
impl IntoRegex for RangeInclusive<u8> {
    fn into(self) -> Regex {
        from_set(self)
    }
}
impl IntoRegex for Vec<u8> {
    fn into(self) -> Regex {
        let mut symbols = Vec::new();
        for byte in self {
            symbols.push(Regex::Char(byte));
        }
        if symbols.len() == 0 {
            return Regex::Empty;
        }
        Regex::Union(symbols)
    }
}


pub fn from_set(set: RangeInclusive<u8>) -> Regex {
    if set.is_empty() {
        return Regex::Empty;
    }
    let mut vec_char = Vec::new();
    for char in set {
        vec_char.push(Regex::Char(char));
    }
    Regex::Union(vec_char)
}
pub fn from_word(word: &str) -> Regex { 
    let vec_char: Vec<u8> = word.as_bytes().to_vec();
    if vec_char.is_empty() {
        return Regex::Empty;
    }
    Regex::Word(vec_char)
}
pub fn from_byte(char: u8) -> Regex {
    Regex::Char(char)
}

pub fn any() -> Regex {
    from_set(0..=255)
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
pub fn union(regexs: Vec<Regex>) -> Regex {
    if regexs.is_empty() {
        return Regex::Empty;
    }
    let mut rc = Vec::new();
    for re in regexs {
        rc.push(re.clone());
    }
    Regex::Union(rc)
}
pub fn repeat(re: Regex) -> Regex {
    Regex::Repeat(Box::new(re.clone()))
}