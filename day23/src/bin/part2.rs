use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

trait GetNeighbors {
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<([usize; 2], char)>;
}

impl GetNeighbors for Matrix<char> {
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<([usize; 2], char)> {
        let mut ret = Vec::new();
        if x > 0 {
            ret.push(([x - 1, y], self.get(x - 1, y).unwrap()));
        }
        if x < self.num_cols() - 1 {
            ret.push(([x + 1, y], self.get(x + 1, y).unwrap()));
        }
        if y > 0 {
            ret.push(([x, y - 1], self.get(x, y - 1).unwrap()));
        }
        if y < self.num_rows() - 1 {
            ret.push(([x, y + 1], self.get(x, y + 1).unwrap()));
        }
        ret
    }
}

#[derive(Clone, Debug)]
struct Path {
    head: [i32; 2],
    len: usize,
    path: Vec<[i32; 2]>,
}

struct NoMoreSteps;
impl Path {
    fn step(&mut self, map: &Matrix<char>) -> Result<Vec<Path>, NoMoreSteps> {
        let acceptable: Vec<[i32; 2]> = map
            .get_neighbors(
                self.head[0].try_into().unwrap(),
                self.head[1].try_into().unwrap(),
            )
            .iter()
            .map(|x| {
                (
                    [x.0[0].try_into().unwrap(), x.0[1].try_into().unwrap()],
                    x.1,
                )
            })
            .filter(|x| match x.1 {
                '#' => false,
                '.' | '^' | '>' | '<' | 'v' => true,
                _ => false,
            })
            .map(|x| x.0)
            .filter(|x| !self.path.contains(x))
            .collect();

        if acceptable.len() == 0 {
            return Err(NoMoreSteps);
        }

        Ok(acceptable
            .iter()
            .map(|x| {
                let mut new = self.clone();
                new.head = *x;
                new.len += 1;
                new.path.push(*x);
                new
            })
            .collect())
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
    let map = Matrix::from_iter(contents.iter().map(|x| x.chars().collect()));

    let mut paths = vec![Path {
        head: [1, 0],
        len: 0,
        path: vec![[1, 0]],
    }];

    while !paths
        .iter()
        .all(|x| x.head[1] == i32::try_from(map.num_rows() - 1).unwrap())
    {
        let mut newpaths: Vec<Path> = paths
            .iter()
            .filter(|x| x.head[1] == i32::try_from(map.num_rows() - 1).unwrap())
            .cloned()
            .collect();
        for path in paths
            .iter_mut()
            .filter(|x| x.head[1] != i32::try_from(map.num_rows() - 1).unwrap())
        {
            if let Ok(mut ret) = path.step(&map) {
                newpaths.append(&mut ret);
            } else if path.head[1] == i32::try_from(map.num_rows() - 1).unwrap() {
                newpaths.push(path.clone());
            }
        }
        paths = newpaths;
        println!("{}", paths.len());
    }

    println!("{paths:?}");
    println!("{}", paths.iter().map(|x| x.len).max().unwrap())
}
