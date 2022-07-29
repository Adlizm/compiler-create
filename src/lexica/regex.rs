use std::ops::{Add, BitOr, RangeInclusive};

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
        if let Regex::Union(vec) = self {
            let mut new_vec = vec.to_vec();
            new_vec.push(other.clone());
            return Regex::Union(new_vec);
        }
        Regex::Union(vec![self.clone(), other.clone()])
    }
}

impl BitOr for Regex {
    type Output = Regex;

    fn bitor(self, other: Self) -> Regex {
        if let Regex::Concat(vec) = self {
            let mut new_vec = vec.to_vec();
            new_vec.push(other.clone());
            return Regex::Concat(new_vec);
        }
        Regex::Concat(vec![self.clone(), other.clone()])
    }
}

impl Regex {
    pub fn new<A>(args: A) -> Regex
        where A: IntoRegex {
        args.into()
    }

    pub fn from_set(set: RangeInclusive<u8>) -> Regex {
        let mut bytes = Vec::new();
            for byte in set {
                bytes.push(Regex::Char(byte));
            }
        Regex::Union(bytes)
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
        Regex::from_set(0..=255)
    }
    
    pub fn repeat(re: Regex) -> Regex {
        Regex::Repeat(Box::new(re.clone()))
    }
}
pub trait IntoRegex{
    fn into(self) -> Regex;
}

impl IntoRegex for u8 {
    fn into(self) -> Regex {
        Regex::from_byte(self)
    }
}
impl IntoRegex for &str {
    fn into(self) -> Regex {
        Regex::from_word(self)
    }
}
impl IntoRegex for RangeInclusive<u8> {
    fn into(self) -> Regex {
        Regex::from_set(self)
    }
}
impl IntoRegex for Vec<u8> {
    fn into(self) -> Regex {
        if self.len() == 0 {
            return Regex::Empty;
        }
        let mut bytes = Vec::new();
        for byte in self {
            bytes.push(Regex::Char(byte));
        }
        Regex::Union(bytes)
    }
}