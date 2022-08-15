use std::fmt::Debug;
use std::hash::Hash;

use crate::lexica::{
    analysis::LexicalAnalysis,
    tokens::Token
};
use crate::syntax::{
    first::{First, FirstTable}, 
    utils::IsTerminal,
    rules::{Rule, Derivation},
};

pub struct SyntaxAnalysis<T> 
    where 
        T: Eq + Copy + Hash + Debug + IsTerminal
{
    rules: Vec<Rule<T>>,
    firts_table: FirstTable<T>,
    next_token: Option<Token<T>>,
    callback: fn(T, &Derivation<T>) -> Result<(), String>
}

impl<T> SyntaxAnalysis<T> 
    where 
        T: Eq + Copy + Hash + Debug + IsTerminal
{
    pub fn new(rules: Vec<Rule<T>>, callback: fn(T, &Derivation<T>) -> Result<(), String>) -> Self {
        Self {
            firts_table: First::calculate(&rules),
            rules: rules,
            next_token: None,
            callback
        }
    }

    pub fn init(&mut self, initial: T, la: &mut LexicalAnalysis<T>) -> Result<(), String> {
        if initial.is_terminal() {
            return Err(String::from("Error: unable to derive language from a terminal variable"))
        }

        self.next_token = la.next();
        if let Err(error) = self.analysis(initial, la) {
            return Err(error);
        }
        if let None = self.next_token {
            return Err(String::from("Error: Expect end of file, and found another token"));
        }
        return Ok(())
    }

    pub fn evaluate_derivation(&mut self, current: T, derivation: &Derivation<T>, la: &mut LexicalAnalysis<T>) -> Result<(), String> {
        if let Derivation::Normal(vars) = derivation {
            for var in vars {
                if var.is_terminal() {
                    if let Some(token) = &self.next_token  {
                        if *var == token.t_type {
                            self.next_token = la.next();
                            continue;
                        }
                        return Err(String::from(
                            format!("Error({},{}): Expect {:?} and found {:?}", la.row, la.col, *var, token.t_type))
                        );
                    }
                    return Err(String::from(
                        format!("Error({},{}): Expect {:?} and found None", la.row, la.col, *var))
                    );
                } else {
                    let result = self.analysis(*var, la);
                    if let Err(_) = result {
                        return result;
                    }
                }
            }
        }
        return (self.callback)(current, derivation);
    }
    pub fn analysis(&mut self, current: T, la: &mut LexicalAnalysis<T>) -> Result<(), String> {
        if current.is_terminal() {
            return Err(String::from("Error: cannot make derivation a terminal variable"))
        }
        if let Some(token) = &self.next_token {
            let mut contains_empyty = false;
            for rule in self.rules.clone() {
                if rule.from() == current {
                    for derivation in rule.derivations() {
                        match derivation {
                            Derivation::Empty => { 
                                contains_empyty = true 
                            },
                            Derivation::Normal(vars_seq) => { 
                                if First::first_from(&self.firts_table, vars_seq[0], token.t_type) {
                                    return self.evaluate_derivation(current, derivation, la);
                                }
                            }
                        }
                    }
                }
            }
            if !contains_empyty {
                let firts = self.firts_table.get(&current).unwrap();
                return Err(String::from(
                    format!("Error({},{}): Expect {:?} and found {:?}", la.row, la.col, firts, token.t_type))
                );
            }
            return Ok(())
        } else if First::firts_empty(&self.firts_table, current) {
            let firts = self.firts_table.get(&current).unwrap();
            return Err(String::from(
                format!("Error({},{}): Expect {:?} and found None", la.row, la.col, firts))
            );
        }
        Err(String::from(""))
    }
    
}

