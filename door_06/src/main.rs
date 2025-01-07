use common::read_lines;
use std::collections::HashSet;

fn main() {
    let (mut start_map, guard) = read_file("door_06/input.txt");
    let mut start_guard = guard.expect("Some guard");

    let mut stuck_counter = 0;

    for y in (0..start_map.height) {
        for x in (0..start_map.width) {
            let mut map = start_map.clone();
            let mut guard = start_guard.clone();

            if x == guard.pos.x && y == guard.pos.y {
                continue;
            }

            {
                let field = map.get_field_mut(x, y).unwrap();
                match field {
                    Field::Cell(visited) => *field = Field::Obstacle(None),
                    Field::Obstacle(_) => {
                        continue;
                    }
                }
            }

            while guard.is_in_map(&map) {
                //print(&map);

                if update(&mut map, &mut guard) {
                    stuck_counter += 1;
                    break;
                }
            }
        }
    }

    println!("stuck_counter: {}", stuck_counter);
}

fn check_ob_pos(ob_pos: Position, mut map: Map, mut guard: Guard) {}

fn print(map: &Map) {
    for y in &map.map {
        let mut row = String::new();
        for x in y {
            match x {
                Field::Cell(v) => {
                    if *v {
                        row.push('x')
                    } else {
                        row.push('.')
                    }
                }
                Field::Obstacle(_) => row.push('#'),
            }
        }
        println!("{}", row);
    }
}

fn update(map: &mut Map, guard: &mut Guard) -> bool {
    let next_pos = guard.get_next_position();
    if map.is_out_of_bounds(&next_pos) {
        guard.move_to_next()
    } else {
        let next_field = &mut map.get_field_mut(next_pos.x, next_pos.y).unwrap();

        match next_field {
            Field::Cell(visited) => {
                guard.move_to_next();
                *visited = true;
            }
            Field::Obstacle(hit_from) => {
                let dir = guard.direction.clone();
                match hit_from {
                    None => *hit_from = Some(dir),
                    Some(direction) => {
                        if *direction == dir {
                            return true;
                        } else {
                            *hit_from = Some(dir)
                        }
                    }
                }

                guard.rotate_right();
            }
        }
    }

    false
}

#[derive(Clone)]
struct Map {
    pub map: Vec<Vec<Field>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub(crate) fn get_visited_fields(&self) -> usize {
        let mut count = 0;

        self.map.iter().for_each(|row| {
            row.iter().for_each(|field| {
                if let Field::Cell(true) = field {
                    count += 1;
                }
            })
        });

        count
    }
}

impl Map {
    fn new(map: Vec<Vec<Field>>) -> Self {
        let height = map.len() as i32;
        let width = map.get(0).unwrap().len() as i32;
        Map { map, width, height }
    }

    fn get_field_mut(&mut self, x: i32, y: i32) -> Option<&mut Field> {
        let mut rows: &mut Vec<Field> = self.map.get_mut(y as usize).expect("expect row");
        rows.get_mut(x as usize)
    }

    fn is_out_of_bounds(&self, pos: &Position) -> bool {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            return false;
        }

        true
    }
}

#[derive(Clone)]
enum Field {
    Cell(bool),
    Obstacle(Option<Direction>),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Position {
    pub x: i32,
    pub y: i32,
}

struct Dir {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_right(&mut self) {
        match self {
            Direction::Up => *self = Self::Right,
            Direction::Right => *self = Self::Down,
            Direction::Down => *self = Self::Left,
            Direction::Left => *self = Self::Up,
        }
    }
}

#[derive(Clone)]
struct Guard {
    pub start_pos: Position,
    pub pos: Position,
    pub direction: Direction,
}

impl Guard {
    pub(crate) fn get_next_position(&self) -> Position {
        let mut current_x = self.pos.x;
        let mut current_y = self.pos.y;
        match &self.direction {
            Direction::Up => {
                current_y -= 1;
            }
            Direction::Down => {
                current_y += 1;
            }
            Direction::Right => {
                current_x += 1;
            }
            Direction::Left => {
                current_x -= 1;
            }
        }
        Position {
            x: current_x,
            y: current_y,
        }
    }

    pub(crate) fn get_current_pos(&self) -> Position {
        self.pos.clone()
    }

    pub fn move_to_next(&mut self) {
        let next = self.get_next_position();
        self.pos = next;
    }

    pub fn rotate_right(&mut self) {
        self.direction.rotate_right();
    }

    pub fn is_in_map(&self, map: &Map) -> bool {
        !map.is_out_of_bounds(&self.pos)
    }
}

impl Guard {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        let pos = Position { x, y };
        let start_pos = Position { x, y };
        let mut path = HashSet::with_capacity(10);
        path.insert((Position { x, y }, Direction::Up));
        Self {
            start_pos,
            pos,
            direction,
        }
    }

    fn reset(&mut self) {
        self.pos = Position {
            x: self.start_pos.x,
            y: self.start_pos.y,
        };
    }
}

fn read_file(path: &str) -> (Map, Option<Guard>) {
    let mut map: Vec<Vec<Field>> = Vec::new();
    let mut guard: Option<Guard> = None;
    let lines = read_lines(path).expect(&format!("Failed to read {}", path));
    for (y, line) in lines.enumerate() {
        let line = line.expect("Failed to read line");
        let mut l = Vec::new();
        for (x, field) in line.chars().enumerate() {
            match field {
                '#' => l.push(Field::Obstacle(None)),
                '^' => {
                    l.push(Field::Cell(true));
                    guard = Some(Guard::new(x as i32, y as i32, Direction::Up))
                }
                _ => l.push(Field::Cell(false)),
            }
        }
        map.push(l);
    }

    (Map::new(map), guard)
}
