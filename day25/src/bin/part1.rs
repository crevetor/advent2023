use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Index;
use std::process;

#[derive(Debug, Clone)]
struct Connection {
    start: String,
    end: String,
}

impl Connection {
    fn new(start: &str, end: &str) -> Self {
        Connection {
            start: start.to_string(),
            end: end.to_string(),
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}
impl Eq for Connection {}

#[derive(Hash, Debug, Clone)]
struct Node {
    name: String,
    connections: Vec<String>,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Node {}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            connections: Vec::new(),
        }
    }

    fn add_connection(&mut self, connection: &str) {
        if !self.connections.contains(&connection.to_string()) {
            self.connections.push(connection.to_string());
        }
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

fn parse_input(contents: &Vec<String>) -> Vec<Connection> {
    let mut ret = Vec::new();
    for line in contents {
        let (source, connections) = line.split_once(": ").unwrap();
        for connection in connections.split(" ") {
            ret.push(Connection::new(source, connection));
        }
    }
    ret
}

fn print_graph(connections: &Vec<Connection>) {
    let mut out = File::create("graph.dot").unwrap();
    out.write(b"graph connections {").unwrap();
    for connection in connections {
        write!(out, "{} -- {};", connection.start, connection.end);
    }
    out.write(b"}");
}

fn nodes_from_connections(connections: &Vec<Connection>) -> HashMap<String, Node> {
    let mut ret: HashMap<String, Node> = HashMap::new();

    for connection in connections {
        for (start, end) in [
            (connection.start.clone(), connection.end.clone()),
            (connection.end.clone(), connection.start.clone()),
        ] {
            if let Some(node) = ret.get_mut(&start) {
                node.add_connection(&end);
            } else {
                let mut node = Node::new(&start);
                node.add_connection(&end);
                ret.insert(start.clone(), node);
            }
        }
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
    let mut connections = parse_input(&contents);
    println!("{connections:?}");
    for i in [
        Connection::new("rhh", "mtc"),
        Connection::new("njn", "xtx"),
        Connection::new("gpj", "tmb"),
    ] {
        if let Some(idx) = connections.iter().position(|x| x == &i) {
            connections.remove(idx);
        } else {
            println!("Didn't find {i:?}");
        }
    }

    let mut nodes: HashMap<String, Node> = nodes_from_connections(&connections);
    let mut visited: HashSet<Node> = HashSet::new();
    let mut curnodes: HashSet<Node> = HashSet::new();
    curnodes.insert(nodes.get("gpj").unwrap().clone());

    while curnodes.difference(&visited).count() != 0 {
        let mut newcur: HashSet<Node> = HashSet::new();
        for node in curnodes {
            visited.insert(node.clone());
            newcur.extend(
                node.connections
                    .iter()
                    .map(|x| nodes.get(x).unwrap().clone()),
            );
        }
        curnodes = newcur;
    }

    println!("{}", visited.len());
    println!("{}", nodes.len() - visited.len());
    println!("{}", visited.len() * (nodes.len() - visited.len()));
}
