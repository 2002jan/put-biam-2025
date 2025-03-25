use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use crate::cost_matrix::CostMatrix;
use crate::problem::{BestSolution, QapProblem};

pub fn load_from_file(path: &String) -> QapProblem {
    let path = Path::new(path);

    if !path.exists() || !path.is_file() || !path.extension().unwrap().eq("dat") {
        panic!("Invalid problem path: {}", path.display())
    }

    let file = File::open(path).expect("Error while opening file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let problem_size = lines.next()
        .unwrap().unwrap().trim().parse::<usize>()
        .expect("Could not parse problem size");

    // println!("Problem size: {problem_size}");

    let mut distance_matrix = CostMatrix::new(problem_size);
    fill_matrix(&mut lines, problem_size, &mut distance_matrix);

    let mut flow_matrix = CostMatrix::new(problem_size);
    fill_matrix(&mut lines, problem_size, &mut flow_matrix);

    QapProblem {
        distance_matrix,
        flow_matrix,
        size: problem_size,
    }
}

fn fill_matrix(lines: &mut Lines<BufReader<File>>, problem_size: usize, output: &mut CostMatrix) {
    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(problem_size);
    let mut numbers: Vec<usize> = Vec::with_capacity(problem_size);

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.trim().len() == 0 {
            continue;
        }

        let mut number: Option<usize> = None;


        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as usize;

                if let Some(num) = number {
                    number = Some(num * 10 + digit)
                } else {
                    number = Some(digit)
                }
            } else {
                if let Some(num) = number {
                    numbers.push(num);
                    number = None;
                }
            }
        }

        if let Some(num) = number {
            numbers.push(num);
        }

        if numbers.len() == problem_size {
            matrix.push(numbers);
            numbers = Vec::with_capacity(problem_size);

            if matrix.len() == problem_size {
                break;
            }
        }
    }

    assert_eq!(matrix.len(), problem_size);

    for i in 0..problem_size {
        for j in 0..problem_size {
            output.set(j, i, matrix[i][j])
        }
    }
}

pub fn load_optimal_solution(solution_path: Option<PathBuf>, problem_path: &Path, problem: &QapProblem) -> BestSolution {
    let solution_path = match solution_path {
        None => {
            println!("Solution file path not provided, deducting from problem_path: {:?}", problem_path);
            let path = problem_path.with_extension("sln");


            println!("Deducted solution file: {:?}", path);

            path
        }
        Some(path) => path.to_path_buf()
    };

    if !solution_path.exists() || !solution_path.is_file() || !solution_path.extension().unwrap().eq("sln") {
        panic!("Deducted solution file does not exists: {:?}", solution_path)
    }

    let file = File::open(solution_path).expect("Error while opening file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut numbers: LinkedList<u32> = LinkedList::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.trim().len() == 0 {
            continue;
        }

        let mut number: Option<u32> = None;

        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();

                if let Some(num) = number {
                    number = Some(num * 10 + digit)
                } else {
                    number = Some(digit)
                }
            } else {
                if let Some(num) = number {
                    numbers.push_back(num);
                    number = None;
                }
            }
        }

        if let Some(num) = number {
            numbers.push_back(num);
        }
    }

    let problem_size = numbers.pop_front().expect("Problem size not found in solution") as usize;
    let best_score = numbers.pop_front().expect("Score not found in solution") as i32;

    assert_eq!(problem_size, numbers.len());
    assert_eq!(problem.size, numbers.len());

    BestSolution {
        size: problem_size,
        best_score,
        solution: numbers.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
    }
}