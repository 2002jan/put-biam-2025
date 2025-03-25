use crate::cost_matrix::CostMatrix;

pub struct QapProblem {
    pub distance_matrix: CostMatrix,
    pub flow_matrix: CostMatrix,
    pub size: usize
}

pub struct BestSolution {
    pub size: usize,
    pub best_score: i32,
    pub solution: Vec<usize>
}
