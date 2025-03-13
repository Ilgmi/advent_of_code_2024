use common::read_lines;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;

fn main() {
    let lines = read_lines("door_08/input.txt").unwrap();
    let map = AntennasMap::from_lines_with_bufreader(lines);
    print!("{}", map.count_antennas_2())
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Distance {
    x: i32,
    y: i32,
}

impl Distance {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn rev(self) -> Self {
        Self {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Position) -> Distance {
        Distance::new(self.x - other.x, self.y - other.y)
    }

    pub fn calculate_antinode_position(&self, distance: &Distance) -> Self {
        Self {
            x: self.x - distance.x,
            y: self.y - distance.y,
        }
    }
}

struct AntennasMap {
    pub width: usize,
    pub height: usize,
    pub antennas: HashMap<char, Vec<Position>>,
}

impl AntennasMap {
    fn from_lines_with_bufreader(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut width: usize = 0;
        let mut height = 0;
        let mut antennas = HashMap::new();

        for (y, line) in lines.enumerate() {
            let line = line.unwrap();
            width = line.len();
            for (x, entry) in line.chars().enumerate() {
                if entry.is_ascii_alphabetic() || entry.is_ascii_digit() {
                    let positions = antennas.entry(entry).or_insert(Vec::new());
                    positions.push(Position::new(x as i32, y as i32));
                }
            }
            height += 1;
        }

        Self {
            width,
            height,
            antennas,
        }
    }

    fn from_lines(lines: core::str::Lines) -> Self {
        let mut width: usize = 0;
        let mut height = 0;
        let mut antennas = HashMap::new();

        for (y, line) in lines.enumerate() {
            width = line.len();
            for (x, entry) in line.chars().enumerate() {
                if entry.is_ascii_alphabetic() || entry.is_ascii_digit() {
                    let positions = antennas.entry(entry).or_insert(Vec::new());
                    positions.push(Position::new(x as i32, y as i32));
                }
            }
            height += 1;
        }

        Self {
            width,
            height,
            antennas,
        }
    }

    fn count_antennas(&self) -> usize {
        let mut anitnode_positions = HashSet::new();
        for (name, positions) in &self.antennas {
            for position in positions {
                for other in positions {
                    if position == other {
                        continue;
                    }

                    let distance = position.distance(other).rev();
                    let antinode_pos = position.calculate_antinode_position(&distance);
                    anitnode_positions.insert(antinode_pos.clone());
                }
            }
        }

        let mut count = 0;
        for y in 0..self.height {
            let mut dbg = String::new();
            for x in 0..self.width {
                let p = Position::new(x as i32, y as i32);
                if anitnode_positions.contains(&p) {
                    dbg.push('#');
                    count += 1;
                    continue;
                }
                dbg.push('.')
            }
            println!("{dbg}")
        }

        count
    }

    fn count_antennas_2(&self) -> usize {
        let mut anitnode_positions = HashSet::new();
        for (name, positions) in &self.antennas {
            for position in positions {
                for other in positions {
                    if position == other {
                        continue;
                    }

                    let distance = position.distance(other).rev();

                    let mut pos = position.clone();
                    loop {
                        let antinode_pos = pos.calculate_antinode_position(&distance);
                        anitnode_positions.insert(antinode_pos.clone());
                        pos = antinode_pos;

                        if pos.x < 0
                            || pos.x >= self.width as i32 && pos.y < 0
                            || pos.y >= self.height as i32
                        {
                            break;
                        }
                    }

                    let distance = position.distance(other);

                    let mut pos = position.clone();
                    loop {
                        let antinode_pos = pos.calculate_antinode_position(&distance);
                        anitnode_positions.insert(antinode_pos.clone());
                        pos = antinode_pos;

                        if pos.x < 0
                            || pos.x >= self.width as i32 && pos.y < 0
                            || pos.y >= self.height as i32
                        {
                            break;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for y in 0..self.height {
            let mut dbg = String::new();
            for x in 0..self.width {
                let p = Position::new(x as i32, y as i32);
                if anitnode_positions.contains(&p) {
                    dbg.push('#');
                    count += 1;
                    continue;
                }
                dbg.push('.')
            }
            println!("{dbg}")
        }

        count
    }

    fn is_inside(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_lines;

    const example: &'static str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn create_map() {
        let lines = example.lines();
        let antennas = AntennasMap::from_lines(lines);
        assert_eq!(antennas.width, 12);
        assert_eq!(antennas.height, 12);
        assert_eq!(antennas.antennas.len(), 2);
    }

    #[test]
    fn count_antinode_positions_with_simple_example() {
        let lines = example.lines();
        let antennas = AntennasMap::from_lines(lines);
        assert_eq!(antennas.width, 12);
        assert_eq!(antennas.height, 12);

        let count = antennas.count_antennas();
        assert_eq!(count, 14);
    }

    #[test]
    fn count_antinode_positions_2_with_simple_example() {
        let lines = example.lines();
        let antennas = AntennasMap::from_lines(lines);
        assert_eq!(antennas.width, 12);
        assert_eq!(antennas.height, 12);

        let count = antennas.count_antennas_2();
        assert_eq!(count, 34);
    }

    #[test]
    fn create_map_from_input() {
        let path = "/home/milg/RustroverProjects/advent_of_code_2024/door_08/input_test.txt";
        let lines = read_lines(path).unwrap();
        let antennas = AntennasMap::from_lines_with_bufreader(lines);
        assert_eq!(antennas.width, 12);
        assert_eq!(antennas.height, 12);
        assert_eq!(antennas.antennas.len(), 2);
    }
}
