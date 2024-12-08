use common::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

fn read_file(path: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();
    let mut pages = Vec::new();
    let mut read_rules = true;

    let lines = read_lines(path).expect("unable to read file");
    for line in lines.flatten() {
        if line.len() == 0 {
            read_rules = false;
            continue;
        }

        if read_rules {
            let r: Vec<_> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            rules.entry(r[0]).or_insert(HashSet::new()).insert(r[1]);
        } else {
            let p: Vec<_> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            pages.push(p);
        }
    }

    (rules, pages)
}
