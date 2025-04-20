pub mod memory;
pub mod move_list;
pub mod solution_list;

use qap_utils::Solution;
use crate::local_search::neighbourhoods::Move;

pub trait TabuList {
    fn new(tenure: u32) -> Self;
    fn finish_iteration(&mut self);
    fn is_move_tabu(&self, mov: &Move, solution: &Solution, iteration: u32) -> bool;
    fn add_to_memory(&mut self, mov: &Move, solution: &Solution, iteration: u32);

    fn name() -> String;
    fn snaked_name() -> String;
}

