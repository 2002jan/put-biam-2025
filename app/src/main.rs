use std::cmp::max;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::time::Instant;
use qap_algos::heuristics::greedy_construction_heuristic::GreedyConstructionHeuristic;
use qap_algos::local_search::local_search::LocalSearch;
use qap_algos::local_search::search_types::greedy::GreedyLocalSearch;
use qap_algos::local_search::search_types::steepest::SteepestLocalSearch;
use qap_algos::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use qap_algos::random_algos::random_search::RandomSearch;
use qap_algos::random_algos::random_walk::RandomWalk;
use qap_algos::test_algorithm::test_qap_algorithm;
use qap_utils::problem_loader::{load_from_file, load_optimal_solution};
use run_utils::args::Args;

fn main() {
    let args = Args::build();

    let mut problem = load_from_file(&args.file);

    let solution_path = match args.solution_file {
        None => None,
        Some(path) => Some(Path::new(&path).to_path_buf())
    };

    let best_solution = load_optimal_solution(solution_path, Path::new(&args.file), &problem);
    let problem_name = Path::new(&args.file).file_stem().unwrap().to_str().unwrap();

    let output_path: Option<PathBuf> = match args.outputs_folder {
        None => None,
        Some(path) => {
            let current_datetime = format!("{}_{}", chrono::Utc::now().format("%Y_%m_%d_%H_%M_%S"), problem_name);
            let path = Path::new(&path).join(&current_datetime);

            if !path.exists() {
                create_dir_all(&path).expect("Could not create output folder");
            }

            println!("Results will be saved to {current_datetime}\n");

            Some(path)
        }
    };

    let start = Instant::now();
    test_qap_algorithm::<LocalSearch<GreedyLocalSearch, RandomStartingSolution>>(&problem, &best_solution, &output_path, true, args.calculate_similarity);
    let lsg_runtime = start.elapsed();

    let start = Instant::now();
    test_qap_algorithm::<LocalSearch<SteepestLocalSearch, RandomStartingSolution>>(&problem, &best_solution, &output_path, true, args.calculate_similarity);
    let lss_runtime = start.elapsed();

    problem.avg_ls_runtime = max(lss_runtime.as_micros() / 200, lsg_runtime.as_micros() / 200);

    test_qap_algorithm::<RandomWalk<RandomStartingSolution>>(&problem, &best_solution, &output_path, true, args.calculate_similarity);
    test_qap_algorithm::<RandomSearch>(&problem, &best_solution, &output_path, true, args.calculate_similarity);
    test_qap_algorithm::<GreedyConstructionHeuristic>(&problem, &best_solution, &output_path, true, args.calculate_similarity);
}
