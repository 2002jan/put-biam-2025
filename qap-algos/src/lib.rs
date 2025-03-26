pub mod local_search;
pub mod test_algorithm;
pub mod random_algos;

use qap_utils::problem::QapProblem;

pub trait TspAlgorithm {
    fn run(problem: &QapProblem) -> Vec<usize>;

    fn name() -> String;

    fn snaked_name() -> String;
}