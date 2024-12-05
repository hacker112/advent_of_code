use shared::read_lines;
// use std::{collections::HashMap, ops};

#[derive(Debug, PartialEq)]
struct PageOrder {
    page: u8,
    before_page: u8,
}

#[derive(Debug, PartialEq)]
struct PageUpdate {
    pages: Vec<u8>,
}

// #[derive(Debug, PartialEq, Eq, Hash)]
// struct Coord {
//     x: i32,
//     y: i32,
// }

// impl ops::Add<&Coord> for &Coord {
//     type Output = Coord;

//     fn add(self, _rhs: &Coord) -> Coord {
//         let x = self.x + _rhs.x;
//         let y = self.y + _rhs.y;
//         Coord { x, y }
//     }
// }

// impl ops::Add<&Coord> for Coord {
//     type Output = Coord;

//     fn add(self, _rhs: &Coord) -> Coord {
//         (&self).add(_rhs)
//     }
// }

// impl Coord {
//     fn try_get_matrix_value(&self, matrix: &Vec<Vec<u8>>) -> Option<u8> {
//         if self.x < 0 || self.y < 0 {
//             return None;
//         }

//         let x = self.x as usize;
//         let y = self.y as usize;

//         if let Some(row) = matrix.get(x) {
//             if let Some(val) = row.get(y) {
//                 // println!("val ({},{}){}", x, y, val);
//                 return Some(val.clone());
//             }
//         }

//         None
//     }
// }

fn main() {
    let filename = "./input";
    let (page_orders, pages_updates) = read_pages(filename);

    println!("A: {}, {}", page_orders.len(), pages_updates.len());
    // let total_found = part1(&matrix);
    // println!("part 1: {}", total_found);

    // let found = part2(&matrix);
    // println!("part 2: {}", found);
}

// fn part1(matrix: &Vec<Vec<u8>>) -> u32 {
//     let needle: Vec<_> = "XMAS".as_bytes().to_owned();

//     let directions: Vec<Coord> = (-1..=1)
//         .flat_map(|x| (-1..=1).map(move |y| Coord { x, y }))
//         .filter(|c| !(c.x == 0 && c.y == 0))
//         .collect();

//     let total_found: u32 = directions
//         .into_iter()
//         .map(|dir| {
//             let found: u32 = (0..matrix.len() as i32)
//                 .flat_map(|x| (0..matrix[x as usize].len() as i32).map(move |y| Coord { x, y }))
//                 .map(|start| rec_find_needle(&start, &dir, &needle, &matrix))
//                 .sum();

//             found
//         })
//         .sum();
//     total_found
// }

fn read_pages(filename: &str) -> (Vec<PageOrder>, Vec<PageUpdate>) {
    let lines = read_lines(filename);

    let (page_orders, pages_updates) = lines.fold(
        (Vec::<PageOrder>::new(), Vec::<PageUpdate>::new()),
        |(mut page_orders, mut pages_updates), line| {
            if line.contains('|') {
                let numbers = line
                    .clone()
                    .split('|')
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                page_orders.push(PageOrder {
                    page: numbers[0],
                    before_page: numbers[1],
                });
            }
            if line.contains(',') {
                let numbers = line
                    .clone()
                    .split(',')
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                pages_updates.push(PageUpdate { pages: numbers });
            }

            (page_orders, pages_updates)
        },
    );

    (page_orders, pages_updates)
}

// fn rec_find_needle(start: &Coord, dir: &Coord, needle: &Vec<u8>, matrix: &Vec<Vec<u8>>) -> u32 {
//     if let Some(char) = needle.first() {
//         if let Some(matrix_value) = start.try_get_matrix_value(matrix) {
//             let is_match = matrix_value == *char;
//             if is_match {
//                 if needle.len() == 1 {
//                     return 1;
//                 }
//                 let next = start + dir;
//                 let next_needle: Vec<u8> = needle[1..].to_owned();
//                 return rec_find_needle(&next, dir, &next_needle, matrix);
//             }
//         }
//     }

//     0
// }

// fn part2(matrix: &Vec<Vec<u8>>) -> i32 {
//     let needle: Vec<_> = "MAS".as_bytes().to_owned();

//     let directions: Vec<Coord> = (-1..=1)
//         .flat_map(|x| (-1..=1).map(move |y| Coord { x, y }))
//         .filter(|c| !(c.x == 0 || c.y == 0))
//         .collect();

//     let coords: Vec<_> = (0..matrix.len() as i32)
//         .flat_map(|x| (0..matrix[x as usize].len() as i32).map(move |y| Coord { x, y }))
//         .collect();

//     let found = directions
//         .iter()
//         .flat_map(|dir| {
//             coords.iter().filter_map(|start| {
//                 let found = rec_find_needle(&start, dir, &needle, &matrix);

//                 if found > 0 {
//                     let a_coord = start + dir;
//                     let sign: i32 = dir.x * dir.y;
//                     // println!("A={:?}, {:?}, sign= {}", a_coord, dir, sign);
//                     return Some((a_coord, sign));
//                 }

//                 None
//             })
//         })
//         .fold(HashMap::<Coord, i32>::new(), |mut acc, next| {
//             let coord = next.0;
//             let sign = next.1;

//             let stored_sign = acc.entry(coord).or_insert(0);
//             *stored_sign += sign;

//             acc
//         })
//         .iter()
//         .fold(0, |sum, next| {
//             if *next.1 == 0 {
//                 return sum + 1;
//             }
//             sum
//         });
//     found
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let (page_orders, pages_updates) = read_pages("./input_example");
        assert_eq!(page_orders.len(), 21);
        assert_eq!(
            page_orders[0],
            PageOrder {
                page: 47,
                before_page: 53
            }
        );
        assert_eq!(pages_updates.len(), 6);
    }

    // #[test]
    // fn test_part_1() {
    //     let matrix = read_matrix("./input_example");
    //     let sum = part1(&matrix);
    //     assert_eq!(sum, 18);
    // }

    // #[test]
    // fn test_part_2() {
    //     let matrix = read_matrix("./input_example");
    //     let sum = part2(&matrix);
    //     assert_eq!(sum, 9);
    // }
}
