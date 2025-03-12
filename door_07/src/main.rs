use common::read_lines;
use std::ops::{AddAssign, BitAnd};
use std::str::FromStr;

fn main() {
    let path = "door_07/input.txt";
    let number_of_true: i128 = read_lines(path)
        .unwrap()
        .map(|l| Equation::from(l.unwrap().as_str()))
        .filter_map(|e| {
            if e.has_solution_2() {
                return Some(e.result);
            }
            None
        })
        .sum();
    print!("Number of true solutions: {}", number_of_true);
}

pub struct State {
    num_of_states: usize,
    state: Vec<u32>,
}

impl State {
    pub fn new(size: usize, num_of_states: usize) -> Self {
        Self {
            num_of_states,
            state: vec![0; size],
        }
    }

    pub fn state(&self) -> &[u32] {
        self.state.as_slice()
    }

    pub fn next(&mut self) {
        let mut overflow = false;
        for (index, state) in self.state.iter_mut().enumerate() {
            if index == 0 {
                state.add_assign(1);
                if *state == self.num_of_states as u32 {
                    overflow = true;
                    *state = 0;
                }
            } else {
                if overflow {
                    overflow = false;
                    state.add_assign(1);
                    if *state == self.num_of_states as u32 {
                        overflow = true;
                        *state = 0;
                    }
                }
            }
        }
    }
}

pub struct Equation {
    pub result: i128,
    pub numbers: Vec<i128>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let a = value.find(":").unwrap();
        let result = value[..a].parse::<i128>().unwrap();
        let values = &value[a + 1..];
        let numbers = values
            .trim()
            .split(" ")
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        Self { result, numbers }
    }
}

impl Equation {
    pub fn has_solution(&self) -> bool {
        let size = self.numbers.len() - 1;
        let size_of_solution_room = 2_u32.pow(size as u32);
        for i in 0..size_of_solution_room {
            let mut res = self.numbers.get(0).unwrap().clone();
            for j in 0..size {
                let op_pos = 1 << j;
                let op = i.bitand(op_pos) >> j;
                let n = self.numbers.get(j + 1).unwrap().clone();
                match op {
                    0 => res = res + n,
                    1 => match res.checked_mul(n) {
                        None => continue,
                        Some(r) => res = r,
                    },
                    _ => {
                        panic!("Should not be reached")
                    }
                }
            }
            if res == self.result {
                return true;
            }
        }
        false
    }

    pub fn has_solution_2(&self) -> bool {
        let mut state_machine = State::new(self.numbers.len() - 1, 3);
        let size = self.numbers.len() - 1;
        let size_of_solution_room = 3_u32.pow(size as u32);
        for i in 0..size_of_solution_room {
            let state = state_machine.state();
            let mut res = self.numbers[0];
            let mut index = 1;
            for operation in state {
                match operation {
                    0 => res += self.numbers.get(index).unwrap().clone(),
                    1 => res *= self.numbers.get(index).unwrap().clone(),
                    2 => {
                        res = i128::from_str(&format!(
                            "{res}{}",
                            self.numbers.get(index).unwrap().clone()
                        ))
                        .unwrap()
                    }
                    _ => panic!("Should not reach"),
                }
                index += 1;
            }
            if res == self.result {
                return true;
            }

            state_machine.next();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_equation() {
        let equation_str = "123: 1 2 3";
        let equation: Equation = equation_str.into();
        assert_eq!(equation.result, 123);
        assert_eq!(equation.numbers.len(), 3);
    }

    #[test]
    fn test_simple_equation_that_has_a_solution() {
        let equation_str = "190: 10 19";
        let equation: Equation = equation_str.into();
        assert_eq!(equation.result, 190);
        assert_eq!(equation.numbers.len(), 2);

        assert_eq!(equation.has_solution(), true);
    }

    #[test]
    fn test_simple_equation_that_has_not_a_solution() {
        let equation_str = "190: 1 19";
        let equation: Equation = equation_str.into();
        assert_eq!(equation.result, 190);
        assert_eq!(equation.numbers.len(), 2);

        assert_eq!(equation.has_solution(), false);
    }

    #[test]
    fn test_equations() {
        let equations = vec![
            (Equation::from("190: 10 19"), true),
            (Equation::from("190: 1 10 19"), true),
            (Equation::from("190: 10 1 19"), true),
            (Equation::from("190: 1 1 19"), false),
            (Equation::from("3267: 81 40 27"), true),
            (Equation::from("21037: 9 7 18 13"), false),
            (Equation::from("292: 11 6 16 20"), true),
        ];

        for (equation, expected) in equations {
            assert_eq!(equation.has_solution(), expected);
        }
    }

    #[test]
    fn test_equation_2_simple() {
        let equation = Equation::from("156: 15 6");
        assert_eq!(equation.has_solution_2(), true);
    }

    #[test]
    fn test_equation_2_advanced() {
        let equation = Equation::from("7290: 6 8 6 15");
        assert_eq!(equation.has_solution_2(), true);
    }
    #[test]
    fn test_equation_2_advanced_2() {
        let equation = Equation::from("192: 17 8 14");
        assert_eq!(equation.has_solution_2(), true);
    }

    #[test]
    fn test_equations_with_second() {
        let equations = vec![
            (Equation::from("190: 10 19"), true),
            (Equation::from("190: 1 10 19"), true),
            (Equation::from("190: 10 1 19"), true),
            (Equation::from("190: 1 1 19"), false),
            (Equation::from("3267: 81 40 27"), true),
            (Equation::from("21037: 9 7 18 13"), false),
            (Equation::from("292: 11 6 16 20"), true),
        ];

        for (equation, expected) in equations {
            assert_eq!(equation.has_solution_2(), expected);
        }
    }

    #[test]
    fn test_state() {
        let mut state = State::new(2, 2);
        assert_eq!(state.state(), &[0, 0]);
        state.next();
        assert_eq!(state.state(), &[1, 0]);
        state.next();
        assert_eq!(state.state(), &[0, 1]);
        state.next();
        assert_eq!(state.state(), &[1, 1]);
        state.next();
        assert_eq!(state.state(), &[0, 0]);
    }

    #[test]
    fn test_state_with_size_of_three() {
        let mut state = State::new(3, 2);
        assert_eq!(state.state(), &[0, 0, 0]);
        state.next();
        assert_eq!(state.state(), &[1, 0, 0]);
        state.next();
        assert_eq!(state.state(), &[0, 1, 0]);
        state.next();
        assert_eq!(state.state(), &[1, 1, 0]);
        state.next();
        assert_eq!(state.state(), &[0, 0, 1]);
        state.next();
        assert_eq!(state.state(), &[1, 0, 1]);
        state.next();
        assert_eq!(state.state(), &[0, 1, 1]);
        state.next();
        assert_eq!(state.state(), &[1, 1, 1]);
    }

    #[test]
    fn test_state_with_three_pos_states() {
        let mut state = State::new(2, 3);
        assert_eq!(state.state(), &[0, 0]);
        state.next();
        assert_eq!(state.state(), &[1, 0]);
        state.next();
        assert_eq!(state.state(), &[2, 0]);
        state.next();
        assert_eq!(state.state(), &[0, 1]);
        state.next();
        assert_eq!(state.state(), &[1, 1]);
        state.next();
        assert_eq!(state.state(), &[2, 1]);
        state.next();
        assert_eq!(state.state(), &[0, 2]);
        state.next();
        assert_eq!(state.state(), &[1, 2]);
        state.next();
        assert_eq!(state.state(), &[2, 2]);
        state.next();
        assert_eq!(state.state(), &[0, 0]);
    }
}
