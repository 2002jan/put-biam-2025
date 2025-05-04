use rand::{rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::local_search::neighbourhoods::{LocalSearchNeighbourhood, Move};
use crate::local_search::search_types::LocalSearchType;

pub struct SimulatedAnnealing {
    rng: ThreadRng,
    start_order: Vec<usize>,
    current_start: usize,
    next_start: usize,
    next_target: usize,
}


impl LocalSearchType for SimulatedAnnealing {
    fn new(problem: &QapProblem) -> Self {
        let thread_rng = rng();
        let size = problem.size;

        SimulatedAnnealing {
            rng: thread_rng,
            start_order: (0..size).collect(),
            current_start: 0,
            next_start: 0,
            next_target: 0,
        }
    }

    fn run(problem: &QapProblem, starting_solution: Vec<usize>, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut rng = rng();
        let size = problem.size;

        let mut current_solution = starting_solution;
        let mut current_score = evaluate_solution(&current_solution, problem);

        let mut best_solution = current_solution.clone();
        let mut best_score = current_score;


        if let Some(rec) = &mut recorder {
            rec.record_iteration(current_score);
            rec.record_evaluation();
        }

        // Estimate starting temperature (95% acceptance of bad moves)
        let mut deltas: Vec<f64> = Vec::new();
        let samples = 100.min(size * size);

        for _ in 0..samples {
            let i = rng.random_range(0..size);
            let j = rng.random_range(0..size);

            if i != j {
                let mv = Move { p1: i, p2: j };
                let delta = LocalSearchNeighbourhood::evaluate_move(problem, &mv, &current_solution) as f64;
                if delta > 0.0 {
                    deltas.push(delta);
                }
            }
        }

        let avg_bad_delta = if deltas.is_empty()
        { 1.0 } else {
            deltas.iter().sum::<f64>() / deltas.len() as f64
        };

        let mut temperature = -avg_bad_delta / (0.95f64.ln());  // set temperature to allow 95% acceptance of bad moves (from the formula T = -delta / ln(0.95))

        let alpha = 0.90;
        let markov_length = size * size;
        let p = 10;
        let mut iterations_since_improvement = 0;

        while iterations_since_improvement < p * markov_length && temperature > 1e-3 {
            for _ in 0..markov_length {
                let i = rng.random_range(0..size);
                let j = rng.random_range(0..size);
                if i == j {
                    continue;
                }

                let mv = Move { p1: i, p2: j };
                let delta = LocalSearchNeighbourhood::evaluate_move(problem, &mv, &current_solution);

                if let Some(rec) = &mut recorder {
                    rec.record_partial_evaluation();
                }

                if delta < 0 || rng.random_bool((-delta as f64 / temperature).exp()) {
                    LocalSearchNeighbourhood::apply_move(&mv, &mut current_solution);
                    current_score += delta;
                }

                if let Some(rec) = &mut recorder {
                    rec.record_iteration(current_score);
                }

                if current_score < best_score {
                    best_score = current_score;
                    best_solution = current_solution.clone();
                    iterations_since_improvement = 0;
                } else {
                    iterations_since_improvement += 1;
                }


            }

            temperature *= alpha;

        }

        best_solution
    }

    fn reset_iterator(&mut self) {
        self.start_order.shuffle(&mut self.rng);
        self.current_start = 0;
        self.next_start = 0;
        self.next_target = 0;
    }

    fn next(&mut self) -> Option<Move> {
        None
    }

    fn name() -> String {
        String::from("Simulated Annealing")
    }

    fn snaked_name() -> String {
        String::from("simluated_annealing")
    }
}