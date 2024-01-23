use matrix::Matrix;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Path {
    head: [usize; 2],
    cost: u32,
    estimated_cost: u32,
    path: Vec<[usize; 2]>,
    limits: [usize; 2],
}

impl Path {
    fn new(head: [usize; 2], cost: u32, limits: [usize; 2]) -> Self {
        Path {
            head,
            cost,
            estimated_cost: cost + u32::try_from(limits[0] - 1 + limits[1] - 1).unwrap(),
            path: Vec::new(),
            limits,
        }
    }

    fn current_direction(&self) -> Direction {
        if let Some(last_pos) = self.path.iter().rev().nth(0) {
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

    fn next_in_same_direction(&self) -> Option<[usize; 2]> {
        match self.current_direction() {
            Direction::North => {
                if self.head[1] > 0 {
                    Some([self.head[0], self.head[1] - 1])
                } else {
                    None
                }
            }
            Direction::South => Some([self.head[0], self.head[1] + 1]),
            Direction::East => Some([self.head[0] + 1, self.head[1]]),
            Direction::West => {
                if self.head[0] > 0 {
                    Some([self.head[0] - 1, self.head[1]])
                } else {
                    None
                }
            }
            Direction::Unknown => None,
        }
    }

    fn should_turn(&self) -> bool {
        self.path.len() >= 3
            && (self.path.iter().rev().take(3).all(|x| x[0] == self.head[0])
                || self.path.iter().rev().take(3).all(|x| x[1] == self.head[1]))
    }

    fn next(&self, map: &Matrix<u32>) -> Vec<Path> {
        let neighbors = map.get_neighbors(self.head[0], self.head[1]);
        neighbors
            .iter()
            .filter(|(pos, val)| {
                *pos != *self.path.iter().last().unwrap_or(&[0, 0])
                    && !(self.should_turn() && Some(*pos) == self.next_in_same_direction())
            })
            .map(|(pos, val)| {
                let mut new = self.clone();
                new.head = *pos;
                new.cost = self.cost + *val;
                new.estimated_cost = new.cost
                    + u32::try_from(self.limits[0] - self.head[0] + self.limits[1] - self.head[1])
                        .unwrap();
                new.path.push(self.head);
                new
            })
            .collect()
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.head[0].cmp(&self.head[0]))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Matrix<u32>, start: [usize; 2], goal: [usize; 2]) -> Vec<Path> {
    let mut dist: HashMap<[usize; 2], u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut cheapest: Vec<Path> = Vec::new();

    dist.insert(start, 0);
    heap.push(Path::new(
        start,
        0,
        [map.num_cols() - 1, map.num_rows() - 1],
    ));

    while let Some(curpath) = heap.pop() {
        if curpath.head == goal {
            cheapest.push(curpath.clone());
        }

        if curpath.cost > *dist.get(&curpath.head).unwrap_or(&u32::MAX) {
            continue;
        }

        for next in curpath.next(&map) {
            if next.cost <= *dist.get(&next.head).unwrap_or(&(u32::MAX - 1)) + 1 {
                heap.push(next.clone());
                dist.insert(next.head, next.cost);
            }
        }
    }

    cheapest
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

    println!("{}", map.get(0, 0).unwrap());
    let shortest = shortest_path(&map, [0, 0], [map.num_cols() - 1, map.num_rows() - 1]);
    for path in shortest.iter() {
        println!("{path:?}");
    }
    println!("{}", shortest.iter().map(|x| x.cost).min().unwrap());
}
