use qap_utils::problem::QapProblem;

pub mod random_starting_solution;

pub trait StartingSolution {
    fn get_string_solution(problem: &QapProblem) -> Vec<usize>;
    fn name() -> String;
    fn snaked_name() -> String;
}