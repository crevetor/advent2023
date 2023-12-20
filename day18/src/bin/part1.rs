use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Step {
    direction: char,
    distance: i32,
    color: String,
}

#[derive(Debug)]
struct StepToStrError;

impl FromStr for Step {
    type Err = StepToStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let direction = parts.next().unwrap().chars().nth(0).unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        let color = parts.next().unwrap();

        Ok(Step {
            direction,
            distance,
            color: color.to_string(),
        })
    }
}

impl Step {
    fn gen_coords(&self, initial: [i32; 2]) -> Vec<[i32; 2]> {
        let vel = match self.direction {
            'R' => [1, 0],
            'L' => [-1, 0],
            'U' => [0, -1],
            'D' => [0, 1],
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
