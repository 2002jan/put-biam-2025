use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::search_types::tabu_search::tabu_list::TabuList;
use crate::local_search::search_types::LocalSearchType;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use std::collections::BTreeMap;

pub mod tabu_list;

const MAX_NO_IMPROVEMENT: u32 = 100;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct CandidateTabuMove {
    mov: Move,
    delta: i32,
}

pub struct TabuSearch<TL: TabuList> {
    current_start: usize,
    next_start: usize,
    next_target: usize,
    size: usize,
    list: TL,
}

impl<TL: TabuList> TabuSearch<TL> {
    fn get_candidate_moves(&mut self, current_solution: &Vec<usize>, problem: &QapProblem, mut recorder: &mut Option<&mut AlgorithmRunStatsRecorder>) -> BTreeMap<i32, CandidateTabuMove> {
        let mut moves = BTreeMap::new();

        self.reset_iterator();

        while let Some(mov) = self.next() {
            let delta = LocalSearchNeighbourhood::evaluate_move(problem, &mov, current_solution);

            if let Some(rec) = &mut recorder {
                rec.record_partial_evaluation();
            }

            moves.insert(delta, CandidateTabuMove {
                mov,
                delta,
            });
        }

        moves
    }
}

impl<TL: TabuList> LocalSearchType for TabuSearch<TL> {
    fn new(problem: &QapProblem) -> Self {
        TabuSearch {
            current_start: 0,
            next_start: 0,
            next_target: 0,
            size: problem.size,
            list: TL::new((problem.size / 4) as u32),
        }
    }

    fn run(problem: &QapProblem, starting_solution: Vec<usize>, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut best_solution = starting_solution.clone();
        let mut current_solution = starting_solution;

        let mut best_solutions_cost = evaluate_solution(&best_solution, problem);
        let mut current_solution_cost = best_solutions_cost;

        if let Some(rec) = &mut recorder {
            rec.record_evaluation();
            rec.record_iteration(current_solution_cost)
        }

        let mut iteration: u32 = 0;
        let mut no_improvement_count: u32 = 0;

        let k = problem.size / 10;

        let mut tabu_search_iterator = Self::new(problem);

        // let mut rng = rng();

        while no_improvement_count < MAX_NO_IMPROVEMENT {
            iteration += 1;

            let candidate_moves = tabu_search_iterator.get_candidate_moves(&current_solution, problem, &mut recorder);

            let elite_candidates: Vec<&CandidateTabuMove> = candidate_moves.iter().take(k).map(|(_, mov)| mov).collect();

            // let sample_size = max(1, (0.2 * candidate_moves.len() as f32).ceil() as usize);
            // let mut sampled_moves = candidate_moves.iter().map(|(_, mov)| mov).choose_multiple(&mut rng, sample_size);
            // sampled_moves.sort_by_cached_key(|mov| mov.delta);
            // let top_sample: Vec<&CandidateTabuMove> = sampled_moves.iter().copied().take((sampled_moves.len() as f32 * 0.2).ceil() as usize).map(|mov| mov).collect();

            let mut best_move: Option<&CandidateTabuMove> = None;
            let mut best_move_delta = i32::MAX;

            for mov in elite_candidates {
                let mov_delta = mov.delta;
                let allowed = if tabu_search_iterator.list.is_move_tabu(&mov.mov, &current_solution, iteration) {
                    current_solution_cost + mov_delta < best_solutions_cost
                } else {
                    true
                };

                if allowed && best_move_delta > mov_delta {
                    best_move = Some(mov);
                    best_move_delta = mov_delta;
                }
            }

            if let Some(candidate_mov) = best_move {
                LocalSearchNeighbourhood::apply_move(&candidate_mov.mov, &mut current_solution);
                current_solution_cost += candidate_mov.delta;

                tabu_search_iterator.list.finish_iteration();
                tabu_search_iterator.list.add_to_memory(&candidate_mov.mov, &current_solution, iteration);


                if let Some(rec) = &mut recorder {
                    rec.record_iteration(current_solution_cost);
                }

                if current_solution_cost < best_solutions_cost {
                    best_solutions_cost = current_solution_cost;
                    best_solution = current_solution.clone();
                } else {
                    no_improvement_count += 1;
                }
            } else {
                break;
            }
        }

        best_solution
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
        String::from("Tabu search with ") + TL::name().as_str()
    }

    fn snaked_name() -> String {
        String::from("tabu_search_with") + TL::snaked_name().as_str()
    }
}