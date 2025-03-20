use qap_utils::problem::QapProblem;

pub struct Move {
    pub p1: usize,
    pub p2: usize,
}

pub struct LocalSearchNeighbourhood {}

impl LocalSearchNeighbourhood {
    pub fn evaluate_move(problem: &QapProblem, mov: &Move, current_solution: &Vec<usize>) -> i32 {
        if mov.p1 == mov.p2 {
            return 0;
        }

        let n = current_solution.len();
        let mut delta_cost: i32 = 0;

        let fa = current_solution[mov.p1];
        let fb = current_solution[mov.p2];

        for k in 0..n {
            if k != mov.p1 && k != mov.p2 {
                let fk = current_solution[k]; // Facility at location k

                delta_cost += (problem.flow_matrix.get(fa, fk) as i32) * (problem.distance_matrix.get(mov.p2, k) as i32 - problem.distance_matrix.get(mov.p1, k) as i32);
                delta_cost += (problem.flow_matrix.get(fb, fk) as i32) * (problem.distance_matrix.get(mov.p1, k) as i32 - problem.distance_matrix.get(mov.p2, k) as i32);

                delta_cost += (problem.flow_matrix.get(fk, fa) as i32) * (problem.distance_matrix.get(k, mov.p2) as i32 - problem.distance_matrix.get(k, mov.p1) as i32);
                delta_cost += (problem.flow_matrix.get(fk, fb) as i32) * (problem.distance_matrix.get(k, mov.p1) as i32 - problem.distance_matrix.get(k, mov.p2) as i32);
            }
        }

        // Direct swap impact
        delta_cost += (problem.flow_matrix.get(fa, fb) as i32) * (problem.distance_matrix.get(mov.p2, mov.p1) as i32 - problem.distance_matrix.get(mov.p1, mov.p2) as i32);
        delta_cost += (problem.flow_matrix.get(fb, fa) as i32) * (problem.distance_matrix.get(mov.p1, mov.p2) as i32 - problem.distance_matrix.get(mov.p2, mov.p1) as i32);

        delta_cost
    }

    pub fn apply_move(mov: &Move, current_solution: &mut Vec<usize>) {
        let temp = current_solution[mov.p1];
        current_solution[mov.p1] = current_solution[mov.p2];
        current_solution[mov.p2] = temp;
    }
}