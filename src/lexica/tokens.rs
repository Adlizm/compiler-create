use std::hash::Hash;
use std::fmt::Debug;

pub type TokenUses = u32;
pub const DEFAULT: TokenUses = 1;
pub const GET_LEXEME: TokenUses = 2;
pub const IGNORE_THIS: TokenUses = 3;
pub const INIT_BLOCK_COMMENT: TokenUses = 4;
pub const END_BLOCK_COMMENT: TokenUses = 5;
pub const INIT_INLINE_COMMENT: TokenUses = 6;


#[derive(Debug)]
pub struct Token<T>
    where T: Eq + Copy + Hash + Debug
{
    pub t_type: T,
    pub t_name: Option<String>
}