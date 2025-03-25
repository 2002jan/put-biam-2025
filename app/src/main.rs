use std::path::Path;
use qap_algos::local_search::local_search::LocalSearch;
use qap_algos::local_search::search_types::greedy::GreedyLocalSearch;
use qap_algos::local_search::search_types::steepest::SteepestLocalSearch;
use qap_algos::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use qap_algos::test_algorithm::test_qap_algorithm;
use qap_utils::problem_loader::{load_from_file, load_optimal_solution};
use run_utils::args::Args;

fn main() {
    let args = Args::build();

    let problem = load_from_file(&args.file);

    let solution_path = match args.solution_file {
        None => None,
        Some(path) => Some(Path::new(&path).to_path_buf())
    };

    let best_solution = load_optimal_solution(solution_path, Path::new(&args.file), &problem);

    test_qap_algorithm::<LocalSearch<GreedyLocalSearch, RandomStartingSolution>>(&problem, true);
    test_qap_algorithm::<LocalSearch<SteepestLocalSearch, RandomStartingSolution>>(&problem, true);
}
