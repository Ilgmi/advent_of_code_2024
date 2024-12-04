use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::path::Path;

#[derive(Clone)]
#[derive(PartialEq)]
enum Direction {
    Init,
    Increasing,
    Decreasing,
}

fn main() {
    let file = File::open(Path::new("door_02/input.txt")).unwrap();
    let reader = BufReader::new(file);
    let numbers: Vec<_> = reader.lines()
        .map(|line| {
            let mut line = line.unwrap();
            let mut line: Vec<i32> = line
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect();

            let result = check(&line);


            return if !result {
                let result = check_all(line);
                result
            } else {
                result
            }
        })
        .collect();
    let count = numbers.iter().filter(|&n| *n).count();

    println!("{count}");
}


fn check(line: &[i32]) -> bool {


    let mut val_one = line.get(0).unwrap();
    let mut val_two = line.get(1).unwrap();

    let mut old_direction = Direction::Init;

    for index in 0..line.len() {
        val_one = line.get(index).unwrap();
        val_two = match line.get(index + 1) {
            None => {
                continue;
            }
            Some(val_two) => val_two,
        };
        let mut distance = val_one.sub(val_two).abs();

        if !(distance > 0 && distance <= 3) {
            return false;
        }

        let direction = if val_one.sub(val_two) > 0 {
            Direction::Decreasing
        } else {
            Direction::Increasing
        };

        if old_direction == Direction::Init {
            old_direction = direction.clone();
        }

        if old_direction != direction {
            return false;
        }

        old_direction = direction.clone();
    }

    true
}


fn check_all(line: Vec<i32>) -> bool {
    for index in 0..line.len() {
        let mut test_line: Vec<i32> = line.clone();
        test_line.remove(index);
        if check(&test_line) {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_check(){

        let vals = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", false),
            ("8 6 4 4 1", false),
            ("1 3 6 7 9", true),
        ];

        for (index,(val, expect)) in vals.iter().enumerate() {
            let mut val: Vec<i32> = val
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect();
            let result = check(&val);

            println!("check result: {result:?}");
            assert_eq!(*expect, result);
        }


    }

    #[test]
    fn test_check_with_tolerant(){

        let vals = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", true),
            ("8 6 4 4 1", true),
            ("1 3 6 7 9", true),
        ];

        for (index,(val, expect)) in vals.iter().enumerate() {
            let mut val: Vec<i32> = val
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect();
            let result = check(&val);


            if !result{
                let result = check_all(val);

                println!("check result: {result:?}");
                assert_eq!(*expect, result);
            }else {
                assert!(result)
            }


        }


    }
}