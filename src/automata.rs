use std::collections::HashMap;
use crate::regex::Regex;

pub const INIT_STATE: i16 = 0; 
pub const ERROR_STATE: i16 = -1;

#[derive(Debug)]
pub struct TokensNFA {
    pub states: i16,
    pub finals: Vec<i16>,
    pub transitions: HashMap<(i16, u8), Vec<i16>>,
    pub token_state: HashMap<String, i16>
}

impl TokensNFA {
    pub fn new(tokens_regexs: Vec<(String, Regex)>) -> Self {
        let mut states = 1;
        let mut finals = Vec::new();
        let mut transitions = HashMap::new();
        let mut token_state = HashMap::new();

        let mut nfa = Self {states, finals, transitions, token_state };
        
        for (token_name, regex) in tokens_regexs{
            let final_state = nfa.include_regex(INIT_STATE, regex);
            nfa.finals.push(final_state);
            nfa.token_state.insert(token_name, final_state);
        }
        return nfa;
    }

    pub fn test_string(self, string: String) -> Option<String> {
        return self.test(string, 0, INIT_STATE);
    }

    fn test(&self, string: String, step: usize, current: i16) -> Option<String>{
        if step >= string.len() && self.finals.contains(&current) {
            for (k, v) in &self.token_state {
                if *v == current {
                    return Some(k.to_string());
                }
            }
        }
        if let Some(nexts) = self.transitions.get(&(current, string.as_bytes()[step as usize])) {
            for next in nexts {
                if let Some(token) = self.test(string.clone(), step + 1, *next ) {
                    return Some(token);
                }
            }
        }
        return None;
    }

    fn include_regex(&mut self,mut current_state: i16, regex: Regex) -> i16{
        match regex {
            Regex::Concat(regexs) => {
                for re in regexs {
                    current_state = self.include_regex(current_state, re);
                }
                return current_state;
            },
            Regex::Union(regexs) => {
                let to = self.states;
                self.states += 1;
                for re in regexs {
                    let from = self.include_regex(current_state, re);
                    self.insert_empty_transition(from, to);
                }
                return to;
            },
            Regex::Repeat(re) => {
                let from = self.include_regex(current_state, *re);
                self.insert_empty_transition(from, current_state);
                return current_state;
            },
            Regex::Word(word) => {
                for char in word {
                    let next = self.states;
                self.states += 1;
                if self.transitions.contains_key(&(current_state, char)) {
                    self.transitions.get_mut(&(current_state, char)).unwrap().push(next);
                } else {
                    self.transitions.insert((current_state, char), vec![next]);
                }
                current_state = next;
                }
                return current_state;
            },
            Regex::Char(char) => {
                let next = self.states;
                self.states += 1;
                if self.transitions.contains_key(&(current_state, char)) {
                    self.transitions.get_mut(&(current_state, char)).unwrap().push(next);
                } else {
                    self.transitions.insert((current_state, char), vec![next]);
                }
                return next;
            },
            Regex::Empty => {
                let to = self.states;
                self.states += 1;
                self.insert_empty_transition(current_state, to);
                return to;
            }
        }
        
    }

    fn insert_empty_transition(&mut self, from: i16, to: i16){
        for (k, v) in self.transitions.iter_mut() {
            for states in v {
                if *states == from {
                    *states = to;
                }
            }
        }
    }
}


#[derive(Debug)]
pub struct TokensDFA {
    pub states: u16,
    pub finals: Vec<bool>,
    pub transitions: Vec<[i16; 256]>,
    pub token_state: Vec<Option<String>>
}

impl TokensDFA {
    pub fn new(nfa: TokensNFA) -> Self {
        let afd = TokensDFA::from_nfa(nfa);

        let states = afd.0.len() as u16;
        let mut finals = vec![false; states as usize];
        let mut token_state = vec![None; states as usize];
        
        for index in afd.1 {
            finals[index as usize] = true;
        }
        for (s, t) in afd.2 {
            token_state[t as usize] = Some(s);
        }
        Self { 
            states, 
            finals,
            transitions: afd.0, 
            token_state,
        }
    }

    pub fn test_string(&self, string: String) -> Option<String> {
        let mut state = INIT_STATE; 
        for char in string.as_bytes(){
            state = self.transitions[state as usize][*char as usize];
            if state == ERROR_STATE {
                return None;
            }
        }
        if let Some(token) = &self.token_state[state as usize]{
            return Some(token.to_string())
        }
        return None;
    }

    fn from_nfa(nfa: TokensNFA) -> (Vec<[i16; 256]>, Vec<i16>, HashMap<String, i16>) {
        let finals = nfa.finals; 
        let token_state = nfa.token_state;
        let transitions = nfa.transitions;

        let mut table: Vec<[i16; 256]> = vec![[ERROR_STATE; 256]];
        let mut new_finals: Vec<i16> = Vec::new();
        let mut new_tokens: HashMap::<String, i16> = HashMap::new();

        let mut states = vec![vec![INIT_STATE]];
        let mut total_states = 1;

        for i in 0.. {
            if i == total_states {
                break;
            }

            for letter in 0..=255 {
                let mut new_state = Vec::new();
                let mut is_final = true;

                for current in &states[i] {
                    if transitions.contains_key(&(*current, letter)) {
                        for next in transitions.get(&(*current, letter)).unwrap() {
                            if !new_state.contains(next) {
                                new_state.push(*next);
                                is_final = is_final && finals.contains(next);
                            }
                        }
                    }
                }
                if new_state.is_empty() {
                    continue;
                }
                new_state.sort();
                if states.contains(&new_state) {
                    for index in 0..total_states {
                        if states[index] == new_state {
                            table[i][letter as usize] = index as i16;
                            break;
                        }
                    }
                } else {
                    if is_final{
                        new_finals.push(total_states as i16);
                        for (t, other) in &token_state {
                            if new_state.contains(other) {
                                new_tokens.insert(t.to_string(), total_states as i16);
                                break;
                            } 
                        }
                    }
                    states.push(new_state);
                    table.push([ERROR_STATE; 256]);
                    table[i][letter as usize] = total_states as i16;                    
                    total_states += 1;
                }
            }
        }
        return (table, new_finals, new_tokens);
    }
}