use rayon::iter::IntoParallelRefIterator;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::ops::{AddAssign, Deref};
use std::path::Path;
use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{Receiver, Sender};

const INPUT: &str = "4022724 951333 0 21633 5857 97 702 6";

#[tokio::main]
async fn main() {
    let blinks = 75;
    let mut stones = Stones::new(INPUT);

    // for round in 0..blinks {
    //     println!("{round}");
    //
    // }

    // calc_stones(75).await;

    let c = part_two(INPUT).unwrap();
    println!("{c}");
    println!("{}", stones.stones.len());
}

#[derive(Debug)]
struct StonesTwo {
    stones: Vec<u64>,
    stone_cache: HashMap<(u64, usize), u64>,
}

impl StonesTwo {
    pub fn from_input(input: &str) -> Self {
        Self {
            stones: input
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
            stone_cache: HashMap::new(),
        }
    }

    pub fn simulate(mut self, iterations: usize) -> u64 {
        self.stones
            .clone()
            .into_iter()
            .map(|s| self.blink(s, iterations))
            .sum()
    }

    // The idea is not to store the stones but rather check how many stones each starting stone produces after n iterations.
    // To further speed things up memoization is used
    fn blink(&mut self, stone: u64, iterations: usize) -> u64 {
        if iterations == 0 {
            return 1;
        }

        let parameters = (stone, iterations);
        if self.stone_cache.contains_key(&parameters) {
            return *self.stone_cache.get(&parameters).unwrap();
        }

        let count = if stone == 0 {
            self.blink(1, iterations - 1)
        } else if has_even_number_of_digits(stone) {
            let stone_string = stone.to_string();
            let left = stone_string[..stone_string.len() / 2]
                .parse::<u64>()
                .unwrap();
            let right = stone_string[stone_string.len() / 2..]
                .parse::<u64>()
                .unwrap();
            self.blink(left, iterations - 1) + self.blink(right, iterations - 1)
        } else {
            self.blink(stone * 2024, iterations - 1)
        };

        self.stone_cache.insert(parameters, count);

        count
    }
}

fn has_even_number_of_digits(value: u64) -> bool {
    value.to_string().len() % 2 == 0
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = StonesTwo::from_input(input);
    Some(stones.simulate(75))
}

async fn calc_stones(rounds: usize) -> usize {
    let input_path = Path::new("door_11/input/");
    let num_workers = 10;
    let mut count = 0;
    for round in 0..rounds {
        println!("{round}");
        let (writer_tx, result_rx) = create_writer(
            input_path
                .join((round + 1).to_string())
                .to_string_lossy()
                .to_string(),
        );
        let workers = create_workers(num_workers * (round + 1), writer_tx);

        let file = tokio::fs::File::open(input_path.join(round.to_string()))
            .await
            .unwrap();
        let mut lines = BufReader::new(file).lines();
        let mut i = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            let stone = Stone::from_str(&line).unwrap();

            let worker = workers.get(i % workers.len()).unwrap();
            worker.send(stone).await.unwrap();
            i += 1;
        }

        drop(workers);

        count = result_rx.await.unwrap();
    }

    count
}

fn create_writer(path: String) -> (Sender<Vec<Stone>>, tokio::sync::oneshot::Receiver<usize>) {
    let (result_sender, result_receiver) = tokio::sync::oneshot::channel::<usize>();
    let (stones_sender, stones_receiver) = tokio::sync::mpsc::channel::<Vec<Stone>>(100);

    tokio::task::spawn(writer(stones_receiver, path, result_sender));
    (stones_sender, result_receiver)
}

fn create_workers(num: usize, tx_writer: Sender<Vec<Stone>>) -> Vec<Sender<Stone>> {
    let mut channels = Vec::with_capacity(num);
    for _ in 0..num {
        let (tx_worker, rx_worker) = tokio::sync::mpsc::channel::<Stone>(100);
        tokio::task::spawn(worker(rx_worker, tx_writer.clone()));
        channels.push(tx_worker);
    }
    channels
}

async fn worker(mut rx: Receiver<Stone>, tx: Sender<Vec<Stone>>) {
    while let Some(stone) = rx.recv().await {
        let stones = stone.blink();
        tx.send(stones.clone()).await.unwrap();
    }
}

