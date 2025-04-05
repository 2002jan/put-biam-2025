use std::collections::LinkedList;
use crate::evaluate_solution::evaluate_solution;
use crate::problem::{BestSolution, QapProblem};

pub fn check_similarity_best(
    solutions: &LinkedList<Vec<usize>>,
    best_solution: &BestSolution,
    problem: &QapProblem,
) -> LinkedList<(i32, i32)> {
    solutions.iter().map(|solution| (
        evaluate_solution(solution, problem),
        evaluate_similarity(solution, &best_solution.solution)
    )).collect()
}

pub fn check_similarity_avg(
    solutions: &LinkedList<Vec<usize>>,
    problem: &QapProblem,
) -> LinkedList<(i32, f32)> {
    let mut pairs: LinkedList<(i32, f32)> = LinkedList::new();

    for (i, s1) in solutions.iter().enumerate() {
        let mut similarity: f32 = 0.0;
        let mut k = 0;
        for (j, s2) in solutions.iter().enumerate() {
            if i == j {
                continue;
            }

            similarity += evaluate_similarity(s1, s2) as f32;
            k += 1;
        }

        similarity /= k as f32;
        pairs.push_back((evaluate_solution(s1, problem), similarity));
    }

    pairs
}


fn evaluate_similarity(solution1: &Vec<usize>, solution2: &Vec<usize>) -> i32 {
    (0..solution1.len()).filter(|&i| solution1[i] == solution2[i]).count() as i32
}
