pub const DEFAULT: u8 = 1;
pub const GET_LEXEME: u8 = 2;
pub const IGNORE_THIS: u8 = 3;
pub const INIT_BLOCK_COMMENT: u8 = 4;
pub const END_BLOCK_COMMENT: u8 = 5;
pub const INIT_INLINE_COMMENT: u8 = 6;


#[derive(Debug)]
pub struct Token{
    //pub t_type: i16,
    pub t_type: Option<String>,
    pub t_name: Option<String>
}

