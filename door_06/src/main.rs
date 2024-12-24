use common::read_lines;

fn main() {
    let (mut map, guard) = read_file("door_06/input.txt");
    let mut guard = guard.expect("Some guard");

    while guard.is_in_map(&map) {
        update(&mut map, &mut guard);
        //print(&map);
    }
    print(&map);
    let visited_fields = map.get_visited_fields();
    println!("visited_fields: {:?}", visited_fields);
}

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
                Field::Obstacle => row.push('#'),
            }
        }
        println!("{}", row);
    }
}

fn update(map: &mut Map, guard: &mut Guard) {
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
            Field::Obstacle => {
                guard.rotate_right();
            }
        }
    }
}

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

enum Field {
    Cell(bool),
    Obstacle,
}

#[derive(Clone)]
struct Position {
    pub x: i32,
    pub y: i32,
}

struct Dir {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
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
    pub pos: Position,
    pub direction: Direction,
    pub path: Vec<Position>,
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
        Self {
            pos,
            direction,
            path: Vec::new(),
        }
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
                '#' => l.push(Field::Obstacle),
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
