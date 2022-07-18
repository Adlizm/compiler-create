use std::{collections::HashMap, fs::File, io::Read};
use crate::automata::{TokensDFA, ERROR_STATE, INIT_STATE};
use crate::tokens::{Token, USE_GET_LEXEME, USE_INIT_COMMENT, USE_IGNORE_THIS, USE_END_COMMENT};

const MAX_SIZE_LEXEME: usize = 512;
pub struct SyntaxAnalysis {
    pub row: u16, 
    pub col: u16,
    init: usize,
    next: usize,

    char: u8,
    bytes_loaded: u16,

    buffer: [u8; (MAX_SIZE_LEXEME as usize) * 2],
    buffer_loaded: bool,

    file: File,
    tokens_attrs: HashMap<String, u8>,
    dfa: TokensDFA,
}

impl SyntaxAnalysis {
    pub fn new(dfa: TokensDFA, tokens_attrs: HashMap<String, u8>, filepath: &str) -> Self {
        let file = File::open(filepath).unwrap();
        
        let mut sa = SyntaxAnalysis {
            row: 0, 
            col: 0,
            init: 0,
            next: 0,
            char: 0,
            buffer: [0; MAX_SIZE_LEXEME * 2],
            bytes_loaded: 0,
            buffer_loaded: true,
            file,
            tokens_attrs,
            dfa,
        };
        SyntaxAnalysis::next_char(&mut sa);
        return sa;
    }

    pub fn next_char(&mut self) -> u8 {
        if (self.next - self.init + 2 * MAX_SIZE_LEXEME) % (2 * MAX_SIZE_LEXEME) > MAX_SIZE_LEXEME {
            println!("\nError ({},{}): Lexema execede o tamanho de caracteres permitidos! ", self.row, self.col);
            return 0;
        }
        if self.bytes_loaded == 0 {
            if self.buffer_loaded {
                self.bytes_loaded = self.file.read(&mut self.buffer[0..MAX_SIZE_LEXEME]).unwrap() as u16;
            } else {
                self.bytes_loaded = self.file.read(&mut self.buffer[MAX_SIZE_LEXEME..]).unwrap() as u16;
            }
            self.buffer_loaded = !self.buffer_loaded;
        }
        self.char = if self.bytes_loaded <= 0 { 255 } else { self.buffer[self.next] };
        
        self.row = if self.char == b'\n' { self.row + 1 } else { self.row };
        self.col = if self.char == b'\n' { 1 } else { self.col + 1 };
        self.next = (self.next + 1) % (2 * MAX_SIZE_LEXEME);
        self.bytes_loaded -= 1;
        return self.char;
    }

    pub fn next_token(&mut self) -> Token {
        let mut state = INIT_STATE;
        let mut oldState = ERROR_STATE;
        loop {
            oldState = state;
            state = self.dfa.table[state as usize][self.char as usize];
            
            if state == ERROR_STATE {
                break;
            }
            self.next_char();
        }

        if self.dfa.finals[oldState as usize] {
            let token_type = self.dfa.token_state[oldState as usize].as_ref().unwrap();
            let uses = self.tokens_attrs.get(token_type).unwrap();
            let init = self.init;
            
            self.init = self.next;
            if uses & USE_INIT_COMMENT != 0 { 
                self.handle_comment();
                return self.next_token();
            } else if uses & USE_IGNORE_THIS != 0 {
                return self.next_token();
            }
            
            if uses & USE_GET_LEXEME != 0 {
                let t_name = Some(get_string_buffer(&self.buffer, init, self.next));
                Token { t_type: oldState, t_name }
            } else { 
                Token { t_type: oldState, t_name: None } 
            }
        }else {
            panic!("Error({},{}): charcter '{}' not expect", self.row, self.col, self.char);
        }
    }

    pub fn handle_comment(&mut self) {
        let mut final_state = ERROR_STATE;
        'find_state: for (s, t) in &self.tokens_attrs {
            if t & USE_END_COMMENT != 0 {
                for (index, v) in self.dfa.token_state.iter().enumerate(){
                    if let Some(string) = v {
                        if string == s {
                            final_state = index as i16;
                            break 'find_state;
                        }
                    }
                }
            }
        }
        if final_state == ERROR_STATE {
            panic!("Error: A token of INIT_COMMENT has been specificted and a token END_COMMENT hasn't specificted");
        }
        
        'init: loop {  
            let mut state = INIT_STATE;
            while state != ERROR_STATE  {
                state = self.dfa.table[state as usize][self.char as usize];
                self.next_char();
                if state == final_state{
                    break 'init;
                }
                self.init = self.next;
            }
        }
    }
}

fn get_string_buffer(buffer: &[u8], init: usize, end: usize) -> String{
    let length = (end - 1 - init + 2 * MAX_SIZE_LEXEME) % (2 * MAX_SIZE_LEXEME);
    let j = init;

    let mut buf = [0 as u8; MAX_SIZE_LEXEME];
    for _ in 0..length {
        buf[j]= buffer[j % 2 * MAX_SIZE_LEXEME];
    }
    return std::str::from_utf8(&buf).unwrap().to_string()
}