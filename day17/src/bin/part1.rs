use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

#[derive(Debug, Clone)]
struct Path {
    head: [usize; 2],
    cost: u32,
    path: Vec<[usize; 2]>,
    limits: [usize; 2],
}

impl Path {
    fn new(head: [usize; 2], limits: [usize; 2]) -> Self {
        Path {
            head,
            cost: 0,
            path: Vec::new(),
            limits,
        }
    }

    fn current_direction(&self) -> Direction {
        if let Some(last_pos) = self.path.iter().rev().nth(1) {
            if self.head[0] == last_pos[0] {
                if self.head[1] < last_pos[1] {
                    return Direction::North;
                }
                if self.head[1] > last_pos[1] {
                    return Direction::South;
                }
            }

            if self.head[1] == last_pos[1] {
                if self.head[0] < last_pos[0] {
                    return Direction::West;
                }
                if self.head[0] > last_pos[0] {
                    return Direction::East;
                }
            }
        }
        Direction::Unknown
    }

    fn next_pos(&self) -> Vec<[usize; 2]> {
        let mut ret = Vec::new();
        let same_dir = self.path.iter().rev().take(2).count() == 2
            && (self.path.iter().rev().take(2).all(|x| x[0] == self.head[0])
                || self.path.iter().rev().take(2).all(|x| x[1] == self.head[1]));

        if self.head[0] > 0 && !(same_dir && self.current_direction() == Direction::West) {
            ret.push([self.head[0] - 1, self.head[1]]);
        }

        if self.head[0] < self.limits[0]
            && !(same_dir && self.current_direction() == Direction::East)
        {
            ret.push([self.head[0] + 1, self.head[1]])
        }

        if self.head[1] > 0 && !(same_dir && self.current_direction() == Direction::North) {
            ret.push([self.head[0], self.head[1] - 1]);
        }

        if self.head[1] < self.limits[1]
            && !(same_dir && self.current_direction() == Direction::South)
        {
            ret.push([self.head[0], self.head[1] + 1]);
        }

        if let Some(last_pos) = self.path.iter().rev().nth(1) {
            ret = ret.iter().filter(|x| x != &last_pos).cloned().collect();
        }

        ret
    }

    fn step(&mut self, map: &Matrix<u32>) -> Vec<Self> {
        self.cost += map.get(self.head[0], self.head[1]).unwrap();
        self.path.push(self.head.clone());
        self.next_pos()
            .iter()
            .map(|x| {
                let mut new = self.clone();
                new.head = *x;
                new
            })
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let map = Matrix::from_iter(
        contents
            .iter()
            .map(|a| a.chars().map(|c| c.to_digit(10).unwrap()).collect()),
    );

    let mut paths = vec![Path::new([0, 0], [map.num_cols() - 1, map.num_rows() - 1])];
    let mut i = 0;
    while !paths
        .iter()
        .all(|x| x.head == [map.num_cols() - 1, map.num_rows() - 1])
    {
        let mut newpaths = Vec::new();
        for path in paths.iter_mut() {
            newpaths.append(&mut path.step(&map));
        }
        paths = newpaths;

        i += 1;
        if i == 3 {
            //break;
        }
    }
    println!("{:?}", paths.iter().map(|x| x.cost).min().unwrap(),);
}
