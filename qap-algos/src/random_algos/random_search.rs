use std::time::Instant;
use rand::prelude::SliceRandom;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::TspAlgorithm;

pub struct RandomSearch;

const TIME_LIMIT: u128 = 10;

impl TspAlgorithm for RandomSearch {
    fn run(problem: &QapProblem, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut current_solution = (0..problem.size).collect::<Vec<usize>>();
        let mut current_score = evaluate_solution(&current_solution, problem);

        if let Some(rec) = &mut recorder {
            rec.record_iteration(current_score)
        }

        let start = Instant::now();

        let mut best_score = current_score;
        let mut best_solution = current_solution.clone();

        while start.elapsed().as_millis() < TIME_LIMIT {
            current_solution.shuffle(&mut rand::rng());

            current_score = evaluate_solution(&current_solution, problem);

            if current_score < best_score {
                best_score = current_score;
                best_solution = current_solution.clone();
            }

            if let Some(rec) = &mut recorder {
                rec.record_iteration(current_score)
            }
        }

        best_solution
    }

    fn name() -> String {
        "Random Search".to_string()
    }

    fn snaked_name() -> String {
        "random_search".to_string()
    }
}