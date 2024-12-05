use std::{fs, iter::zip};

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}

fn parse(inp: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_col = vec![];
    let mut right_col = vec![];
    for line in inp.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        let first_token = tokens.next();
        let second_token = tokens.next();
        if let Some(token) = first_token {
            if let Ok(num) = token.trim().parse::<i32>() {
                left_col.push(num)
            } else {
                println!("Failed to parse '{token}', pushing 0!");
                left_col.push(0);
            };
        }
        if let Some(token) = second_token {
            if let Ok(num) = token.trim().parse::<i32>() {
                right_col.push(num)
            } else {
                println!("Failed to parse '{token}', pushing 0!");
                right_col.push(0);
            };
        }
    }
    (left_col, right_col)
}

fn eval_sum(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut result = 0;
    for (val1, val2) in zip(v1, v2) {
        result += (val1 - val2).abs();
    }
    result
}

fn eval_count(v1: &[i32], v2: &[i32]) -> i32 {
    let mut result = 0;
    for val in v1 {
        result += (v2.iter().filter(|x| **x == *val).count() as i32) * val;
    }
    result
}

fn solve_1(path: &str) {
    print!("Solving problem 1: ");
    let contents = read_file(path);
    let (mut v1, mut v2) = parse(&contents);
    v1.sort_unstable();
    v2.sort_unstable();
    let result = eval_sum(v1, v2);
    println!("{result}");
}

fn solve_2(path: &str) {
    print!("Solving problem 2: ");
    let contents = read_file(path);
    let (v1, v2) = parse(&contents);
    let result = eval_count(&v1, &v2);
    println!("{result}");
}

fn main() {
    solve_1("day202401.txt");
    solve_2("day202401.txt");
}
