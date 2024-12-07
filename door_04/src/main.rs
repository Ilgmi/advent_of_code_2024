use common::read_lines;
use std::io::BufRead;
use std::ops::{AddAssign, Sub};

fn main() {
    let state = create_state("door_04/input.txt");
    let count = count_xmases(&state);
    println!("{}", count);
}

struct Letter {
    pub letter: char,
    pub x: i32,
    pub y: i32,
}

struct Position {
    pub x: i32,
    pub y: i32,
}

struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Letter {
    fn new(letter: char, x: i32, y: i32) -> Self {
        Letter { letter, x, y }
    }

    fn check(&self, l: &char) -> bool {
        self.letter == *l
    }
}

fn create_state(path: &str) -> Vec<Vec<char>> {
    let lines = read_lines(path).expect("Couldn't open file");

    let mut state: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        let chars = line.chars().collect();
        state.push(chars);
    }

    state
}

fn count_xmases(state: &Vec<Vec<char>>) -> usize {
    let positions = vec![
        Direction { x: 1, y: 1 },
        Direction { x: -1, y: -1 },
        Direction { x: -1, y: 1 },
        Direction { x: 1, y: -1 },
    ];

    let mut result = 0;
    for (y, line) in state.iter().enumerate() {
        let mut y = y;
        for (x, char) in line.iter().enumerate() {
            let char = (*char).to_ascii_uppercase();
            if char == 'A' {
                let one = {};

                let x = x as i32;
                let y = y as i32;
                let mut count = 0;
                for position in &positions {
                    let x = x + (position.x * -1);
                    let y = y + (position.y * -1);

                    let p = Position { x, y };

                    let c = check(&state, &p, position, "MAS");
                    count.add_assign(c);
                }

                if count == 2 {
                    result.add_assign(1);
                }
            }
        }
        y.add_assign(1);
    }

    result
}

fn check(state: &Vec<Vec<char>>, index: &Position, direction: &Direction, word: &str) -> usize {
    let mut x = index.x;
    let mut y = index.y;
    for letter in word.chars() {
        let line = state.get(y as usize);
        if line == None {
            return 0;
        }

        let character = line.unwrap().get(x as usize);
        if character == None {
            return 0;
        }
        let character = character.unwrap().to_ascii_uppercase();
        if letter != character {
            return 0;
        }

        x.add_assign(direction.x);
        y.add_assign(direction.y);
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right() {
        let s = vec![vec!['X', 'M', 'A', 'S']];

        let direction = Direction { x: 1, y: 0 };

        let p = Position { x: 0, y: 0 };
        let count = check(&s, &p, &direction);

        assert_eq!(count, 1);
    }

    #[test]
    fn test_count_xmases() {
        let state =
            create_state("/home/milg/RustroverProjects/advent_of_code_2024/door_04/test.txt");
        let count = count_xmases(&state);

        assert_eq!(count, 18);
    }
}
