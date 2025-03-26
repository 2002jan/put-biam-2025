use std::time::Instant;
use rand::Rng;
use qap_utils::problem::QapProblem;
use crate::local_search::starting_solution::StartingSolution;
use crate::TspAlgorithm;

pub struct RandomWalk<
    SS: StartingSolution,
>;

const TIME_LIMIT: u128 = 1000;

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
    fn run(problem: &QapProblem) -> Vec<usize> {
        let mut current_solution = SS::get_starting_solution(problem);

        let start = Instant::now();

        while start.elapsed().as_millis() < TIME_LIMIT {
            //     get two indices
            let (i, j) = generate_distinct_pair(problem.size);

            apply_move(&mut current_solution, i, j);
        }

        current_solution

    }

    fn name() -> String {
        "Random Walk".to_string()
    }

    fn snaked_name() -> String {
        "random_walk".to_string()
    }
}