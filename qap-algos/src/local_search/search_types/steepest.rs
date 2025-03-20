use qap_utils::problem::QapProblem;
use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::search_types::LocalSearchType;

pub struct SteepestLocalSearch {
    current_start: usize,
    next_start: usize,
    next_target: usize,
    size: usize,
}


impl LocalSearchType for SteepestLocalSearch {
    fn new(problem: &QapProblem) -> Self {
        SteepestLocalSearch {
            current_start: 0,
            next_start: 0,
            next_target: 0,
            size: problem.size,
        }
    }

    fn run(problem: &QapProblem, starting_solution: Vec<usize>) -> Vec<usize> {
        let mut current_solution = starting_solution;

        let mut neighbourhood_iterator = Self::new(problem);

        loop {
            neighbourhood_iterator.reset_iterator();

            let mut best_change = 0;
            let mut bets_move: Option<Move> = None;

            while let Some(mov) = neighbourhood_iterator.next() {
                let change = LocalSearchNeighbourhood::evaluate_move(problem, &mov, &current_solution);

                if change < best_change {
                    best_change = change;
                    bets_move = Some(mov);
                }
            }

            if let Some(mov) = bets_move {
                LocalSearchNeighbourhood::apply_move(&mov, &mut current_solution);
            } else {
                break;
            }
        }

        current_solution
    }

    fn reset_iterator(&mut self) {
        self.current_start = 0;
        self.next_start = 0;
        self.next_target = 0;
    }

    fn next(&mut self) -> Option<Move> {
        if self.next_target >= self.size || self.next_start == 0 {
            self.current_start = self.next_start;
            self.next_start += 1;

            self.next_target = 0;
        }

        if self.next_start >= self.size || self.current_start == self.size - 1 {
            return None;
        }

        if self.next_target == self.current_start {
            self.next_target += 1;
        }

        self.next_target += 1;

        Some(Move {
            p1: self.current_start,
            p2: self.next_target - 1,
        })
    }

    fn name() -> String {
        String::from("Steepest")
    }

    fn snaked_name() -> String {
        String::from("steepest")
    }
}
