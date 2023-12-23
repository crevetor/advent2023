use matrix::Matrix;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

trait Find {
    fn find(&self, needle: char) -> Option<[usize; 2]>;
}

impl Find for Matrix<char> {
    fn find(&self, needle: char) -> Option<[usize; 2]> {
        for (y, row) in self.rows().enumerate() {
            if let Some(x) = row.iter().position(|x| x == &needle) {
                return Some([x, y]);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Path {
    len: usize,
    head: [usize; 2],
}

impl Path {
    fn new(head: [usize; 2]) -> Self {
        Path { len: 0, head }
    }

    fn next_steps(&self, garden: &Matrix<char>) -> Vec<Path> {
        let mut ret = Vec::new();
        if self.head[0] > 0 && garden.get(self.head[0] - 1, self.head[1]) != Some('#') {
            ret.push(Path {
                len: self.len + 1,
                head: [self.head[0] - 1, self.head[1]],
            });
        }

        if self.head[0] < garden.num_cols() - 1
            && garden.get(self.head[0] + 1, self.head[1]) != Some('#')
        {
            ret.push(Path {
                len: self.len + 1,
                head: [self.head[0] + 1, self.head[1]],
            });
        }

        if self.head[1] > 0 && garden.get(self.head[0], self.head[1] - 1) != Some('#') {
            ret.push(Path {
                len: self.len + 1,
                head: [self.head[0], self.head[1] - 1],
            });
        }

        if self.head[1] < garden.num_rows() - 1
            && garden.get(self.head[0], self.head[1] + 1) != Some('#')
        {
            ret.push(Path {
                len: self.len + 1,
                head: [self.head[0], self.head[1] + 1],
            });
        }
        ret
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
    let mut heads = HashSet::from([Path::new(starting_point)]);
    let steps = 64;
    for _ in 0..steps {
        let mut new_heads = HashSet::new();
        for head in heads.iter() {
            new_heads.extend(head.next_steps(&garden));
        }
        heads = new_heads;
    }

    println!("{}", heads.len());
}
