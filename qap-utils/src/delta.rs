use crate::cost_matrix::CostMatrix;

pub fn delta_evaluate_swap(flow_matrix: &CostMatrix, distance_matrix: &CostMatrix, assignment: &Vec<usize>, a: usize, b: usize,
) -> i32 {
    if a == b {
        return 0;
    }

    let n = assignment.len();
    let mut delta_cost: i32 = 0;

    let fa = assignment[a];
    let fb = assignment[b];

    for k in 0..n {
        if k != a && k != b {
            let fk = assignment[k]; // Facility at location k

            delta_cost += (flow_matrix.get(fa, fk) as i32) * (distance_matrix.get(b, k) as i32 - distance_matrix.get(a, k) as i32);
            delta_cost += (flow_matrix.get(fb, fk) as i32) * (distance_matrix.get(a, k) as i32 - distance_matrix.get(b, k) as i32);

            delta_cost += (flow_matrix.get(fk, fa) as i32) * (distance_matrix.get(k, b) as i32 - distance_matrix.get(k, a) as i32);
            delta_cost += (flow_matrix.get(fk, fb) as i32) * (distance_matrix.get(k, a) as i32 - distance_matrix.get(k, b) as i32);
        }
    }

    // Direct swap impact
    delta_cost += (flow_matrix.get(fa, fb) as i32) * (distance_matrix.get(b, a) as i32 - distance_matrix.get(a, b) as i32);
    delta_cost += (flow_matrix.get(fb, fa) as i32) * (distance_matrix.get(a, b) as i32 - distance_matrix.get(b, a) as i32);

    delta_cost
}