async fn writer(
    mut rx: Receiver<Vec<Stone>>,
    path: String,
    finished: tokio::sync::oneshot::Sender<usize>,
) {
    let path = Path::new(&path);
    let file = File::create(path).unwrap();
    let mut count = 0;
    let mut writer = LineWriter::new(file);
    while let Some(stones) = rx.recv().await {
        count += stones.len();
        for stone in stones {
            writer.write_all(stone.number.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        writer.flush().unwrap();
    }

    finished.send(count).unwrap();
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Stone {
    number: String,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Stones {
    stones: Vec<Stone>,
}

impl Stones {
    pub fn new(input: &str) -> Stones {
        let stones: Vec<Stone> = input.split(" ").map(|x| x.parse().unwrap()).collect();
        Stones { stones }
    }

    pub fn from_vec(stones: Vec<Stone>) -> Self {
        Self { stones }
    }

    pub fn blink(self) -> Self {
        let mut stones: Vec<Stone> = Vec::new();
        for stone in self.stones {
            for new_stone in stone.blink() {
                stones.push(new_stone);
            }
        }
        Self { stones }
    }

    pub fn blink_fast(self) -> Self {
        let mut works: Vec<Vec<Stone>> = Vec::new();

        let mut index = 0;
        for (i, stone) in self.stones.iter().enumerate() {
            if i == 0 {
                works.push(vec![]);
            } else if i % 100 == 0 {
                index += 1;
                works.push(vec![]);
            }
            works.get_mut(index).unwrap().push(stone.clone());
        }

        let mut tasks = Vec::new();
        for (i, x) in works.iter().enumerate() {
            let s = x.clone();
            let t = std::thread::spawn(move || do_work(i, s));
            tasks.push(t);
        }

        let mut results = BTreeMap::new();
        for task in tasks {
            let (i, stones) = task.join().unwrap();
            results.insert(i, stones);
        }

        let stones = results.values().flatten().cloned().collect();

        Self { stones }
    }
}

fn do_work(id: usize, stones: Vec<Stone>) -> (usize, Vec<Stone>) {
    let mut new_stones = Vec::new();
    for x in stones {
        for new_stone in x.blink() {
            new_stones.push(new_stone);
        }
    }
    (id, new_stones)
}

impl Display for Stones {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.stones
                .iter()
                .map(|x| x.number.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Stone {
    fn blink(self) -> Vec<Stone> {
        let number = self.number.parse::<u128>().unwrap();
        let mut stones = Vec::new();

        if number == 0 {
            stones.push("1".parse().unwrap());
        } else if number == 1 {
            stones.push("2024".parse().unwrap());
        } else {
            let digits_len = self.number.len();
            if digits_len % 2 == 0 {
                let (left, right) = self.number.split_at(digits_len / 2);
                stones.push(left.parse().unwrap());
                stones.push(right.parse().unwrap());
            } else {
                stones.push((number * 2024).to_string().parse().unwrap());
            }
        }

        stones
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number: u128 = s.parse().unwrap();
        Ok(Self {
            number: number.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_with_0_blinks_to_1() {
        let stone: Stone = "0".parse().unwrap();
        let stone = stone.blink();
        assert_eq!(stone[0].number, "1");
    }

    #[test]
    fn stone_with_1_blinks_to_2024() {
        let stone: Stone = "1".parse().unwrap();
        let stone = stone.blink();
        assert_eq!(stone[0].number, "2024");
    }

    #[test]
    fn stone_with_10_blinks_to_1_and_0() {
        let stone: Stone = "10".parse().unwrap();
        let stone = stone.blink();
        assert_eq!(stone[0].number, "1");
        assert_eq!(stone[1].number, "0");
    }

    #[test]
    fn stone_with_99_blinks_to_9_and_9() {
        let stone: Stone = "99".parse().unwrap();
        let stone = stone.blink();
        assert_eq!(stone[0].number, "9");
        assert_eq!(stone[1].number, "9");
    }

    #[test]
    fn stone_with_999_blinks_to_2021976() {
        let stone: Stone = "999".parse().unwrap();
        let stone = stone.blink();
        assert_eq!(stone[0].number, "2021976");
    }

    #[test]
    fn blik_from_stones() {
        let stones = Stones::new("0");
        let stones = stones.blink();
        assert_eq!(stones.stones[0].number, "1");
    }

    #[test]
    fn blink_stones() {
        let tests = vec![
            (Stones::new("0"), Stones::new("1")),
            (Stones::new("1"), Stones::new("2024")),
            (Stones::new("11"), Stones::new("1 1")),
            (
                Stones::new("0 1 10 99 999"),
                Stones::new("1 2024 1 0 9 9 2021976"),
            ),
            (Stones::new("125 17"), Stones::new("253000 1 7")),
            (Stones::new("253000 1 7"), Stones::new("253 0 2024 14168")),
            (
                Stones::new("253 0 2024 14168"),
                Stones::new("512072 1 20 24 28676032"),
            ),
            (
                Stones::new("512072 1 20 24 28676032"),
                Stones::new("512 72 2024 2 0 2 4 2867 6032"),
            ),
            (
                Stones::new("512 72 2024 2 0 2 4 2867 6032"),
                Stones::new("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
            ),
            (
                Stones::new("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
                Stones::new("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"),
            ),
        ];

        for (to_test, asserted) in tests {
            assert_eq!(to_test.blink(), asserted)
        }
    }

    #[test]
    fn blik_fast_from_stones() {
        let stones = Stones::new("0");
        let stones = stones.blink_fast();
        assert_eq!(stones.stones[0].number, "1");
    }

    #[test]
    fn blink_fast_stones() {
        let tests = vec![
            (Stones::new("0"), Stones::new("1")),
            (Stones::new("1"), Stones::new("2024")),
            (Stones::new("11"), Stones::new("1 1")),
            (
                Stones::new("0 1 10 99 999"),
                Stones::new("1 2024 1 0 9 9 2021976"),
            ),
            (Stones::new("125 17"), Stones::new("253000 1 7")),
            (Stones::new("253000 1 7"), Stones::new("253 0 2024 14168")),
            (
                Stones::new("253 0 2024 14168"),
                Stones::new("512072 1 20 24 28676032"),
            ),
            (
                Stones::new("512072 1 20 24 28676032"),
                Stones::new("512 72 2024 2 0 2 4 2867 6032"),
            ),
            (
                Stones::new("512 72 2024 2 0 2 4 2867 6032"),
                Stones::new("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
            ),
            (
                Stones::new("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
                Stones::new("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"),
            ),
        ];

        for (to_test, asserted) in tests {
            assert_eq!(to_test.blink(), asserted)
        }
    }
}
