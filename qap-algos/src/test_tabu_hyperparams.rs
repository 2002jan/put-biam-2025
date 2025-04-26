use std::collections::LinkedList;
use std::io::{stdout, Write};
use qap_utils::problem::{BestSolution, QapProblem};
use qap_utils::Solution;
use crate::local_search::local_search::LocalSearch;
use crate::local_search::neighbourhoods::Move;
use crate::local_search::search_types::tabu_search::tabu_list::memory::long_term::LongTermMemory;
use crate::local_search::search_types::tabu_search::tabu_list::memory::short_term::ShortTermMemory;
use crate::local_search::search_types::tabu_search::tabu_list::move_list::MoveTabuList;
use crate::local_search::search_types::tabu_search::tabu_list::solution_list::SolutionTabuList;
use crate::local_search::search_types::tabu_search::TabuSearch;
use crate::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use crate::test_algorithm::test_qap_algorithm;
use crate::test_tabu_hyperparams::Algos::{LTM, LTS, STM, STS};

#[derive(Clone)]
enum Algos {
    LTM,
    LTS,
    STM,
    STS,
}

impl Algos {
    fn test<const NO_IMPROVEMENT_COUNT: u32>(&self, problem: &QapProblem, optimum: &BestSolution) -> (i32, i32, i32) {
        match self {
            Algos::LTM => {
                test_qap_algorithm::<LocalSearch<TabuSearch<MoveTabuList<LongTermMemory<Move>>, NO_IMPROVEMENT_COUNT>, RandomStartingSolution>>(problem, optimum, &None, false, false)
            }
            Algos::LTS => {
                test_qap_algorithm::<LocalSearch<TabuSearch<SolutionTabuList<LongTermMemory<Solution>>, NO_IMPROVEMENT_COUNT>, RandomStartingSolution>>(problem, optimum, &None, false, false)
            }
            Algos::STM => {
                test_qap_algorithm::<LocalSearch<TabuSearch<MoveTabuList<ShortTermMemory<Move>>, NO_IMPROVEMENT_COUNT>, RandomStartingSolution>>(problem, optimum, &None, false, false)
            }
            Algos::STS => {
                test_qap_algorithm::<LocalSearch<TabuSearch<SolutionTabuList<ShortTermMemory<Solution>>, NO_IMPROVEMENT_COUNT>, RandomStartingSolution>>(problem, optimum, &None, false, false)
            }
        }
    }

    fn name(&self) -> String {
        match self {
            LTM => String::from("Long term move"),
            LTS => String::from("Long term solution"),
            STM => String::from("Short term move"),
            STS => String::from("Short term solution")
        }
    }
}

pub fn test_tabu_hyperparams(problem: &QapProblem, optimum: &BestSolution) {
    static ALGOS: [Algos; 4] = [LTM, LTS, STM, STS];

    let mut scores: LinkedList<(Algos, u32, i32, i32, i32)> = LinkedList::new();

    for algo in &ALGOS {
        println!("{}", algo.name());

        print!("{}", 10);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<10>(problem, optimum);
        scores.push_back((algo.clone(), 10, min, max, avg));

        print!("\r{}", 20);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<20>(problem, optimum);
        scores.push_back((algo.clone(), 20, min, max, avg));

        print!("\r{}", 50);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<50>(problem, optimum);
        scores.push_back((algo.clone(), 50, min, max, avg));

        print!("\r{}", 100);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<100>(problem, optimum);
        scores.push_back((algo.clone(), 100, min, max, avg));

        print!("\r{}", 150);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<150>(problem, optimum);
        scores.push_back((algo.clone(), 150, min, max, avg));

        print!("\r{}", 200);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<200>(problem, optimum);
        scores.push_back((algo.clone(), 200, min, max, avg));

        print!("\r{}", 300);
        stdout().flush().unwrap();
        let (min, max, avg) = algo.test::<300>(problem, optimum);
        scores.push_back((algo.clone(), 300, min, max, avg));
        println!();
    }

    print!("\n\nScores!\n");

    for (a, n, min_cost, max_cost, aggregated_cost) in scores {
        println!("Algo: {}", a.name());
        println!("No improvement value: {}", n);
        println!("Min cost: {}({})\nMax cost: {}({})\nAverage cost: {}({})\n", min_cost, min_cost as f32 / optimum.best_score as f32, max_cost, max_cost as f32 / optimum.best_score as f32, aggregated_cost, aggregated_cost as f32 / optimum.best_score as f32);
    }
}