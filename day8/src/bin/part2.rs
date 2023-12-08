use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    Left,
    Right,
    Unknown,
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    neighbors: HashMap<Direction, String>,
}

#[derive(Debug)]
struct NodeErr;

impl FromStr for Node {
    type Err = NodeErr;

    fn from_str(s: &str) -> Result<Self, NodeErr> {
        let (id, neighbors) = s.split_once(" = ").unwrap();
        let (left, right) = neighbors.split_once(", ").unwrap();
        Ok(Node {
            id: id.to_string(),
            neighbors: HashMap::from([
                (
                    Direction::Left,
                    left.chars().filter(|x| x.is_alphanumeric()).collect(),
                ),
                (
                    Direction::Right,
                    right.chars().filter(|x| x.is_alphanumeric()).collect(),
                ),
            ]),
        })
    }
}

impl Node {
    fn is_start(&self) -> bool {
        self.id.chars().last().unwrap() == 'A'
    }

    fn is_end(&self) -> bool {
        self.id.chars().last().unwrap() == 'Z'
    }
}

#[derive(Debug, Clone)]
struct Path {
    start: String,
    nodes: Vec<String>,
    path_loop: Option<(usize, usize)>,
}

impl Path {
    fn new(start: &str) -> Self {
        Path {
            start: start.to_string(),
            nodes: Vec::new(),
            path_loop: None,
        }
    }

    fn append(&mut self, node: &str) -> bool {
        if self.has_loop() {
            return false;
        }

        if node.ends_with('Z') && self.nodes.contains(&node.to_string()) {
            let pos = self
                .nodes
                .iter()
                .position(|x| x == &node.to_string())
                .unwrap();
            self.path_loop = Some((pos, self.nodes.len() - pos));
            false
        } else {
            self.nodes.push(node.to_string());
            true
        }
    }

    fn has_loop(&self) -> bool {
        self.path_loop != None
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

// Taken from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut iter = contents.iter();
    let directions: Vec<Direction> = iter
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::Unknown,
        })
        .collect();

    iter.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in iter {
        let node = Node::from_str(line).unwrap();
        nodes.insert(node.id.clone(), node);
    }
    println!("{:?}", nodes);

    let mut curnodes: Vec<Node> = nodes.values().filter(|x| x.is_start()).cloned().collect();
    let mut directions_iter = directions.iter().cycle();
    let mut paths: Vec<Path> = curnodes.iter().map(|x| Path::new(&x.id)).collect();

    while !paths.iter().all(|x| x.has_loop()) {
        let direction = directions_iter.next().unwrap();
        let nextnode_ids: Vec<String> = curnodes
            .iter()
            .map(|x| x.neighbors.get(direction).unwrap())
            .cloned()
            .collect();

        for (i, curnode) in curnodes.iter().enumerate() {
            paths[i].append(&nextnode_ids[i]);
        }
        curnodes = nextnode_ids
            .iter()
            .map(|x| nodes.get(x).unwrap())
            .cloned()
            .collect();
    }
    println!("---------------------------");

    for path in &paths {
        //println!("{:?}", path);
        println!("{} {:?}", path.start, path.path_loop);
    }

    let path_lengths: Vec<usize> = paths.iter().map(|x| x.path_loop.unwrap().1).collect();
    let lcm = lcm(&path_lengths);
    println!("{}", lcm);
}
