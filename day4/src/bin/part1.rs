use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn parse_input(lines: Vec<String>) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut ret: Vec<(HashSet<usize>, HashSet<usize>)> = Vec::new();
    for line in lines {
        let (_, num) = line.split_once(": ").expect("Couldn't split on ':'");
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
        ret.push((winning, mine));
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
    let numbers = parse_input(contents);
    let mut total: i64 = 0;
    for (winning, mine) in numbers {
        let overlap: Vec<usize> = winning.intersection(&mine).cloned().collect();
        if overlap.len() > 0 {
            total += 2_i64.pow((overlap.len() - 1).try_into().unwrap());
        }
    }
    println!("{}", total);
}
