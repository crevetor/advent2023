use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Step {
    direction: u32,
    distance: i32,
}

#[derive(Debug)]
struct StepToStrError;

impl FromStr for Step {
    type Err = StepToStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hex: Vec<char> = s
            .split(" ")
            .last()
            .unwrap()
            .chars()
            .filter(|x| x.is_ascii_hexdigit())
            .collect();
        let distance = i32::from_str_radix(&hex.iter().take(5).collect::<String>(), 16).unwrap();
        let direction = hex.iter().last().unwrap().to_digit(16).unwrap();

        Ok(Step {
            direction,
            distance,
        })
    }
}

impl Step {
    fn gen_coords(&self, initial: [i32; 2]) -> Vec<[i32; 2]> {
        let vel = match self.direction {
            0 => [1, 0],
            2 => [-1, 0],
            3 => [0, -1],
            1 => [0, 1],
            _ => panic!("Got an unexpected direction"),
        };
        (1..=self.distance)
            .map(|x| [initial[0] + x * vel[0], initial[1] + x * vel[1]])
            .collect()
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

fn gen_coords(n: &[i32; 2]) -> Vec<[i32; 2]> {
    vec![
        [n[0] - 1, n[1]],
        [n[0] + 1, n[1]],
        [n[0], n[1] - 1],
        [n[0], n[1] + 1],
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let steps: Vec<_> = contents
        .iter()
        .map(|x| Step::from_str(x).unwrap())
        .collect();
    println!("{steps:?}");
    let mut coords: Vec<[i32; 2]> = vec![[0, 0]];
    for step in steps {
        coords.append(&mut step.gen_coords(coords.iter().cloned().last().unwrap()));
    }

    let mut all_coords: HashSet<[i32; 2]> = HashSet::from_iter(coords.iter().cloned());
    let mut q = VecDeque::new();
    q.push_back([1, 1]);
    while !q.is_empty() {
        if let Some(n) = q.pop_front() {
            if !all_coords.contains(&n) {
                all_coords.insert(n.clone());
                q.append(&mut gen_coords(&n).into());
            }
        }
    }

    println!("{}", all_coords.iter().count());
}
