use std::{collections::HashMap};

mod tokens;
mod regex;
mod automata;
mod lexica_analysis;

use regex::{all, from_set, from_word, from_char, concat, union, repeat};
use tokens::{USE_DEFAULT, USE_IGNORE_THIS, USE_GET_LEXEME};
use automata::TokensDFA;
use lexica_analysis::SyntaxAnalysis;

fn main() {
    let letter = from_set(b'a'..b'z');
    let digit = from_set(b'0'..b'9');
    let digits = concat(vec![&digit, &repeat(&digit)]);

    let tokens_regexs = HashMap::from(
        [   
            ("ID".to_string()           , concat( vec![&letter, &repeat( &union(vec![&letter, &digit])) ])),
            ("CONST_INT".to_string()    , digits.clone()),
            ("CONST_FLOAT".to_string()  , concat( vec![&digits, &from_char(b'.'), &digits]) ),
            ("CONST_CHAR".to_string()   , concat( vec![&from_char(b'\''), &all() , &from_char(b'\'')])),
            ("FUNCTION".to_string()     , from_word("function")),
            ("WS".to_string()           , union( vec![&from_char(b'\n'), &from_char(b'\t'), &from_char(b' '), &from_char(b'\0')]))
        ]
    );
    let tokens_attrs = HashMap::from(
        [
            ("ID".to_string()           , USE_GET_LEXEME),
            ("CONST_INT".to_string()    , USE_GET_LEXEME),
            ("CONST_FLOAT".to_string()  , USE_GET_LEXEME),
            ("CONST_CHAR".to_string()   , USE_GET_LEXEME),
            ("FUNCTION".to_string()     , USE_DEFAULT),
            ("WS".to_string()           , USE_IGNORE_THIS)
        ]
    );

    let DFA = TokensDFA::new(tokens_regexs);
    
}
