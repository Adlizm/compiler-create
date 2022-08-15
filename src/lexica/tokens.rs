use std::hash::Hash;

#[derive (Clone, Copy, PartialEq, Eq)]
pub enum TokenUses {
    Default = 1,
    GetLexeme =  2,
    IgnoreThis =  3,
    InitBlockComment =  4,
    EndBlockComment =  5,
    InitInlineComment =  6
} 


#[derive(Debug)]
pub struct Token<T>
    where T: Eq + Copy + Hash
{
    pub t_type: T,
    pub t_name: Option<String>
}