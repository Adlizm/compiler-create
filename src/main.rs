use compiler_create::lexica::{
    regex::Regex,
    tokens::TokenUses
};

use compiler_create::rule;
use compiler_create::syntax::utils::IsTerminal;



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
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
impl IsTerminal for Vars {
    fn is_terminal(self) -> bool {
        match self {
           x if x >= Vars::If && x <= Vars::EndComment => true,
            _ => false
        }
    }
}

fn main() {
    let letter = Regex::new(b'a'..=b'z') + Regex::new(b'A'..=b'Z');
    let digit  = Regex::new(b'0'..=b'9');
    let digits = Regex::new(b'0'..=b'9') | Regex::repeat(Regex::new(b'0'..=b'9'));

    let tokens_regexs = vec![  
        (Vars::If   , TokenUses::Default, Regex::new("if")),
        (Vars::Else , TokenUses::Default, Regex::new("else")),
        (Vars::While, TokenUses::Default, Regex::new("while")),
        (Vars::Let  , TokenUses::Default, Regex::new("let")),
        (Vars::Set  , TokenUses::Default, Regex::new("=")),

        (Vars::Id, TokenUses::GetLexeme, letter.clone()|Regex::repeat(letter.clone() + digit.clone())),

        (Vars::ConstInt    , TokenUses::GetLexeme  , digits.clone()),
        (Vars::ConstFloat  , TokenUses::GetLexeme  , digits.clone()|Regex::new(b'.')|digits.clone()),
        (Vars::ConstChar   , TokenUses::GetLexeme  , Regex::new(b'\'')|Regex::any()|Regex::new(b'\'')),
        
        (Vars::Add, TokenUses::Default, Regex::new(b'+')),
        (Vars::Sub, TokenUses::Default, Regex::new(b'-')),
        (Vars::Mul, TokenUses::Default, Regex::new(b'*')),
        (Vars::Div, TokenUses::Default, Regex::new(b'/')),

        (Vars::LessThan     , TokenUses::Default, Regex::new(">")),
        (Vars::GreatThan    , TokenUses::Default, Regex::new("<")),
        (Vars::LessEquals   , TokenUses::Default, Regex::new(">=")),
        (Vars::GreatEquals  , TokenUses::Default, Regex::new("<=")),
        (Vars::Equals       , TokenUses::Default, Regex::new("==")),
        (Vars::NotEquals    , TokenUses::Default, Regex::new("!=")),

        (Vars::RightBrace      , TokenUses::Default, Regex::new(b'}')),
        (Vars::LeftBrace       , TokenUses::Default, Regex::new(b'{')),
        (Vars::RightParentheses, TokenUses::Default, Regex::new(b')')),
        (Vars::LeftParentheses , TokenUses::Default, Regex::new(b'(')),

        (Vars::Comma        , TokenUses::Default, Regex::new(b',')),
        (Vars::Semicolon    , TokenUses::Default, Regex::new(b';')),
        
        (Vars::Ws          , TokenUses::IgnoreThis        , Regex::new(vec![b'\r', b'\n', b'\t', b' ',  b'\0'])),
        (Vars::LineComment , TokenUses::InitInlineComment , Regex::new("//")),
        (Vars::InitComment , TokenUses::InitBlockComment  , Regex::new("/*")),
        (Vars::EndComment  , TokenUses::EndBlockComment   , Regex::new("*/")),
    ];

    
    let rules = vec![
        rule!(Vars::Init => Vars::Id Vars::LeftParentheses Vars::RightParentheses Vars::Block), 
        rule!(Vars::Block => Vars::LeftBrace Vars::VarsDeclarate Vars::SeqCommands Vars::RightBrace; 
                             Vars::Command),
                             
        rule!(Vars::VarsDeclarate => Vars::Let Vars::Id Vars::Set Vars::Expression Vars::Semicolon Vars::VarsDeclarate; ),
        rule!(Vars::SeqCommands => Vars::Command Vars::SeqCommands; ),
        rule!(Vars::Command => Vars::Selection; Vars::Repatation; Vars::Assignment),
        
        rule!(Vars::Selection  => Vars::If Vars::Condition Vars::Block Vars::Selection_),
        rule!(Vars::Selection_ => Vars::Else Vars::Block; ),
        rule!(Vars::Repatation => Vars::While Vars::Condition Vars::Block),
        rule!(Vars::Assignment => Vars::Id Vars::Set Vars::Expression Vars::Semicolon),
        
        rule!(Vars::Expression  => Vars::Term Vars::Expression_),
        rule!(Vars::Expression_ => Vars::Add Vars::Term Vars::Expression_; 
                                   Vars::Sub Vars::Term Vars::Expression_; ),
        
        rule!(Vars::Term  => Vars::Factor Vars::Term_),
        rule!(Vars::Term_ => Vars::Mul Vars::Factor Vars::Term_; 
                             Vars::Div Vars::Factor Vars::Term_; ),
        
        rule!(Vars::Factor => Vars::Id; 
                              Vars::ConstInt; 
                              Vars::ConstFloat; 
                              Vars::ConstChar; 
                              Vars::Sub Vars::Factor; 
                              Vars::LeftParentheses Vars::Expression Vars::RightParentheses),
 
        rule!(Vars::Condition => Vars::Expression Vars::Equals      Vars::Expression;
                                 Vars::Expression Vars::NotEquals   Vars::Expression;
                                 Vars::Expression Vars::LessEquals  Vars::Expression;
                                 Vars::Expression Vars::LessThan    Vars::Expression;
                                 Vars::Expression Vars::GreatEquals Vars::Expression;
                                 Vars::Expression Vars::GreatThan   Vars::Expression)
    ];

}
