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

fn parse_input(contents: &Vec<String>) -> Vec<Race> {
    let mut ret = Vec::new();
    let mut iter = contents.iter();

    let times: Vec<usize> = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = iter
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    if times.len() != distances.len() {
        panic!("Times and distance aren't the same size");
    }

    for i in 0..times.len() {
        ret.push(Race::new(times[i], distances[i]));
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
    let races = parse_input(&contents);
    println!("{:?}", races);

    let margins = races.iter().map(|x| x.num_winning());
    println!("{:?}", margins.clone().collect::<Vec<usize>>());
    println!("{}", margins.product::<usize>());
}
