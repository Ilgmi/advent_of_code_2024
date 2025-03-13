use common::read_lines;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufReader, Lines};
use std::rc::Rc;

fn main() {
    let lines = read_lines("door_10/input.txt").unwrap();
    let map = Map::from(lines);
    let trails_score = map.search_trails();
    println!("{:?}", trails_score);
}

struct Map {
    map: Vec<Vec<Rc<Tile>>>,
}

impl Map {
    fn get_tail(&self, tail: &Tile, direction: &Direction) -> Option<Rc<Tile>> {
        match direction {
            Direction::Top => {
                if let Some(y) = tail.y.checked_sub(1) {
                    if let Some(row) = self.map.get(y) {
                        if let Some(tail) = row.get(tail.x) {
                            return Some(tail.clone());
                        }
                    }
                }
            }
            Direction::Right => {
                if let Some(row) = self.map.get(tail.y) {
                    if let Some(tail) = row.get(tail.x + 1) {
                        return Some(tail.clone());
                    }
                }
            }
            Direction::Bottom => {
                if let Some(row) = self.map.get(tail.y + 1) {
                    if let Some(tail) = row.get(tail.x) {
                        return Some(tail.clone());
                    }
                }
            }
            Direction::Left => {
                if let Some(row) = self.map.get(tail.y) {
                    if let Some(x) = tail.x.checked_sub(1) {
                        if let Some(tail) = row.get(x) {
                            return Some(tail.clone());
                        }
                    }
                }
            }
        }

        None
    }

    fn search_trails(&self) -> u32 {
        let mut tailheads: Vec<u32> = Vec::new();

        for row in self.map.as_slice() {
            for tile in row {
                if tile.size == 0 {
                    let searcher = TrailSearcher::new(tile.clone());
                    let found = searcher.search(self);
                    tailheads.push(found);
                }
            }
        }

        tailheads.iter().sum()
    }
}

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Tile {
    x: usize,
    y: usize,
    size: usize,
}

impl Tile {
    fn new(x: usize, y: usize, size: usize) -> Self {
        Self { x, y, size }
    }
}

struct Path {
    tail: Rc<Tile>,
    before: Option<Rc<Path>>,
}

impl Path {
    fn new(tail: Rc<Tile>, before: Option<Rc<Path>>) -> Self {
        Self { tail, before }
    }
}

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

struct TrailSearcher {
    trail: Rc<Path>,
}

impl TrailSearcher {
    fn new(start: Rc<Tile>) -> TrailSearcher {
        Self {
            trail: Rc::new(Path::new(start.clone(), None)),
        }
    }

    fn search(&self, map: &Map) -> u32 {
        let mut queue = vec![self.trail.clone()];
        let directions = vec![
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ];

        let mut possible_end = HashSet::new();

        while let Some(path) = queue.pop() {
            if path.tail.size == 9 {
                possible_end.insert(path.tail.clone());
                continue;
            }
            let next_size = path.tail.size + 1;
            for direction in &directions {
                if let Some(next_tail) = map.get_tail(&path.tail, direction) {
                    if next_size == next_tail.size {
                        queue.push(Rc::new(Path::new(next_tail.clone(), Some(path.clone()))));
                    }
                }
            }
        }

        possible_end.iter().count() as u32
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut map = Vec::new();
        for (y, line) in value.lines().enumerate() {
            let mut row: Vec<Rc<Tile>> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let size = c.to_string().parse::<usize>().unwrap_or(0);
                row.push(Rc::new(Tile::new(x, y, size)))
            }
            map.push(row);
        }
        Self { map }
    }
}

impl From<io::Lines<io::BufReader<File>>> for Map {
    fn from(value: Lines<BufReader<File>>) -> Self {
        let mut map = Vec::new();
        for (y, line) in value.enumerate() {
            let mut row: Vec<Rc<Tile>> = Vec::new();
            for (x, c) in line.unwrap().chars().enumerate() {
                let size = c.to_string().parse::<usize>().unwrap();
                row.push(Rc::new(Tile::new(x, y, size)))
            }
            map.push(row);
        }
        Self { map }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn should_create_map_successful() {
        let map = Map::from(TEST_INPUT);
        assert_eq!(map.map.len(), 8);
        assert_eq!(map.map.first().unwrap().len(), 8);
    }

    #[test]
    fn should_return_tails_from_direction_successful() {
        let map = Map::from(TEST_INPUT);
        let directions = vec![
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ];

        let tail = Tile::new(1, 1, 0);
        for direction in directions {
            let tile = map.get_tail(&tail, &direction);
            assert!(tile.is_some());
            let tile = tile.unwrap();
            assert!(tile.x <= 2 && tile.y <= 2)
        }
    }

    const MAP_WITH_TWO_TRAILHEAD: &str = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    #[test]
    fn should_find_trails() {
        let map = Map::from(MAP_WITH_TWO_TRAILHEAD);
        let tail = Tile::new(3, 0, 0);
        let searcher = TrailSearcher::new(Rc::new(tail));
        let trails_count = searcher.search(&map);
        assert_eq!(trails_count, 2);
    }

    #[test]
    fn should_find_all_trailhead_scores() {
        let map = Map::from(TEST_INPUT);
        let score = map.search_trails();
        assert_eq!(score, 36);
    }

    const MAP_WITH_COMPLEX_MAP: &str = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    #[test]
    fn should_find_trails_for_complex_map() {
        let map = Map::from(MAP_WITH_COMPLEX_MAP);
        let tail = Tile::new(3, 0, 0);
        let searcher = TrailSearcher::new(Rc::new(tail));
        let trails_count = searcher.search(&map);
        assert_eq!(trails_count, 4);
    }
}
