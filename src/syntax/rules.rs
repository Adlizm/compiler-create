use std::hash::Hash;
use crate::syntax::utils::IsTerminal;

#[derive (Clone)]
pub enum Derivation<T>
    where
        T: Hash + Eq + Copy + IsTerminal,
{
    Empty,
    Normal(Vec<T>)
}

#[derive (Clone)]
pub struct Rule<T>
    where
        T: Hash + Eq + Copy + IsTerminal
{
    from: T,
    to: Vec<Derivation<T>>
}
impl<T> Rule<T> 
    where 
        T: Hash + Eq + Copy + IsTerminal
{
    pub fn new(from: T, to: Vec<Derivation<T>>) -> Self {
        Self { from, to}
    }

    pub fn derivations(&self) -> &Vec<Derivation<T>>{
        &self.to
    }
    pub fn from(&self) -> T {
        self.from
    }
}

#[macro_export]
macro_rules! rule {
    ( $name:expr => $($( $x:expr )*); +  ) => {
        {
            use compiler_create::syntax::rules::{Rule, Derivation};
            let mut rules_vec = Vec::new();

            $(
                let mut count = 0;
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                    count += 1;
                )*

                if count == 0 {
                    rules_vec.push(Derivation::Empty);
                }else {
                    rules_vec.push(Derivation::Normal(temp_vec));
                }
            )+

            if IsTerminal::is_terminal($name) {
                panic!("Var is a terminal and cannot been derived");
            }
            Rule::new($name, rules_vec)
        }
    };
}