use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use super::rules::{Rule, Derivation};
use super::utils::IsTerminal;

#[derive (Clone, Copy, PartialEq, Eq)]
pub enum First<T> 
    where 
        T: Eq + Copy + Hash + Debug + IsTerminal
{
    Empty,
    Terminal(T)
}

pub type FirstTable<T> = HashMap::<T, Vec<First<T>>>;

impl<T> First<T> 
    where 
        T: Eq + Copy + Hash + Debug + IsTerminal
{
    pub fn first_from(firts_table: &FirstTable<T>, var: T, other: T) -> bool {
        if other.is_terminal() {
            if var.is_terminal() {
                return other == var;
            } else {
                return firts_table.contains_key(&var) && firts_table.get(&var).unwrap().contains(&First::Terminal(other))
            }
        }
        return false;
    }
    pub fn firts_empty(firts_table: &FirstTable<T>, var: T) -> bool {
        if var.is_terminal() {
            return false;
        } else {
            return firts_table.contains_key(&var) && firts_table.get(&var).unwrap().contains(&First::Empty)
        }
    }

    pub fn calculate(rules: &Vec<Rule<T>>) -> FirstTable<T> {
        let mut first: FirstTable<T> = FirstTable::new();

        for rule in rules {
            if !first.contains_key(&rule.from()) {
                first.insert(rule.from() , Vec::new());
            }
            for derivation in rule.derivations() {
                if let Derivation::Normal(var) = derivation {
                    if !var.is_empty() && var[0].is_terminal() {
                        first.get_mut(&rule.from()).unwrap().push(First::Terminal(var[0]))
                    }
                }
            }
        } 

        loop {
            let mut appended = false;

            for rule in rules {
                let mut var_firsts: Vec<First<T>> = first.get(&rule.from()).unwrap().clone();

                'step1: for derivation in rule.derivations() {
                            if let Derivation::Normal(vars) = derivation {
                                for var in vars {
                                    if !var.is_terminal() {
                                        let next_firsts: Vec<First<T>> = first.get(var).unwrap().clone();
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
                                        if !var_firsts.contains(&First::Terminal(*var)) {
                                            var_firsts.push(First::Terminal(*var));
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
                first.insert(rule.from(), var_firsts);
            }

            if !appended {
                break;
            }
        }

        return first
    }
}

impl<T> Debug for First<T> 
    where 
        T: Eq + Copy + Hash + Debug + IsTerminal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "{{}}"),
            Self::Terminal(terminal) => terminal.fmt(f)
        }
    }
}