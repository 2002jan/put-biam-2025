use std::cmp::Reverse;
use std::collections::LinkedList;
use qap_utils::algorithm_stats_recorder::AlgorithmRunStatsRecorder;
use qap_utils::evaluate_solution::evaluate_solution;
use qap_utils::problem::QapProblem;
use crate::TspAlgorithm;

pub struct GreedyConstructionHeuristic {}

impl GreedyConstructionHeuristic {
    fn calculate_incremental_cost(facility: usize, location: usize, assigned: &LinkedList<(usize, usize)>, problem: &QapProblem) -> i32 {
        assigned.iter().map(|&(f, l)| {
            (problem.flow_matrix.get(facility, f) * problem.distance_matrix.get(location, l)) as i32
                + (problem.flow_matrix.get(f, facility) * problem.distance_matrix.get(l, location)) as i32
        }).sum()
    }
}

impl TspAlgorithm for GreedyConstructionHeuristic {
    fn run(problem: &QapProblem, mut recorder: Option<&mut AlgorithmRunStatsRecorder>) -> Vec<usize> {
        let mut unassigned_facilities = (0..problem.size).collect::<Vec<usize>>();
        let mut unassigned_locations = (0..problem.size).collect::<Vec<usize>>();
        let mut assigned: LinkedList<(usize, usize)> = LinkedList::new();

        unassigned_facilities.sort_by_cached_key(|&x| {
            let mut flow_sum = 0;
            for y in 0..problem.size {
                flow_sum += problem.flow_matrix.get(x, y)
            }

            flow_sum
        });

        while !unassigned_facilities.is_empty() {
            let facility = unassigned_facilities.pop().unwrap();

            unassigned_locations.sort_by_cached_key(|&x| Reverse(Self::calculate_incremental_cost(facility, x, &assigned, problem)));

            let location = unassigned_locations.pop().unwrap();

            assigned.push_back((facility, location))
        }

        let mut solution = vec![0; problem.size];

        while let Some((f, l)) = assigned.pop_back() {
            solution[l] = f;
        }

        if let Some(rec) = &mut recorder {
            let current_score = evaluate_solution(&solution, problem);
            rec.record_iteration(current_score);
            rec.record_evaluation();
        }

        solution
    }

    fn name() -> String {
        String::from("Greedy construction heuristic")
    }

    fn snaked_name() -> String {
        String::from("greedy_construction_heuristic")
    }
}