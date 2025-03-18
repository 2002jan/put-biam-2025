use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use crate::cost_matrix::CostMatrix;

pub fn load_from_file(path: &String) {
    let path = Path::new(path);

    if !path.exists() || !path.is_file() {
        panic!("Invalid problem path: {}", path.display())
    }

    let file = File::open(path).expect("Error while opening file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let problem_size = lines.next()
        .unwrap().unwrap().trim().parse::<usize>()
        .expect("Could not parse problem size");

    // println!("Problem size: {problem_size}");

    let mut matrix1: Vec<Vec<usize>> = Vec::with_capacity(problem_size);
    fill_matrix(&mut lines, problem_size, &mut matrix1);
    assert_eq!(matrix1.len(), problem_size);
    assert_eq!(matrix1.iter().map(|x| x.len()).sum::<usize>(), problem_size * problem_size);

    let mut matrix2: Vec<Vec<usize>> = Vec::with_capacity(problem_size);
    fill_matrix(&mut lines, problem_size, &mut matrix2);
    assert_eq!(matrix2.len(), problem_size);
    assert_eq!(matrix2.iter().map(|x| x.len()).sum::<usize>(), problem_size * problem_size);

    let mut cost_matrix = CostMatrix::new(problem_size);

    for i in 0..problem_size {
        for j in 0..problem_size {
            cost_matrix.set(i, j, matrix1[i][j] * matrix2[i][j])
        }
    }
}

fn fill_matrix(lines: &mut Lines<BufReader<File>>, problem_size: usize, output: &mut Vec<Vec<usize>>) {
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
            output.push(numbers);
            numbers = Vec::with_capacity(problem_size);

            if output.len() == problem_size {
                break;
            }
        }
    }

    assert_eq!(output.len(), problem_size)
}