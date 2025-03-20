use crate::cost_matrix::CostMatrix;

pub struct QapProblem {
    pub distance_matrix: CostMatrix,
    pub flow_matrix: CostMatrix,
    pub size: usize
}