use std::hash::Hash;

pub mod long_term;
pub mod short_term;

pub trait TabuListMemory<T: Hash + Eq> {
    fn new(tenure: u32) -> Self;
    fn is_tabu(&self, item: &T, iteration: u32) -> bool;
    fn add(&mut self, item: T, iteration: u32);
    fn clear(&mut self);

    fn name() -> String;
    fn snaked_name() -> String;
}