pub const USE_DEFAULT: u8 = 1;
pub const USE_GET_LEXEME: u8 = 2;
pub const USE_IGNORE_THIS: u8 = 4;
pub const USE_INIT_COMMENT: u8 = 12;
pub const USE_END_COMMENT: u8 = 0;

pub struct Token{
    pub t_type: i16,
    pub t_name: Option<String>
}

