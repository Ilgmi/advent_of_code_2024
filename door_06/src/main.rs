use common::read_lines;

fn main() {
    let (map, guard) = read_file("door_06/input.txt");
}

enum Field {
    Empty(bool),
    Obstacle,
}

struct Position {
    pub x: i32,
    pub y: i32,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Guard {
    pos: Position,
    direction: Direction,
}

impl Guard {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        let pos = Position { x, y };
        Self { pos, direction }
    }
}

fn read_file(path: &str) -> (Vec<Vec<Field>>, Some(Guard)) {
    let mut map: Vec<Vec<Field>> = Vec::new();
    let mut guard: Option<Guard> = None;
    let lines = read_lines(path).expect(&format!("Failed to read {}", path));
    for (y, line) in lines.enumerate() {
        let line = line.expect("Failed to read line");
        let mut l = Vec::new();
        for (x, field) in line.chars().enumerate() {
            match field {
                '#' => l.push(Field::Obstacle),
                '^' => guard = Some(Guard::new(x as i32, y as i32, Direction::Up)),
                _ => l.push(Field::Empty(false)),
            }
        }
    }

    (map, guard)
}
