use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

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
                '.' => true,
                '^' => x.0[1] + 1 == self.head[1],
                '>' => x.0[0] == self.head[0] + 1,
                '<' => x.0[0] + 1 == self.head[0],
                'v' => x.0[1] == self.head[1] + 1,
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
        path: Vec::new(),
    }];
    while !paths
        .iter()
        .all(|x| x.head[1] == (map.num_cols() - 1).try_into().unwrap())
    {
        let mut newpaths = Vec::new();
        for path in paths.iter_mut() {
            if let Ok(mut ret) = path.step(&map) {
                newpaths.append(&mut ret);
            }
        }
        paths = newpaths;
    }

    println!("{paths:?}");
}
