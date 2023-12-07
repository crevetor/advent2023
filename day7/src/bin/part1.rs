use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
enum HandType {
    Five = 8,
    Four = 7,
    Full = 6,
    Three = 5,
    TwoPair = 4,
    OnePair = 3,
    HighCard = 2,
    Unknown = 1,
}

impl From<Vec<usize>> for HandType {
    fn from(vals: Vec<usize>) -> HandType {
        let max = vals.iter().max().unwrap();
        match max {
            1 => HandType::HighCard,
            2 => {
                if vals.iter().filter(|x| **x == 2).count() == 1 {
                    HandType::OnePair
                } else {
                    HandType::TwoPair
                }
            }
            3 => {
                if vals.contains(&2) {
                    HandType::Full
                } else {
                    HandType::Three
                }
            }
            4 => HandType::Four,
            5 => HandType::Five,
            _ => HandType::Unknown,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
    handtype: HandType,
}

#[derive(Debug)]
struct HandErr;

impl FromStr for Hand {
    type Err = HandErr;

    fn from_str(s: &str) -> Result<Hand, HandErr> {
        let (cards, bid) = s.split_once(" ").unwrap();
        let mut hand = Hand {
            cards: Vec::new(),
            bid: bid.parse::<usize>().unwrap(),
            handtype: HandType::Unknown,
        };
        let mut counts: HashMap<u8, usize> = HashMap::new();
        for c in cards.chars() {
            let card = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '0'..='9' => c.to_digit(10).unwrap().try_into().unwrap(),
                _ => return Err(HandErr),
            };
            if let Some(val) = counts.get_mut(&card) {
                *val += 1;
            } else {
                counts.insert(card, 1);
            }
            hand.cards.push(card);
        }
        hand.handtype = HandType::from(counts.values().cloned().collect::<Vec<usize>>());
        Ok(hand)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.handtype == other.handtype {
            for i in 0..self.cards.len() {
                if self.cards[i] == other.cards[i] {
                    continue;
                }
                return self.cards[i].cmp(&other.cards[i]);
            }
        }
        self.handtype.cmp(&other.handtype)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.handtype == other.handtype
    }
}
impl Eq for Hand {}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn parse_input(contents: &Vec<String>) -> Vec<Hand> {
    contents
        .iter()
        .map(|x| Hand::from_str(x).unwrap())
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut hands = parse_input(&contents);

    hands.sort();
    println!("{:?}", hands);

    let mut total: usize = 0;
    for i in 1..=hands.len() {
        total += hands[i - 1].bid * i;
    }
    println!("{}", total);
}
