use shared::read_lines;

fn main() {
    let filename = "./input";
    let _matrix = read_matrix(filename);
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

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

fn read_char(c: char) -> MapItem {
    match c {
        '.' => MapItem::Empty,
        '#' => MapItem::Obstacle,
        '^' => MapItem::Guard(Direction::Up),
        '<' => MapItem::Guard(Direction::Left),
        '>' => MapItem::Guard(Direction::Right),
        'v' => MapItem::Guard(Direction::Down),
        _ => panic!("Unkown map item for char={}", c),
    }
}

fn read_matrix(filename: &str) -> Vec<Vec<MapItem>> {
    let lines = read_lines(filename);
    let matrix = lines
        .map(|line| line.chars().map(|c| read_char(c)).collect::<Vec<MapItem>>())
        .collect::<Vec<Vec<MapItem>>>();
    matrix
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
    fn test_read_char() {
        assert_eq!(read_char('.'), MapItem::Empty);
        assert_eq!(read_char('#'), MapItem::Obstacle);
        assert_eq!(read_char('^'), MapItem::Guard(Direction::Up));
        assert_eq!(read_char('<'), MapItem::Guard(Direction::Left));
        assert_eq!(read_char('>'), MapItem::Guard(Direction::Right));
        assert_eq!(read_char('v'), MapItem::Guard(Direction::Down));
    }

    #[test]
    fn test_read() {
        let matrix = read_matrix("./input_example");
        assert_eq!(matrix[0][0], MapItem::Empty);
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
