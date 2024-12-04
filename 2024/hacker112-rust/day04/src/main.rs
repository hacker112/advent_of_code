use std::{
    fs::File,
    io::{self, BufRead},
    ops,
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl ops::Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, _rhs: &Coord) -> Coord {
        let x = self.x + _rhs.x;
        let y = self.y + _rhs.y;
        Coord { x, y }
    }
}

impl Coord {
    fn try_get_matrix_value(&self, matrix: &Vec<Vec<u8>>) -> Option<u8> {
        if self.x < 0 || self.y < 0 {
            return None;
        }

        let x = self.x as usize;
        let y = self.y as usize;

        if let Some(row) = matrix.get(x) {
            if let Some(val) = row.get(y) {
                // println!("val ({},{}){}", x, y, val);
                return Some(val.clone());
            }
        }

        None
    }
}

fn main() {
    let needle: Vec<_> = "XMAS".as_bytes().to_owned();

    let directions: Vec<Coord> = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| Coord { x, y }))
        .filter(|c| !(c.x == 0 && c.y == 0))
        .collect();

    if let Ok(lines) = read_lines("./input") {
        let matrix: Vec<Vec<u8>> = lines
            .flatten()
            .map(|line| line.as_bytes().to_owned())
            .collect();

        let total_found: u32 = directions
            .into_iter()
            .map(|dir| {
                let found: u32 = (0..matrix.len() as i32)
                    .flat_map(|x| (0..matrix[x as usize].len() as i32).map(move |y| Coord { x, y }))
                    .map(|start| rec_find_needle(&start, &dir, &needle, &matrix))
                    .sum();

                found
            })
            .sum();
        println!("part 1: {}", total_found);
    }
}

fn rec_find_needle(start: &Coord, dir: &Coord, needle: &Vec<u8>, matrix: &Vec<Vec<u8>>) -> u32 {
    if let Some(char) = needle.first() {
        if let Some(matrix_value) = start.try_get_matrix_value(matrix) {
            let is_match = matrix_value == *char;
            if is_match {
                if needle.len() == 1 {
                    return 1;
                }
                let next = start + dir;
                let next_needle: Vec<u8> = needle[1..].to_owned();
                return rec_find_needle(&next, dir, &next_needle, matrix);
            }
        }
    }

    0
}
