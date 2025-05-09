pub mod greedy;
pub mod steepest;
pub mod tabu_search;

use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::problem::QapProblem;
use crate::local_search::neighbourhoods::Move;

pub trait LocalSearchType {
    fn new(problem: &QapProblem) -> Self;
    fn run(problem: &QapProblem, starting_solution: Vec<usize>, recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize>;

    fn reset_iterator(&mut self);
    fn next(&mut self) -> Option<Move>;

    fn name() -> String;
    fn snaked_name() -> String;
}
