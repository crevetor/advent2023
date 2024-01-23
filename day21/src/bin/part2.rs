use matrix::Matrix;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Path {
    len: usize,
    head: [i32; 2],
}

impl Path {
    fn new(head: [i32; 2]) -> Self {
        Path { len: 0, head }
    }

    fn next_steps(&self, garden: &Matrix<char>) -> Vec<Path> {
        garden
            .get_neighbors_wraparound(self.head[0], self.head[1])
            .iter()
            .filter(|x| x.1 != '#')
            .map(|x| {
                let mut new = self.clone();
                new.head = x.0;
                new.len += 1;
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
    let garden = Matrix::from_iter(contents.iter().map(|x| x.chars().collect()));
    let starting_point = garden.find('S').unwrap();
    let mut heads = HashSet::from([Path::new([
        i32::try_from(starting_point[0]).unwrap(),
        i32::try_from(starting_point[1]).unwrap(),
    ])]);
    //let steps = 26501365;
    let steps = 50;
    for _ in 0..steps {
        let mut new_heads = HashSet::new();
        for head in heads.iter() {
            new_heads.extend(head.next_steps(&garden));
        }
        heads = new_heads;
    }

    println!("{}", heads.len());
}
