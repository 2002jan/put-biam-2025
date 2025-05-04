use std::collections::LinkedList;
use serde::Serialize;

#[derive(Serialize)]
pub struct AlgorithmStatsRecorder {
    runs: LinkedList<AlgorithmRunStatsRecorder>,
    best_run: Option<AlgorithmRunStatsRecorder>,
    worst_run: Option<AlgorithmRunStatsRecorder>,
    optimum: i32,
    pub avg_runtime: Option<u128>,
    pub similarities_best: Option<LinkedList<(i32, i32)>>,
    pub similarities_avg: Option<LinkedList<(i32, f32)>>
}

#[derive(Serialize, Clone)]
pub struct AlgorithmRunStatsRecorder {
    iterations: i32,
    scores: LinkedList<i32>,
    evaluations: i32,
    partial_evaluations: i32,
    final_score: i32,
    pub algorithm_run_time: u128
}

impl AlgorithmStatsRecorder {
    pub fn new(optimum: i32) -> Self {
        Self {
            runs: LinkedList::new(),
            best_run: None,
            worst_run: None,
            optimum,
            avg_runtime: None,
            similarities_best: None,
            similarities_avg: None,
        }
    }

    pub fn add_run(&mut self, run_recorder: AlgorithmRunStatsRecorder) {
        if let Some(run) = self.best_run.as_ref() {
            if run.final_score > run_recorder.final_score {
                self.best_run = Some(run_recorder.clone())
            }
        } else {
            self.best_run = Some(run_recorder.clone())
        }

        if let Some(run) = self.worst_run.as_ref() {
            if run.final_score < run_recorder.final_score {
                self.worst_run = Some(run_recorder.clone())
            }
        } else {
            self.worst_run = Some(run_recorder.clone())
        }

        self.runs.push_back(run_recorder)
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(self).expect("Could not convert AlgorithmStatsRecorder to json")
    }
}

impl AlgorithmRunStatsRecorder {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            scores: LinkedList::new(),
            evaluations: 0,
            partial_evaluations: 0,
            final_score: i32::MAX,
            algorithm_run_time: 0,
        }
    }

    pub fn record_iteration(&mut self, score: i32) {
        self.iterations += 1;
        self.scores.push_back(score);

        if self.final_score > score {
            self.final_score = score
        }
    }

    pub fn record_evaluation(&mut self) {
        self.evaluations += 1;
    }

    pub fn record_partial_evaluation(&mut self) {
        self.evaluations += 1;
        self.partial_evaluations += 1;
    }
}