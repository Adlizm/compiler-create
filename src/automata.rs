use std::{collections::HashMap, ops::Deref};
use crate::regex::Regex;

pub const INIT_STATE: i16 = 0; 
pub const ERROR_STATE: i16 = -1;


struct TokensNFA {
    pub states: i16,
    pub finals: Vec<i16>,
    pub transitions: HashMap<(i16, u8), Vec<i16>>,
    pub token_state: HashMap<String, Vec<i16>>
}

impl TokensNFA {
    pub fn new(tokens_regexs: HashMap<String, Regex>) -> Self {
        let mut transitions = HashMap::<(i16, u8), Vec<i16>>::new();
        let mut token_state: HashMap::<String, Vec<i16>> = HashMap::new();
        let mut finals = Vec::new();
        let mut states = 1;

        for (token_name, regex) in tokens_regexs{
            let current_state = INIT_STATE;
            let mut finals_states = 
                TokensNFA::include_regex(&regex, &mut states, current_state, &mut transitions);
            finals.append(&mut finals_states);
            token_state.insert(token_name, finals_states);
        }

        Self { states, finals, transitions, token_state }
    }

    fn include_regex(regex: &Regex, states: &mut i16, mut current_state: i16, transitions: &mut HashMap<(i16, u8), Vec<i16>>) -> Vec<i16>{
        match regex {
            Regex::Concat(regexs) => {
                let mut ends = TokensNFA::include_regex(&regexs[0], states, current_state, transitions);
                for re in regexs[1..].into_iter() {
                    let mut next = Vec::new();
                    for current in ends {
                        next.append(&mut TokensNFA::include_regex(re, states, current, transitions));
                    }
                    ends = next;
                }
                return ends;
            },
            Regex::Union(regexs) => {
                let mut ends = Vec::new();
                for re in regexs.deref() {
                    let mut next = TokensNFA::include_regex(re, states, current_state, transitions);
                    ends.append(&mut next);
                }
                return ends;
            },
            Regex::Repeat(re) => {
                TokensNFA::include_regex(&*re, states, current_state, transitions);
                return vec![current_state]
            },
            Regex::Terminal(char) => {
                let next = *states;
                if transitions.contains_key(&(current_state, *char)) {
                    transitions.get_mut(&(current_state, *char)).unwrap().push(next);
                } else {
                    transitions.insert((current_state, *char), vec![next]);
                    *states += 1;
                }
                current_state = next;
                return vec![current_state];
            },
        }
        
    }
}


pub struct TokensDFA {
    pub states: u16,
    pub finals: Vec<bool>,
    pub table: Vec<[i16; 256]>,
    pub token_state: Vec<Option<String>>
}

impl TokensDFA {
    pub fn new(tokens_regexs: HashMap<String, Regex>) -> Self {
        let NFA = TokensNFA::new(tokens_regexs);
        let AFD = TokensDFA::get_afd(&NFA.finals, &NFA.token_state, NFA.transitions);
        
        let states = AFD.0.len() as u16;
        let mut finals = vec![false; states as usize];
        let mut token_state = vec![None; states as usize];
        
        for index in AFD.1 {
            finals[index as usize] = true;
        }
        for (s, t) in AFD.2 {
            token_state[t as usize] = Some(s);
        }
        Self { 
            states, 
            finals,
            table: AFD.0, 
            token_state,
        }
    }

    fn get_afd(finals: &Vec<i16>, token_state: &HashMap::<String, Vec<i16>>, transitions: HashMap<(i16, u8), Vec<i16>>) 
        -> (Vec<[i16; 256]>, Vec<i16>, HashMap<String, i16>) {
            
        let mut table: Vec<[i16; 256]> = Vec::new();
        let mut new_finals: Vec<i16> = Vec::new();
        let mut new_tokens: HashMap::<String, i16> = HashMap::new();

        let mut states = vec![vec![INIT_STATE]];
        for i in 1..states.len() {
            let mut new_state = Vec::new(); 
            for letter in 0..=255{
                for current in &states[i] {
                    if transitions.contains_key(&(*current, letter)) {
                        for next in transitions.get(&(*current, letter)).unwrap() {
                            if !new_state.contains(next) {
                                new_state.push(*next);
                            }
                        }
                    }
                }

                new_state.sort();
                let mut index = None; 
                for i in 0..states.len() {
                    if states[i] == new_state {
                        index = Some(i);
                    }
                }
                if let None = index { 
                    let index = states.len();
                    states.push(new_state.clone());
                    if new_state.iter().any(|state| finals.contains(state)) {
                        new_finals.push(index as i16);
                        for (t, s) in token_state {
                            if s.iter().any(|state| finals.contains(state)) {
                                new_tokens.insert(t.to_string(), index as i16);
                            }
                        }
                    }
                }
                if table.len() <= i {
                    table.push([ERROR_STATE; 256]);
                }
                table[i][usize::from(letter)] = index.unwrap() as i16;
            }
        } 

        return (table, new_finals, new_tokens);
    }
}

type SyntaxTable = TokensDFA;