use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::lexica::analysis::LexicalAnalysis;
use crate::lexica::tokens::Token;

#[derive (Clone)]
pub enum Transition<T> 
    where 
        T: Eq + Copy + Hash + Debug
{
    Empty,
    Normal(Vec<T>)
}

#[derive (Clone, Copy, PartialEq, Eq)]
enum First<T> 
    where 
        T: Eq + Copy + Hash + Debug   
{
    Empty,
    Terminal(T)
}


pub struct SyntaxAnalysis<T> 
    where 
        T: Eq + Copy + Hash + Debug
{
    not_terminals: Vec<T>,
    rules: HashMap<T, Vec<Transition<T>>>,
    first: HashMap<T, Vec<First<T>>>,
    next: Option<Token<T>>
}

impl<T> SyntaxAnalysis<T> 
    where 
        T: Eq + Copy + Hash + Debug
{
    pub fn new(rules: Vec<(T, Vec<T>)>) -> Self {
        let mut sa = Self {
            not_terminals: Vec::new(),
            rules: HashMap::new(),
            first: HashMap::new(),
            next: None,
        };

        for (var, rule) in &rules {
            if !sa.not_terminals.contains(var) {
                sa.not_terminals.push(*var);
                sa.rules.insert(*var, Vec::new());
                sa.first.insert(*var, Vec::new());
            }
            if rule.is_empty() {
                sa.rules.get_mut(var).unwrap().push(Transition::Empty);
                sa.first.get_mut(var).unwrap().push(First::Empty);
                continue;
            } 
            sa.rules.get_mut(var).unwrap().push(Transition::Normal(rule.clone()));
        }
        for (var, rule) in &rules {
            if !rule.is_empty() && !sa.not_terminals.contains(&rule[0]){
                sa.first.get_mut(var).unwrap().push(First::Terminal(rule[0]));
            }
        } 

        sa.calculate_first();
        
        return sa;
    }

    pub fn init(&mut self, initial: T, la: &mut LexicalAnalysis<T>) -> Result<(), String> {
        if !self.not_terminals.contains(&initial) {
            return Err(String::from("Error: unable to derive language from a terminal variable"))
        }
        self.next = la.next();
        if let Err(error) = self.analysis(initial, la) {
            return Err(error);
        }

        if let None = self.next {
            return Err(String::from("Error: Epected end of file, and found another token"));
        }
        return Ok(())
    }

    fn calculate_first(&mut self) {
        loop {
            let mut appended = false;

            for (var, rule) in &self.rules {
                let mut var_firsts: Vec<First<T>> = self.first.get(&var).unwrap().clone();
                'step1: for transition in rule {
                            if let Transition::Normal(vec) = transition {
                                for next in vec {
                                    if self.not_terminals.contains(&next) {
                                        let next_firsts: Vec<First<T>> = self.first.get(&next).unwrap().clone();
                                        for next_f in &next_firsts {
                                            if !var_firsts.contains(next_f) && (*next_f) != First::Empty {
                                                var_firsts.push(*next_f);
                                                appended = true;
                                            }
                                        }
                                        if !next_firsts.contains(&First::Empty) {
                                            continue 'step1;
                                        }
                                    } else {
                                        if !var_firsts.contains(&First::Terminal(*next)) {
                                            var_firsts.push(First::Terminal(*next));
                                            appended = true;
                                        }
                                        continue 'step1;
                                    }
                                }
                                if !var_firsts.contains(&First::Empty) {
                                    var_firsts.push(First::Empty);
                                    appended = true;
                                }
                            }
                        }
                self.first.insert(*var, var_firsts);
            }

            if !appended {
                break;
            }
        }
    }

    fn first_from(&self, token: T, var: T) -> bool {
        ( self.not_terminals.contains(&var) && self.first.get(&var).unwrap().contains(&First::Terminal(token)) ) 
        || var == token
    }

    fn analylis_rule(&self, rule: &Transition<T>, la: &mut LexicalAnalysis<T>) ->Result<(), String> {
        todo!()
    }
    fn analysis(&self, var: T, la: &mut LexicalAnalysis<T>) -> Result<(), String> {
        if let Some(token) = &self.next {
            if self.first_from(token.t_type, var) {
                for rule in self.rules.get(&var).unwrap() {
                    match rule {
                        Transition::Empty => {}
                        Transition::Normal(vars) => {
                            if self.first_from(token.t_type, vars[0]) {
                                return self.analylis_rule(rule, la);
                            }
                        },
                    }
                }
            }
            if self.first.get(&var).unwrap().contains(&First::Empty) {
                return Ok(());
            }
            return Err(format!("Error({},{}): Token {:?} not expected", la.row, la.col, token.t_type));
        } else if !self.first.get(&var).unwrap().contains(&First::Empty) {
            return Err(format!("Error({},{}): tokens expected and not found", la.row, la.col));
        }
        return Ok(())
    }
    

    
}

