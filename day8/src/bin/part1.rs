use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Left,
    Right,
    Unknown,
}

#[derive(Debug)]
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

    let mut node = nodes.get(&"AAA".to_string()).unwrap();
    let mut directions_iter = directions.iter().cycle();
    let mut num_steps = 0;
    while node.id != "ZZZ" {
        let nextnode_id = node.neighbors.get(directions_iter.next().unwrap()).unwrap();
        node = nodes.get(nextnode_id).unwrap();
        num_steps += 1;
    }

    println!("{}", num_steps);
}
