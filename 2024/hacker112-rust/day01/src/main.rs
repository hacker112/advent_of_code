use shared::read_lines;

fn main() {
    let lines = read_lines("./input");

    let (mut first, mut second) =
        lines.fold((Vec::<u32>::new(), Vec::<u32>::new()), |mut acc, line| {
            let numbers: Vec<_> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
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

    let similarities = similiarities_sum(&first, &second);
    println!("part 2: {}", similarities);
}

fn similiarities_sum(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let similarities: u32 = a.iter().map(|n| similarity_score(n, b)).sum();
    similarities
}

fn similarity_score(needle: &u32, haystack: &Vec<u32>) -> u32 {
    let t = haystack
        .iter()
        .filter(|h| {
            let a = **h;
            let b = *needle;

            a == b
        })
        .count();

    t as u32 * needle
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_1_example() {
//         let lines = read_lines("./test_lines.txt");
//         let all_lines: Vec<String> = lines.collect();
//         assert_eq!(all_lines, vec!["A", "B", "C"]);
//     }
// }
