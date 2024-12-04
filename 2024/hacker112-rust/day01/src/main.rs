use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let (mut first, mut second) =
            lines
                .flatten()
                .fold((Vec::<i32>::new(), Vec::<i32>::new()), |mut acc, line| {
                    let numbers: Vec<_> = line
                        .split_whitespace()
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();

                    acc.0.push(numbers[0]);
                    acc.1.push(numbers[1]);

                    return acc;
                });
        first.sort();
        second.sort();

        let sum: u32 = first
            .iter()
            .zip(second.iter())
            .map(|x| {
                let distance = x.0.abs_diff(*x.1);
                distance
            })
            .sum();

        println!("part 1: {}", sum);
    }
}
