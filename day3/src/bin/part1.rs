use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum EltType {
    Symbol(char),
    Number(usize),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Elt {
    val: EltType,
    coords: (usize, usize),
}

impl Elt {
    fn get_num(self: Self) -> usize {
        if let EltType::Number(n) = self.val {
            return n;
        }
        0
    }
}

fn parse_input(lines: Vec<String>) -> HashMap<(usize, usize), Elt> {
    let mut elts: HashMap<(usize, usize), Elt> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let mut cur_str = String::new();
        for (x, cur_char) in line.chars().enumerate() {
            if cur_char.is_digit(10) {
                cur_str.push(cur_char);
            } else {
                if cur_str.len() > 0 {
                    for offset in 1..cur_str.len() + 1 {
                        elts.insert(
                            (x - offset, y),
                            Elt {
                                val: EltType::Number(cur_str.parse().unwrap()),
                                coords: (x - cur_str.len(), y),
                            },
                        );
                    }
                    cur_str = String::new();
                }
                if cur_char != '.' {
                    elts.insert(
                        (x, y),
                        Elt {
                            val: EltType::Symbol(cur_char),
                            coords: (x, y),
                        },
                    );
                }
            }
        }
        if cur_str.len() > 0 {
            for offset in 1..cur_str.len() + 1 {
                elts.insert(
                    (line.len() - offset, y),
                    Elt {
                        val: EltType::Number(cur_str.parse().unwrap()),
                        coords: (line.len() - cur_str.len(), y),
                    },
                );
            }
        }
    }

    elts
}

fn find_neighbouring_numbers(
    x: usize,
    y: usize,
    elts: &HashMap<(usize, usize), Elt>,
) -> HashSet<Elt> {
    let mut nums: HashSet<Elt> = HashSet::new();
    let min_x = if x == 0 { 0 } else { x - 1 };
    let min_y = if y == 0 { 0 } else { y - 1 };
    for my_x in min_x..=x + 1 {
        for my_y in min_y..=y + 1 {
            if let Some(elt) = elts.get(&(my_x, my_y)) {
                if let EltType::Number(_) = elt.val {
                    nums.insert(*elt);
                }
            }
        }
    }
    nums
}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
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
    let elts = parse_input(contents);
    let mut sorted_keys: Vec<(usize, usize)> = elts.keys().cloned().collect();
    sorted_keys.sort_by(|a, b| {
        let y_cmp = a.1.cmp(&b.1);
        if y_cmp != Ordering::Equal {
            return y_cmp;
        } else {
            return a.0.cmp(&b.0);
        }
    });

    for key in sorted_keys {
        println!("{:?} {:?}", key, elts.get(&key));
    }

    let mut nums: HashSet<Elt> = HashSet::new();
    for (coords, elt) in &elts {
        match elt.val {
            EltType::Symbol(_) => {
                nums = nums
                    .union(&find_neighbouring_numbers(coords.0, coords.1, &elts))
                    .cloned()
                    .collect()
            }
            _ => (),
        }
    }
    println!("{:?}", nums);
    println!("{}", nums.iter().map(|x| x.get_num()).sum::<usize>());
}
