use qap_utils::Solution;
use crate::local_search::neighbourhoods::Move;
use crate::local_search::search_types::tabu_search::tabu_list::memory::TabuListMemory;
use crate::local_search::search_types::tabu_search::tabu_list::TabuList;

pub struct MoveTabuList<M: TabuListMemory<Move>> {
    memory: M,
    no_tabu_moves_in_iteration: u32,
}

impl<M: TabuListMemory<Move>> TabuList for MoveTabuList<M> {
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

    fn is_move_tabu(&self, mov: &Move, _: &Solution, iteration: u32) -> bool {
        self.memory.is_tabu(mov, iteration)
    }

    fn add_to_memory(&mut self, mov: &Move, _: &Solution, iteration: u32) {
        self.memory.add(mov.clone(), iteration)
    }

    fn name() -> String {
         M::name() + " Move tabu list"
    }

    fn snaked_name() -> String {
         M::snaked_name() + "move_tabu_list"
    }
}