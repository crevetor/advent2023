use std::env;
use std::fs;
use std::process;

#[derive(Debug, Copy, Clone)]
struct Race {
    time: usize,
    distance: usize,
    acceleration: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self {
            time,
            distance,
            acceleration: 1,
        }
    }

    fn possible_distances(self: &Self) -> Vec<usize> {
        let mut ret = Vec::new();
        for btn_time in 0..=self.time {
            let speed = btn_time * self.acceleration;
            let distance = speed * (self.time - btn_time);
            ret.push(distance);
        }

        ret
    }

    fn num_winning(self: &Self) -> usize {
        self.possible_distances()
            .iter()
            .filter(|x| **x > self.distance)
            .count()
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

fn parse_input(contents: &Vec<String>) -> Race {
    let mut iter = contents.iter();

    let time: usize = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance: usize = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Race::new(time, distance)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let race = parse_input(&contents);
    println!("{:?}", race);

    println!("{}", race.num_winning());
}
