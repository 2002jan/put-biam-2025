use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::search_types::LocalSearchType;

pub struct GreedyLocalSearch {
    current_start: usize,
    next_start: usize,
    next_target: usize,
    random_thread: ThreadRng,
    start_order: Vec<usize>,
    targets: Vec<usize>,
    size: usize,
}


impl LocalSearchType for GreedyLocalSearch {
    fn new(problem: &QapProblem) -> Self {
        let thread_rng = rng();

        let start_order = (0..problem.size).collect::<Vec<usize>>();

        let targets = (0..problem.size).collect::<Vec<usize>>();

        GreedyLocalSearch {
            current_start: 0,
            next_start: 0,
            next_target: 0,
            random_thread: thread_rng,
            start_order,
            targets,
            size: problem.size,
        }
    }

    fn run(problem: &QapProblem, starting_solution: Vec<usize>, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut current_solution = starting_solution;
        let mut current_score = evaluate_solution(&current_solution, problem);

        if let Some(rec) = &mut recorder {
            rec.record_iteration(current_score);
            rec.record_evaluation();
        }

        let mut neighbourhood_iterator = Self::new(problem);


        loop {
            let mut found_better = false;
            neighbourhood_iterator.reset_iterator();

            while let Some(mov) = neighbourhood_iterator.next() {
                let change = LocalSearchNeighbourhood::evaluate_move(problem, &mov, &current_solution);

                if let Some(rec) = &mut recorder {
                    rec.record_partial_evaluation();
                }

                if change < 0 {
                    current_score += change;
                    LocalSearchNeighbourhood::apply_move(&mov, &mut current_solution);

                    if let Some(rec) = &mut recorder {
                        rec.record_iteration(current_score);
                    }

                    found_better = true;
                    break;
                }
            }

            if !found_better {
                break;
            }
        }

        current_solution
    }

    fn reset_iterator(&mut self) {
        self.start_order.shuffle(&mut self.random_thread);
        self.current_start = 0;
        self.next_start = 0;
        self.next_target = 0;
    }

    fn next(&mut self) -> Option<Move> {
        let mut mov: Option<Move> = None;

        'main: while mov.is_none() {
            if self.next_target >= self.size || self.next_start == 0 {
                self.current_start = self.next_start;
                self.next_start += 1;

                self.next_target = 0;
                self.targets.shuffle(&mut self.random_thread);
            }

            if self.next_start >= self.start_order.len() {
                break;
            }

            let current_start = self.start_order[self.current_start];

            let mut current_target = self.targets[self.next_target];
            self.next_target += 1;

            while current_target <= current_start {
                if self.next_target >= self.size {
                    continue 'main;
                }

                current_target = self.targets[self.next_target];
                self.next_target += 1;
            }

            mov = Some(Move {
                p1: current_start,
                p2: current_target,
            })
        }
        mov
    }

    fn name() -> String {
        String::from("Greedy")
    }

    fn snaked_name() -> String {
        String::from("greedy")
    }
}
