use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{Add, Sub};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> anyhow::Result<()> {
    let mut result =
        read_lines("/home/milg/RustroverProjects/advent_of_code_2024/door_01/src/inputs.txt")?
            .flatten()
            .fold((Vec::new(), Vec::new()), |mut input, line| {
                let numbers = line.split("   ").collect::<Vec<&str>>();
                input
                    .0
                    .push(numbers.get(0).unwrap().parse::<i32>().unwrap());
                input
                    .1
                    .push(numbers.get(1).unwrap().parse::<i32>().unwrap());
                input
            });

    result.0.sort();
    result.1.sort();
    let solution_1 = part_one(&mut result);

    let solution_2 = part_two(&mut result);

    println!("{}", solution_1);
    println!("{}", solution_2);

    Ok(())
}

fn part_one(result: &mut (Vec<i32>, Vec<i32>)) -> i32 {
    let mut solution = 0;
    for (index, left) in result.0.iter().enumerate() {
        let right = result.1.get(index).unwrap();
        solution += left.sub(right).abs();
    }
    return solution;
}

fn part_two(result: &mut (Vec<i32>, Vec<i32>)) -> i32 {
    let right = result
        .1
        .clone()
        .into_iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry(item).and_modify(|e| *e += 1).or_insert(1);
            acc
        });
    let left: HashSet<i32> = HashSet::from_iter(result.0.clone());
    left.iter().fold(0, |acc, n| {
        let r = right.get(n).or(Some(&0)).unwrap();
        acc.add(n * r)
    })
}
