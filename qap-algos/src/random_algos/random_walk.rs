use std::marker::PhantomData;
use std::time::Instant;
use rand::Rng;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::starting_solution::StartingSolution;
use crate::TspAlgorithm;

pub struct RandomWalk<
    SS: StartingSolution,
> {
    ss: PhantomData<SS>,
}

const TIME_LIMIT: u128 = 10;

fn generate_distinct_pair(n: usize) -> (usize, usize) {
    let mut rng = rand::rng();
    let first = rng.random_range(0..n);
    let second = (first + rng.random_range(1..n)) % n;
    (first, second)
}

fn apply_move(solution: &mut Vec<usize>, i: usize, j: usize) {
    let temp = solution[i];
    solution[i] = solution[j];
    solution[j] = temp;
}


impl<
    SS: StartingSolution,
> TspAlgorithm for RandomWalk<SS> {
    fn run(problem: &QapProblem, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut current_solution = SS::get_starting_solution(problem);
        let mut current_score = evaluate_solution(&current_solution, problem);

        if let Some(rec) = &mut recorder {
            rec.record_iteration(current_score)
        }

        let start = Instant::now();

        let mut best_score = current_score;
        let mut best_solution = current_solution.clone();

        while start.elapsed().as_millis() < TIME_LIMIT {
            //     get two indices
            let (i, j) = generate_distinct_pair(problem.size);
            let mov = Move {
                p1: i,
                p2: j,
            };

            current_score += LocalSearchNeighbourhood::evaluate_move(problem, &mov, &current_solution);

            apply_move(&mut current_solution, i, j);

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
        "Random Walk".to_string()
    }

    fn snaked_name() -> String {
        "random_walk".to_string()
    }
}