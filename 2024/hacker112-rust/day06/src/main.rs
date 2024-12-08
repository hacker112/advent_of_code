use shared::read_lines;

fn main() {
    let filename = "./input";
    let map = read_map(filename);
    // let total_found = part1(&map);
    // println!("part 1: {}", total_found);
    let found = part2(&map);
    println!("part 2: {}", found);
}

fn part1(map: &Map) -> usize {
    let next_map = map.step_unil_guard_exits();
    next_map.count_guard_visited()
}
fn part2(map: &Map) -> usize {
    map.step_unil_guard_exits_and_find_infinte_loops()
}

#[derive(Debug, PartialEq)]
struct Map {
    items: Vec<Vec<MapItem>>,
    contains_infinite_loop: bool,
}

impl Map {
    fn get_item(&self, (_row, _col): (i32, i32)) -> Option<MapItem> {
        if let Ok(row) = usize::try_from(_row) {
            if let Ok(col) = usize::try_from(_col) {
                if let Some(row_vector) = self.items.get(row) {
                    if let Some(item) = row_vector.get(col) {
                        return Some(item.to_owned());
                    }
                }
            }
        }
        None
    }

    fn set_item(&mut self, (row, col): (i32, i32), item: MapItem) {
        // TODO Handle errors?
        self.items[row as usize][col as usize] = item;
    }

    fn count_guard_visited(&self) -> usize {
        self.items
            .iter()
            .flatten()
            .filter(|item| match item {
                MapItem::Guard(_) => true,
                MapItem::GuardVisited(_) => true,
                _ => false,
            })
            .count()
    }

    fn copy(&self) -> Map {
        Map {
            items: self.items.to_vec(),
            contains_infinite_loop: self.contains_infinite_loop,
        }
    }

    fn step(&self) -> Option<Map> {
        let mut next_map = self.copy();

        let (coords, guard_direction) = self.find_the_guard();
        let next_coords = guard_direction.step_forward(coords);

        // println!("now={},{}, next=,{:?}", row_index, col_index, next_coords);

        if let Some(next_item) = self.get_item(next_coords) {
            if next_item == MapItem::GuardVisited(guard_direction.to_owned()) {
                next_map.contains_infinite_loop = true;
            } else if next_item == MapItem::Obstacle {
                next_map.set_item(coords, MapItem::Guard(guard_direction.turn_right()));
            } else {
                next_map.set_item(coords, MapItem::GuardVisited(guard_direction.to_owned()));
                next_map.set_item(next_coords, MapItem::Guard(guard_direction));
            }
        } else {
            return None;
        }

        Some(next_map)
    }

