use shared::read_lines;

fn main() {
    let filename = "./input";
    let _matrix = read_map(filename);
    // let total_found = part1(&matrix);
    // println!("part 1: {}", total_found);

    // let found = part2(&matrix);
    // println!("part 2: {}", found);
}

#[derive(Debug)]
struct Map {
    items: Vec<Vec<MapItem>>,
}

impl Map {
    fn get_item(&self, (row, col): (usize, usize)) -> MapItem {
        self.items[row][col].to_owned()
    }

    fn set_item(&mut self, (row, col): (usize, usize), item: MapItem) {
        self.items[row][col] = item;
    }

    fn count_guard_visited(&self) -> usize {
        self.items
            .iter()
            .flatten()
            .filter(|item| match item {
                MapItem::Guard(_) => true,
                MapItem::GuardVisited => true,
                _ => false,
            })
            .count()
    }

    fn copy(&self) -> Map {
        Map {
            items: self.items.to_vec(),
        }
    }

    fn step(&self) -> Map {
        let mut next_map = self.copy();

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
        let coords = (row_index, col_index);
        let next_coords = guard_direction.step_forward(coords);

        println!("now={},{}, next=,{:?}", row_index, col_index, next_coords);

        if MapItem::Obstacle == self.get_item(next_coords) {
            next_map.set_item(coords, MapItem::Guard(guard_direction.turn_right()));
        } else {
            next_map.set_item(coords, MapItem::GuardVisited);
            // TODO CURRENT ITEM
            next_map.set_item(next_coords, MapItem::Guard(guard_direction));
        }

        next_map
    }

    fn steps(&self, number_of_steps: usize) -> Map {
        if number_of_steps <= 1 {
            return self.step();
        }
        self.step().steps(number_of_steps - 1)
    }
}

// fn fun_name(row_index: usize, col_index: usize) -> (usize, usize) {
//     let next_coords = (row_index - 1, col_index);
//     next_coords
// }

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step_forward(&self, (row_index, col_index): (usize, usize)) -> (usize, usize) {
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
    GuardVisited,
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
    Map { items }
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
        assert_eq!(map.get_item((0, 0)), MapItem::Empty);
        assert_eq!(map.get_item((0, 4)), MapItem::Obstacle);
        assert_eq!(map.get_item((6, 4)), MapItem::Guard(Direction::Up));
    }

    #[test]
    fn test_step() {
        // Arrange
        let map = read_map("./input_example");

        // Act
        let next_map = map.step();

        // Assert
        assert_eq!(map.count_guard_visited(), 1);
        assert_eq!(map.get_item((5, 4)), MapItem::Empty);
        assert_eq!(map.get_item((6, 4)), MapItem::Guard(Direction::Up));
        assert_eq!(next_map.get_item((6, 4)), MapItem::GuardVisited);
        assert_eq!(next_map.get_item((5, 4)), MapItem::Guard(Direction::Up));
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
        let next_map = map.steps(2);

        // Assert
        assert_eq!(map.get_item((6, 4)), MapItem::Guard(Direction::Up));
        assert_eq!(next_map.get_item((4, 4)), MapItem::Guard(Direction::Up));
        assert_eq!(next_map.count_guard_visited(), 3);
    }

    #[test]
    fn test_steps_obstacle_should_turn_guard() {
        // Arrange
        let map = read_map("./input_example");
        // Act
        let next_map = map.steps(6);

        // Assert
        assert_eq!(map.get_item((6, 4)), MapItem::Guard(Direction::Up));
        assert_eq!(next_map.get_item((1, 4)), MapItem::Guard(Direction::Right));
        assert_eq!(next_map.count_guard_visited(), 6);
    }

    #[test]
    fn test_todo() {
        // Arrange
        let map = read_map("./input_example");
        // Act
        let next_map = map.steps(70);

        // Assert
        dbg!(next_map);
        // assert_eq!(map.get_item((6, 4)), MapItem::Guard(Direction::Up));
        // assert_eq!(next_map.get_item((1, 4)), MapItem::Guard(Direction::Right));
        // assert_eq!(next_map.count_guard_visited(), 6);
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
