use std::time::Instant;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::TspAlgorithm;

const RUNS: i32 = 200;

pub fn test_qap_algorithm<Algorithm: TspAlgorithm>(problem: &QapProblem, verbose: bool) -> (i32, i32, i32) {
    let mut min_cost = i32::MAX;
    let mut min_solution = vec![0];
    let mut max_cost = 0;
    let mut _max_solution = vec![0];
    let mut aggregated_cost = 0;

    let start = Instant::now();

    for _ in 0..RUNS {
        let solution = Algorithm::run(problem);
        let cost = evaluate_solution(&solution, problem);

        if min_cost > cost {
            min_cost = cost;
            min_solution = solution.clone();
        }

        if max_cost < cost {
            max_cost = cost;
            _max_solution = solution.clone();
        }

        aggregated_cost += cost;
    }

    let duration = start.elapsed();

    let aggregated_cost = aggregated_cost as f32 / RUNS as f32;
    let aggregated_cost = aggregated_cost.round() as i32;

    if verbose {
        println!("Results for {}\nMin cost: {}\nMax cost: {}\nAverage cost: {}\n", Algorithm::name(), min_cost, max_cost, aggregated_cost);

        let duration_micros = duration.as_micros();
        let duration_per_run = duration_micros / (RUNS as u128);

        println!("Time took for {} runs: {:.8}s, time per run: {}μs\n", RUNS, duration.as_secs_f64(), duration_per_run);
        println!("Best solution:\n{:?}\n", min_solution);
    }

    (min_cost, max_cost, aggregated_cost)
}