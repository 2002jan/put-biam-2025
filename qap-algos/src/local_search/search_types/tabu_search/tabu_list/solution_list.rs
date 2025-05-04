use qap_utils::Solution;
use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::search_types::tabu_search::tabu_list::memory::TabuListMemory;
use crate::local_search::search_types::tabu_search::tabu_list::TabuList;

pub struct SolutionTabuList<M: TabuListMemory<Solution>> {
    memory: M,
    no_tabu_moves_in_iteration: u32,
}

impl<M: TabuListMemory<Solution>> TabuList for SolutionTabuList<M> {
    fn new(tenure: u32) -> Self {
        Self {
            memory: M::new(tenure),
            no_tabu_moves_in_iteration: 0,
        }
    }

    fn finish_iteration(&mut self) {
        if self.no_tabu_moves_in_iteration == 0 {
            self.memory.clear();
        }

        self.no_tabu_moves_in_iteration = 0;
    }

    fn is_move_tabu(&self, mov: &Move, solution: &Solution, iteration: u32) -> bool {
        let mut solution_after_move = solution.clone();
        LocalSearchNeighbourhood::apply_move(mov, &mut solution_after_move);

        self.memory.is_tabu(&solution_after_move, iteration)
    }

    fn add_to_memory(&mut self, _: &Move, solution: &Solution, iteration: u32) {
        self.memory.add(solution.clone(), iteration);
    }

    fn name() -> String {
        M::name() + " Solution tabu list"
    }

    fn snaked_name() -> String {
        M::snaked_name() + "solution_tabu_list"
    }
}