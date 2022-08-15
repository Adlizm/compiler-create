pub mod analysis;
pub mod first;
pub mod rules;

pub mod utils {
    pub trait IsTerminal {
        fn is_terminal(self) -> bool;    
    }
}