use crate::problem::QapProblem;

pub fn evaluate_solution(solution: &Vec<usize>, problem: &QapProblem) -> i32 {
    let mut total_cost: usize = 0;

    for i in 0..problem.problem_size {
        let f1 = solution[i];
        for j in 0..problem.problem_size {
            let f2 = solution[j];

            total_cost += problem.flow_matrix.get(f1, f2) * problem.distance_matrix.get(i, j);
        }
    }

    total_cost as i32
}