use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use qap_utils::algorithm_stats_recorder::{AlgorithmRunStatsRecorder, AlgorithmStatsRecorder};
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::{BestSolution, QapProblem};
use crate::TspAlgorithm;

const RUNS: i32 = 200;

pub fn test_qap_algorithm<Algorithm: TspAlgorithm>(problem: &QapProblem, optimum: &BestSolution, output_path: &Option<PathBuf>, verbose: bool) -> (i32, i32, i32) {
    let mut min_cost = i32::MAX;
    let mut min_solution = vec![0];
    let mut max_cost = 0;
    let mut _max_solution = vec![0];
    let mut aggregated_cost = 0;

    let start = Instant::now();

    let mut recorder: Option<AlgorithmStatsRecorder> = if output_path.is_some() {
        Some(AlgorithmStatsRecorder::new(optimum.best_score))
    } else {
        None
    };


    for _ in 0..RUNS {
        let solution = if let Some(recorder) = &mut recorder {
            let mut run_recorder = AlgorithmRunStatsRecorder::new();
            let solution = Algorithm::run(problem, Some(&mut run_recorder));
            recorder.add_run(run_recorder);
            solution
        } else {

            Algorithm::run(problem, None)
        };


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

    let duration_micros = duration.as_micros();
    let duration_per_run = duration_micros / RUNS as u128;

    if verbose {
        println!("Results for {}\nMin cost: {}\nMax cost: {}\nAverage cost: {}\n", Algorithm::name(), min_cost, max_cost, aggregated_cost);

        println!("Time took for {} runs: {:.8}s, time per run: {}μs\n", RUNS, duration.as_secs_f64(), duration_per_run);
        println!("Best solution:\n{:?}\n", min_solution);
    }

    if let (Some(path), Some(mut recorder)) = (output_path, recorder) {
        recorder.avg_runtime = Some(duration_per_run);

        let output_path = path.join(format!("{}_output.json", Algorithm::snaked_name()));
        fs::write(output_path, recorder.to_json()).expect("Could not save output to json");
    }

    (min_cost, max_cost, aggregated_cost)
}