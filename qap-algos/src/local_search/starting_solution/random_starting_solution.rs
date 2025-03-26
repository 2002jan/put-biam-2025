use crate::local_search::starting_solution::StartingSolution;
use qap_utils::problem::QapProblem;
use rand::seq::SliceRandom;
use rand::rng;

pub struct RandomStartingSolution;

impl StartingSolution for RandomStartingSolution {
    fn get_starting_solution(problem: &QapProblem) -> Vec<usize> {
        let mut nodes: Vec<usize> = (0..problem.size).collect::<Vec<usize>>();
        nodes.shuffle(&mut rng());

        nodes
    }

    fn name() -> String {
        String::from("Random Start")
    }

    fn snaked_name() -> String {
        String::from("random_start")
    }
}