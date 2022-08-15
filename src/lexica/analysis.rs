use std::{fs::File, io::Read};
use std::hash::Hash;

use crate::lexica::regex::Regex;
use crate::lexica::automata::{TokensDFA, ERROR_STATE, INIT_STATE};
use crate::lexica::tokens::{Token, TokenUses};


const MAX_SIZE_LEXEME: usize = 512;
pub struct LexicalAnalysis<T> where T: Eq + Copy + Hash {
    pub row: u16, 
    pub col: u16,
    init: usize,
    next: usize,
    c: u8,
    
    buffer: [u8; (MAX_SIZE_LEXEME as usize) * 2],
    bytes_loaded: usize,
    buffer_loaded: bool,
    last_buffer: bool,


    file: File,
    dfa_tokens: TokensDFA<T>,
    dfa_end_comment: Option<TokensDFA<u32>>
}

impl<T> LexicalAnalysis<T> where T: Eq + Copy + Hash {
    pub fn new(tokens_regexs: Vec<(T, TokenUses, Regex)>, filepath: &str) -> Self {       
        let mut dfa_end_comment = None;
        for (_,mask, regex) in &tokens_regexs {
            if *mask == TokenUses::EndBlockComment {
                dfa_end_comment = Some(
                    TokensDFA::new(vec![(0, *mask, regex.clone())]
                ));
            }
        }
        let dfa = TokensDFA::new(tokens_regexs);

        let mut sa = LexicalAnalysis {
            row: 0, 
            col: 0,
            init: 0,
            next: 0,
            c: 0,
            buffer: [0; MAX_SIZE_LEXEME * 2],
            bytes_loaded: 0,
            last_buffer: false,
            buffer_loaded: true,
            file: File::open(filepath).unwrap(),
            dfa_tokens: dfa,
            dfa_end_comment,
        };
        LexicalAnalysis::next_char(&mut sa);
        return sa;
    }

    pub fn next_char(&mut self) -> u8 {
        if (self.next - self.init + 2 * MAX_SIZE_LEXEME) % (2 * MAX_SIZE_LEXEME) > MAX_SIZE_LEXEME {
            panic!("\nError ({},{}): Lexema execede o tamanho de caracteres permitidos! ", self.row, self.col);
        }
        if self.bytes_loaded == 0 {
            if self.buffer_loaded {
                self.bytes_loaded = self.file.read(&mut self.buffer[0..MAX_SIZE_LEXEME]).unwrap();
            } else {
                self.bytes_loaded = self.file.read(&mut self.buffer[MAX_SIZE_LEXEME..]).unwrap();
            }
            self.last_buffer = self.bytes_loaded < MAX_SIZE_LEXEME;
            self.buffer_loaded = !self.buffer_loaded;
        }
        self.c = if self.bytes_loaded <= 0 { 255 } else { self.buffer[self.next] };

        self.row = if self.c == b'\n' { self.row + 1 } else { self.row };
        self.col = if self.c == b'\n' { 1 } else { self.col + 1 };
        self.next = (self.next + 1) % (2 * MAX_SIZE_LEXEME);
        self.bytes_loaded = if self.bytes_loaded == 0 { 0 } else { self.bytes_loaded - 1 };
        return self.c;
    }

    fn next_token(&mut self) -> Option<Token<T>> {
        if self.bytes_loaded == 0 && self.last_buffer {
            return None;
        }
        let mut old_state = ERROR_STATE;
        let mut state = INIT_STATE;
        
        loop {
            old_state = state;
            state = self.dfa_tokens.transitions[state as usize][self.c as usize];
            
            if state == ERROR_STATE {
                break;
            }
            self.next_char();
        }

        if let (Some(token_type), mask) = &self.dfa_tokens.finals[old_state as usize] {
            let t_name = Some(self.get_string_buffer());
            self.init = self.next - 1;

            match *mask {
                TokenUses::IgnoreThis => { },
                TokenUses::InitBlockComment => { 
                    self.handle_comment(); 
                },
                TokenUses::InitInlineComment => { 
                    self.handle_inline_comment(); 
                },
                TokenUses::EndBlockComment => { 
                    panic!("Error({},{}): comment not started", self.row, self.col);
                },
                TokenUses::GetLexeme => { 
                    return Some(Token { t_type: *token_type, t_name }); 
                },
                TokenUses::Default => { 
                    return Some(Token { t_type: *token_type, t_name: None }); 
                }
            }
            return self.next_token();
        } else {
            panic!("Error({},{}): charcter '{}' not expect", self.row, self.col, self.c);
        }
    }

    fn handle_inline_comment(&mut self) {
        let init = self.init;
        while self.c != b'\n' && self.c != 255  {  
            self.next_char();
            self.init = self.next;
        }
        let aux = self.init;
        self.init = init;
        self.init = aux
    }

    fn handle_comment(&mut self) {
        let mut final_state = ERROR_STATE;
        if let Some(dfa) = &self.dfa_end_comment {
            for (state, (token, _ ) ) in dfa.finals.iter().enumerate() {
                if let Some(_) = *token {
                    final_state = state as i32;
                }
            }
        }else {
            panic!("Error: A token of INIT_BLOCK_COMMENT has been specificted and a token END_BLOCK_COMMENT hasn't specificted");
        }
        
        'init: loop {  
            let mut state = INIT_STATE;
            loop {
                if state == final_state {
                    break 'init;
                }
                state = self.dfa_end_comment.as_ref().unwrap().transitions[state as usize][self.c as usize];
                self.next_char();
                if state == ERROR_STATE {
                    break;
                }
            }
        }
    }

    fn get_string_buffer(&self) -> String {
        let length = (self.next - 1 - self.init + 2 * MAX_SIZE_LEXEME) % (2 * MAX_SIZE_LEXEME);
        let mut j = self.init;
    
        let mut buf = Vec::new();
        for _ in 0..length {
            buf.push(self.buffer[j]);
            j = (j + 1) % (2 * MAX_SIZE_LEXEME);
        }
        return std::str::from_utf8(buf.as_slice()).unwrap().to_string()
    }
}

impl<T> Iterator for LexicalAnalysis<T> where T: Eq + Copy + Hash {
    type Item = Token<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}