    fn find_the_guard(&self) -> ((i32, i32), Direction) {
        let (row_index, col_index, item) = self
            .items
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                let found = row
                    .iter()
                    .enumerate()
                    .find_map(|(col_index, item)| match item {
                        MapItem::Guard(_) => Some((col_index, item)),
                        _ => None,
                    });

                match found {
                    Some((col_index, item)) => {
                        Some((row_index.to_owned(), col_index, item.to_owned()))
                    }
                    None => None,
                }
            })
            .unwrap();

        let guard_direction = match item {
            MapItem::Guard(direction) => direction,
            _ => panic!("Ooops"),
        };

        ((row_index as i32, col_index as i32), guard_direction)
    }

    fn steps(&self, number_of_steps: usize) -> Option<Map> {
        if number_of_steps <= 1 {
            return self.step();
        } else if let Some(step) = self.step() {
            return step.steps(number_of_steps - 1);
        }

        None
    }

    fn step_unil_guard_exits(&self) -> Map {
        let step = self.step();

        match step {
            Some(step) => {
                if step.contains_infinite_loop {
                    return step;
                }
                step.step_unil_guard_exits()
            }
            None => (self).copy(),
        }
    }

    fn step_unil_guard_exits_and_find_infinte_loops(&self) -> usize {
        let mut map = Some(self.copy());
        let mut total_loops = 0;

        loop {
            match map {
                Some(step) => {
                    let found_loop = step
                        .add_obstacle_in_front_of_guard_and_check_if_map_contains_infinite_loops();
                    total_loops += found_loop as usize;
                    println!(
                        "loops={}, count={}",
                        total_loops,
                        step.count_guard_visited()
                    );

                    map = step.step();
                }
                _ => break,
            }
        }

        total_loops
    }

    fn add_obstacle_in_front_of_guard_and_check_if_map_contains_infinite_loops(&self) -> bool {
        let mut map_with_obstruction = self.copy();
        let (coords, guard_direction) = map_with_obstruction.find_the_guard();
        let next_coords = guard_direction.step_forward(coords);
        if let Some(_) = self.get_item(next_coords) {
            map_with_obstruction.set_item(next_coords, MapItem::Obstacle);
        } else {
            return false;
        }

        let walked_obstruction_map = map_with_obstruction.step_unil_guard_exits();
        walked_obstruction_map.contains_infinite_loop
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step_forward(&self, (row_index, col_index): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (row_index - 1, col_index),
            Direction::Right => (row_index, col_index + 1),
            Direction::Down => (row_index + 1, col_index),
            Direction::Left => (row_index, col_index - 1),
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
    GuardVisited(Direction),
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

fn read_map(filename: &str) -> Map {
    let lines = read_lines(filename);
    let items = lines
        .map(|line| line.chars().map(|c| read_char(c)).collect::<Vec<MapItem>>())
        .collect::<Vec<Vec<MapItem>>>();
    Map {
        items,
        contains_infinite_loop: false,
    }
}

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
        let map = read_map("./input_example");
        assert_eq!(map.get_item((0, 0)).unwrap(), MapItem::Empty);
        assert_eq!(map.get_item((0, 4)).unwrap(), MapItem::Obstacle);
        assert_eq!(map.get_item((6, 4)).unwrap(), MapItem::Guard(Direction::Up));
    }

    #[test]
    fn test_step() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let next_map = map.step().unwrap();

        // Assert
        assert_eq!(map.count_guard_visited(), 1);
        assert_eq!(map.get_item((5, 4)).unwrap(), MapItem::Empty);
        assert_eq!(map.get_item((6, 4)).unwrap(), MapItem::Guard(Direction::Up));
        assert_eq!(
            next_map.get_item((6, 4)).unwrap(),
            MapItem::GuardVisited(Direction::Up)
        );
        assert_eq!(
            next_map.get_item((5, 4)).unwrap(),
            MapItem::Guard(Direction::Up)
        );
        assert_eq!(next_map.count_guard_visited(), 2);
    }

    #[test]
    fn test_step_forward() {
        assert_eq!((Direction::Up).step_forward((6, 4)), (5, 4));
        assert_eq!((Direction::Down).step_forward((6, 4)), (7, 4));
        assert_eq!((Direction::Left).step_forward((6, 4)), (6, 3));
        assert_eq!((Direction::Right).step_forward((6, 4)), (6, 5));
    }
    #[test]
    fn test_turn_right() {
        assert_eq!((Direction::Up).turn_right(), Direction::Right);
        assert_eq!((Direction::Right).turn_right(), Direction::Down);
        assert_eq!((Direction::Down).turn_right(), Direction::Left);
        assert_eq!((Direction::Left).turn_right(), Direction::Up);
    }

    #[test]
    fn test_steps() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let next_map = map.steps(2).unwrap();

        // Assert
        assert_eq!(map.get_item((6, 4)).unwrap(), MapItem::Guard(Direction::Up));
        assert_eq!(
            next_map.get_item((4, 4)).unwrap(),
            MapItem::Guard(Direction::Up)
        );
        assert_eq!(next_map.count_guard_visited(), 3);
    }

    #[test]
    fn test_steps_obstacle_should_turn_guard() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let next_map = map.steps(6).unwrap();

        // Assert
        assert_eq!(map.get_item((6, 4)).unwrap(), MapItem::Guard(Direction::Up));
        assert_eq!(
            next_map.get_item((1, 4)).unwrap(),
            MapItem::Guard(Direction::Right)
        );
        assert_eq!(next_map.count_guard_visited(), 6);
    }

    #[test]
    fn test_steps_too_many_steps_returns_none() {
        // Arrange
        let map = read_map("./input_example");
        // Act
        let next_map = map.steps(70);

        // Assert
        assert_eq!(next_map, None);
    }

    #[test]
    fn test_step_unil_guard_exits() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let next_map = map.step_unil_guard_exits();

        // Assert
        assert_eq!(next_map.count_guard_visited(), 41);
    }

    #[test]
    fn test_step_unil_guard_exits2() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let loops = map.step_unil_guard_exits_and_find_infinte_loops();

        // Assert
        assert_eq!(loops, 6);
    }

    #[test]
    fn test_part_1() {
        let map = read_map("./input_example");
        let sum = part1(&map);
        assert_eq!(sum, 41);
    }

    #[test]
    fn test_part_2() {
        let map = read_map("./input_example");
        let sum = part2(&map);
        assert_eq!(sum, 6);
    }
}
