use compiler_create::lexica:: {
    regex::Regex,
    tokens::{DEFAULT, IGNORE_THIS, GET_LEXEME, INIT_INLINE_COMMENT, INIT_BLOCK_COMMENT, END_BLOCK_COMMENT},
    analysis::LexicalAnalysis
};

use compiler_create::syntax::{
    analysis::SyntaxAnalysis
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Vars { 
    //My terminals
    If, Else, While, Let, Set, Id,
    ConstInt, ConstFloat, ConstChar,
    Add, Sub, Mul, Div,
    LessThan, GreatThan, LessEquals, GreatEquals, Equals, NotEquals,
    RightBrace, LeftBrace, RightParentheses, LeftParentheses,
    Comma, Semicolon,
    Ws, LineComment, InitComment, EndComment,
 
    //My no terminals 
    Init, VarsDeclarate, SeqCommands, Command, Block,
    Selection, Selection_, Repatation, Assignment, Condition,
    Expression, Expression_, Term, Term_, Factor
}

fn main() {
    let letter = Regex::new(b'a'..=b'z') + Regex::new(b'A'..=b'Z');
    let digit  = Regex::new(b'0'..=b'9');
    let digits = Regex::new(b'0'..=b'9') | Regex::repeat(Regex::new(b'0'..=b'9'));

    let tokens_regexs = vec![  
        (Vars::If   , DEFAULT, Regex::new("if")),
        (Vars::Else , DEFAULT, Regex::new("else")),
        (Vars::While, DEFAULT, Regex::new("while")),
        (Vars::Let  , DEFAULT, Regex::new("let")),
        (Vars::Set  , DEFAULT, Regex::new("=")),

        (Vars::Id, GET_LEXEME, letter.clone()|Regex::repeat(letter.clone() + digit.clone())),

        (Vars::ConstInt    , GET_LEXEME  , digits.clone()),
        (Vars::ConstFloat  , GET_LEXEME  , digits.clone()|Regex::new(b'.')|digits.clone()),
        (Vars::ConstChar   , GET_LEXEME  , Regex::new(b'\'')|Regex::any()|Regex::new(b'\'')),
        
        (Vars::Add, DEFAULT, Regex::new(b'+')),
        (Vars::Sub, DEFAULT, Regex::new(b'-')),
        (Vars::Mul, DEFAULT, Regex::new(b'*')),
        (Vars::Div, DEFAULT, Regex::new(b'/')),

        (Vars::LessThan     , DEFAULT, Regex::new(">")),
        (Vars::GreatThan    , DEFAULT, Regex::new("<")),
        (Vars::LessEquals   , DEFAULT, Regex::new(">=")),
        (Vars::GreatEquals  , DEFAULT, Regex::new("<=")),
        (Vars::Equals       , DEFAULT, Regex::new("==")),
        (Vars::NotEquals    , DEFAULT, Regex::new("!=")),

        (Vars::RightBrace      , DEFAULT, Regex::new(b'}')),
        (Vars::LeftBrace       , DEFAULT, Regex::new(b'{')),
        (Vars::RightParentheses, DEFAULT, Regex::new(b')')),
        (Vars::LeftParentheses , DEFAULT, Regex::new(b'(')),

        (Vars::Comma        , DEFAULT, Regex::new(b',')),
        (Vars::Semicolon    , DEFAULT, Regex::new(b';')),
        
        (Vars::Ws          , IGNORE_THIS         , Regex::new(vec![b'\r', b'\n', b'\t', b' ',  b'\0'])),
        (Vars::LineComment , INIT_INLINE_COMMENT , Regex::new("//")),
        (Vars::InitComment , INIT_BLOCK_COMMENT  , Regex::new("/*")),
        (Vars::EndComment  , END_BLOCK_COMMENT   , Regex::new("*/")),
    ];

    let rules = vec![
        (Vars::Init          , vec![Vars::Id, Vars::LeftParentheses, Vars::RightParentheses, Vars::Block]),
        (Vars::Block         , vec![Vars::LeftBrace, Vars::VarsDeclarate, Vars::SeqCommands, Vars::RightBrace]),
        (Vars::Block         , vec![Vars::Command]),
        
        (Vars::VarsDeclarate , vec![]),
        (Vars::VarsDeclarate , vec![Vars::Let, Vars::Id, Vars::Set, Vars::Expression, Vars::Semicolon, Vars::VarsDeclarate]),
        (Vars::SeqCommands   , vec![]),
        (Vars::SeqCommands   , vec![Vars::Command, Vars::SeqCommands]),
        
        (Vars::Command       , vec![Vars::Selection]),
        (Vars::Command       , vec![Vars::Repatation]),
        (Vars::Command       , vec![Vars::Assignment]),
        
        (Vars::Selection     , vec![Vars::If, Vars::Condition, Vars::Block, Vars::Selection_]),
        (Vars::Selection_    , vec![]),
        (Vars::Selection_    , vec![Vars::Else, Vars::Block]),
        (Vars::Repatation    , vec![Vars::While, Vars::Condition, Vars::Block]),
        (Vars::Assignment    , vec![Vars::Id, Vars::Set, Vars::Expression, Vars::Semicolon]),
        
        (Vars::Expression    , vec![Vars::Term, Vars::Expression_]),
        (Vars::Expression_   , vec![Vars::Add, Vars::Term, Vars::Expression_]),
        (Vars::Expression_   , vec![Vars::Sub, Vars::Term, Vars::Expression_]),
        (Vars::Expression_   , vec![]),
        
        (Vars::Term          , vec![Vars::Factor, Vars::Term_]),
        (Vars::Term_         , vec![Vars::Mul, Vars::Factor, Vars::Term_]),
        (Vars::Term_         , vec![Vars::Div, Vars::Factor, Vars::Term_]),
        (Vars::Term_         , vec![]),

        (Vars::Factor        , vec![Vars::Id]),
        (Vars::Factor        , vec![Vars::ConstInt]),
        (Vars::Factor        , vec![Vars::ConstFloat]),
        (Vars::Factor        , vec![Vars::ConstChar]),
        (Vars::Factor        , vec![Vars::Sub, Vars::Factor]),
        (Vars::Factor        , vec![Vars::LeftParentheses, Vars::Expression, Vars::RightParentheses]),

        (Vars::Condition     , vec![Vars::Expression, Vars::Equals     , Vars::Expression]),
        (Vars::Condition     , vec![Vars::Expression, Vars::NotEquals  , Vars::Expression]),
        (Vars::Condition     , vec![Vars::Expression, Vars::LessEquals , Vars::Expression]),
        (Vars::Condition     , vec![Vars::Expression, Vars::LessThan   , Vars::Expression]),
        (Vars::Condition     , vec![Vars::Expression, Vars::GreatEquals, Vars::Expression]),
        (Vars::Condition     , vec![Vars::Expression, Vars::GreatThan  , Vars::Expression]),
    ];

    let la = LexicalAnalysis::new(tokens_regexs, "teste.txt");
    let sa = SyntaxAnalysis::new(rules);
}
