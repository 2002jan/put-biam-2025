use rand::prelude::SliceRandom;
use qap_utils::problem::QapProblem;
use crate::TspAlgorithm;

pub struct RandomSearch;

impl TspAlgorithm for RandomSearch {
    fn run(problem: &QapProblem) -> Vec<usize> {
        let mut current_solution = (0..problem.size).collect::<Vec<usize>>();

        current_solution.shuffle(&mut rand::rng());

        current_solution
    }

    fn name() -> String {
        "Random Search".to_string()
    }

    fn snaked_name() -> String {
        "random_search".to_string()
    }
}