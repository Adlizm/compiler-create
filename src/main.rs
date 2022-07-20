mod tokens;
mod regex;
mod automata;
mod lexical_analysis;

use regex::{Regex, any, repeat, union};
use tokens::{DEFAULT, IGNORE_THIS, GET_LEXEME, INIT_INLINE_COMMENT, INIT_BLOCK_COMMENT, END_BLOCK_COMMENT};
use automata::{TokensDFA, TokensNFA};
use lexical_analysis::LexicalAnalysis;

fn main() {
    let letter = Regex::new(b'a'..=b'z');
    let digit = Regex::new(b'0'..=b'9');
    let digits = digit.clone()|repeat(digit.clone());

    let tokens_regexs = vec!
        [   
            ("FUNCTION".to_string()     , DEFAULT     , Regex::new("function")),
            ("ID".to_string()           , GET_LEXEME  , letter.clone()|repeat(letter.clone() + digit.clone())),
            ("CONST_INT".to_string()    , GET_LEXEME  , digits.clone()),
            ("CONST_FLOAT".to_string()  , GET_LEXEME  , digits.clone()|Regex::new(b'.')|digits.clone()),
            ("CONST_CHAR".to_string()   , GET_LEXEME  , Regex::new(b'\'')|any()|Regex::new(b'\'')),
            ("WS".to_string()           , IGNORE_THIS , Regex::new(vec![b'\r', b'\n', b'\t', b' ',  b'\0'])),
            ("LINE_COMMENT".to_string() , INIT_INLINE_COMMENT , Regex::new("//")),
            ("INIT_COMMENT".to_string() , INIT_BLOCK_COMMENT  , Regex::new("/*")),
            ("END_COMMENT".to_string()  , END_BLOCK_COMMENT   , Regex::new("*/")),
        ];
    
    let sa = LexicalAnalysis::new(tokens_regexs, "teste.txt");
    for token in sa {
        println!("{:?}", token);
    }

}
