use common::read_lines;
use std::collections::HashSet;

fn main() {
    let map = Map::from_file("door_12/input.txt");
    let sum = map.price();
    println!("Part 1: {}", sum);
}

struct Map {
    crops: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(map: &str) -> Self {
        let lines = map.lines();
        let mut crops = Vec::new();

        for (y, cop) in lines.enumerate() {
            let row: Vec<char> = cop.chars().collect();
            crops.push(row)
        }
        let width = crops.first().unwrap().len();
        let height = crops.len();
        Self {
            crops,
            width,
            height,
        }
    }

    fn from_file(path: &str) -> Self {
        let lines = read_lines(path).unwrap();
        let mut crops = Vec::new();

        for (y, cop) in lines.enumerate() {
            let row: Vec<char> = cop.unwrap().chars().collect();
            crops.push(row)
        }
        let width = crops.first().unwrap().len();
        let height = crops.len();
        Self {
            crops,
            width,
            height,
        }
    }

    fn get_by_pos(&self, pos: &Position) -> Option<&char> {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }
        self.crops.get(y as usize).unwrap().get(x as usize)
    }

    fn price(&self) -> usize {
        let mut processed_positions: HashSet<Position> = HashSet::new();
        let mut sum = 0;
        for (y, row) in self.crops.iter().enumerate() {
            for (x, crop) in row.iter().enumerate() {
                if *crop == ' ' {
                    continue;
                }
                let pos = Position {
                    x: x as i32,
                    y: y as i32,
                };
                if !processed_positions.contains(&pos) {
                    let region = self.get_region(&pos, crop);

                    sum += self.calc_region_price(crop, &region);

                    for pos in region {
                        processed_positions.insert(pos);
                    }
                }
            }
        }

        sum
    }

    fn get_region(&self, start_position: &Position, crop: &char) -> HashSet<Position> {
        let mut positions: HashSet<Position> = HashSet::new();
        let mut queue = Vec::new();
        queue.push(start_position.clone());
        while let Some(pos) = queue.pop() {
            positions.insert(pos.clone());
            for next_crop_direction in &DIRECTIONS {
                let position = pos.next(next_crop_direction);
                if let Some(next_crop) = self.get_by_pos(&position) {
                    if next_crop == crop && !positions.contains(&position) {
                        queue.push(position);
                    }
                }
            }
        }
        positions
    }

    fn calc_region_price(&self, crop: &char, region: &HashSet<Position>) -> usize {
        let mut number_of_crops = 0;
        let mut number_of_perimeters = 0;
        let mut processed = HashSet::new();

        let mut number: usize = 0;
        for pos in region {
            processed.insert(pos.clone());
            number = 0;
            for corner in CORNERS {
                if self.has_perimeter(corner, pos, crop, &processed) {
                    number += 1;
                }
            }

            if number > 0 {
                number_of_crops += 1;
                number_of_perimeters += number;
            }
        }

        region.len() * number_of_perimeters
    }

    fn has_perimeter(
        &self,
        corners: &[Direction],
        pos: &Position,
        crop: &char,
        processed_positions: &HashSet<Position>,
    ) -> bool {
        let mut same_crop = 0;
        for dir in corners {
            let n_pos = pos.next(dir);
            let n_crop = self.get_by_pos(&n_pos);

            if processed_positions.contains(&n_pos) {
                return false;
            }

            if let Some(n_crop) = n_crop {
                if n_crop == crop {
                    same_crop += 1;
                    continue;
                }
            }
        }
        if same_crop == 3 {
            return false;
        }

        true
    }
}

const CORNERS: [&[Direction]; 4] = [&TOP_LEFT, &TOP_RIGHT, &BOTTOM_RIGHT, &BOTTOM_LEFT];
const DIRECTIONS: [Direction; 4] = [
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
    Direction::Left,
];
const TOP_LEFT: [Direction; 3] = [Direction::Left, Direction::TopLeft, Direction::Top];
const TOP_RIGHT: [Direction; 3] = [Direction::Top, Direction::TopRight, Direction::Right];
const BOTTOM_RIGHT: [Direction; 3] = [Direction::Right, Direction::BottomRight, Direction::Bottom];
const BOTTOM_LEFT: [Direction; 3] = [Direction::Bottom, Direction::BottomLeft, Direction::Left];

#[derive(Eq, PartialEq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn next(&self, direction: &Direction) -> Position {
        match direction {
            Direction::TopLeft => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::Top => Self {
                y: self.y - 1,
                x: self.x,
            },
            Direction::TopRight => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::Right => Self {
                y: self.y,
                x: self.x + 1,
            },
            Direction::BottomRight => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::Bottom => Self {
                y: self.y + 1,
                x: self.x,
            },
            Direction::BottomLeft => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::Left => Self {
                y: self.y,
                x: self.x - 1,
            },
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_region_test() {
        let map = Map::from_str("AAAAA\nABABA\nAAAAA\nABABA\nAAAAA");
        let region = map.get_region(&Position::new(0, 0), &'A');
        assert_eq!(region.len(), 21)
    }

    #[test]
    fn test_simple_map() {
        let maps = vec![("AAAAA\nABABA\nAAAAA\nABABA\nAAAAA", 772)];

        for (map, expected) in maps {
            let map = Map::from_str(map);
            let sum = map.price();
            assert_eq!(sum, expected);
        }
    }

    #[test]
    fn test_simple_maps() {
        let maps = vec![
            ("AAAA", 40),
            ("BB\nBB", 32),
            ("C \nCC\n C", 40),
            ("D", 4),
            ("AAAAA\nABABA\nAAAAA\nABABA\nAAAAA", 772),
        ];

        for (map, expected) in maps {
            let map = Map::from_str(map);
            let sum = map.price();
            assert_eq!(sum, expected);
        }
    }

    #[test]
    fn test_complex_map() {
        let map = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let map = Map::from_str(map);
        let sum = map.price();
        assert_eq!(sum, 1930);
    }
}
