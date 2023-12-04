use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    mine: HashSet<usize>,
    overlap: Vec<usize>,
}

impl Card {
    fn new(id: usize, winning: HashSet<usize>, mine: HashSet<usize>) -> Self {
        let overlap = winning.intersection(&mine).cloned().collect();

        Card {
            id,
            winning,
            mine,
            overlap,
        }
    }

    fn instances(self: &Self, cards: &HashMap<usize, Card>) -> usize {
        let mut ret: usize = 1;
        for i in 1..=self.overlap.len() {
            if let Some(card) = cards.get(&(self.id + i)) {
                ret += card.instances(cards);
            }
        }
        ret
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn parse_input(lines: Vec<String>) -> HashMap<usize, Card> {
    let mut ret: HashMap<usize, Card> = HashMap::new();
    for line in lines {
        let (id, num) = line.split_once(": ").expect("Couldn't split on ':'");
        let (left, right) = num.split_once(" | ").expect("Couldn't split on '|'");
        let winning: HashSet<usize> = left
            .split(' ')
            .filter(|x| x.len() > 0 && x.chars().all(|c| c.is_digit(10)))
            .map(|x| {
                x.parse()
                    .expect(&format!("Couldn't parse a usize from {}", x))
            })
            .collect();
        let mine: HashSet<usize> = right
            .split(' ')
            .filter(|x| x.len() > 0 && x.chars().all(|c| c.is_digit(10)))
            .map(|x| {
                x.parse()
                    .expect(&format!("Couldn't parse a usize from {}", x))
            })
            .collect();
        let id = id
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .expect("Couldn't parse usize from card id string");
        ret.insert(id, Card::new(id, winning, mine));
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let cards = parse_input(contents);
    let mut instances: Vec<usize> = Vec::new();

    for (_, card) in &cards {
        instances.push(card.instances(&cards))
    }
    println!("{:?}", instances);
    println!("{}", instances.iter().sum::<usize>());
}
