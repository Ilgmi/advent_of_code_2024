use common::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    let (rules, pages) = read_file("door_05/input.txt");

    let mut count = 0;
    let mut count_fixed = 0;
    for mut page in pages {
        if page.is_correct(&rules) {
            count += page.get_middle();
        } else {
            while !page.is_correct(&rules) {
                page.fix(&rules);
            }

            count_fixed += page.get_middle();
        }
    }

    println!("{}", count);
    println!("{}", count_fixed);
}

struct Rule {
    pub page: i32,
    pub before: HashSet<i32>,
}

impl Rule {
    pub fn new(page: i32) -> Self {
        Self {
            page,
            before: HashSet::new(),
        }
    }

    pub fn add(&mut self, x: i32) {
        self.before.insert(x);
    }
}

struct Pages(Vec<i32>);

impl Pages {
    pub(crate) fn fix(&mut self, rules: &HashMap<i32, Rule>) {
        let mut first = 0;
        let mut second = 0;

        for (index, page) in self.0.iter().enumerate() {
            let page_rules = rules.get(&page);
            let page_rule = if page_rules.is_none() {
                continue;
            } else {
                page_rules.unwrap()
            };

            for read_index in (0..index) {
                let page = self.0.get(read_index).unwrap();
                if page_rule.before.contains(page) {
                    first = index;
                    second = read_index;
                    break;
                }
            }
        }

        let first_val = self.0[first];
        let second_val = self.0[second];
        self.0[first] = second_val;
        self.0[second] = first_val;
    }
}

impl Pages {
    pub fn new() -> Self {
        Pages(Vec::new())
    }

    pub fn add_page(&mut self, page: i32) {
        self.0.push(page);
    }

    pub fn get_middle(&self) -> i32 {
        let middle_index = self.0.len() / 2;
        self.0[middle_index]
    }

    pub fn is_correct(&self, rules: &HashMap<i32, Rule>) -> bool {
        for (index, page) in self.0.iter().enumerate() {
            let page_rules = rules.get(&page);
            let page_rule = if page_rules.is_none() {
                continue;
            } else {
                page_rules.unwrap()
            };

            for read_index in (0..index) {
                let page = self.0.get(read_index).unwrap();
                if page_rule.before.contains(page) {
                    return false;
                }
            }
        }

        true
    }
}

fn read_file(path: &str) -> (HashMap<i32, Rule>, Vec<Pages>) {
    let mut rules: HashMap<i32, Rule> = HashMap::new();
    let mut pages: Vec<Pages> = Vec::new();
    let mut read_rules = true;

    let lines = read_lines(path).expect("unable to read file");
    for line in lines.flatten() {
        if line.len() == 0 {
            read_rules = false;
            continue;
        }

        if read_rules {
            let r: Vec<_> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            rules.entry(r[0]).or_insert(Rule::new(r[0])).add(r[1]);
        } else {
            let mut page = Pages::new();
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|x| page.add_page(x));
            pages.push(page);
        }
    }

    (rules, pages)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_page() {}
}
