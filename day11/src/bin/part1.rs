use std::env;
use std::fmt;
use std::fs;
use std::process;

#[derive(Debug, Clone)]
struct Matrix<T> {
    contents: Vec<Vec<T>>,
}

impl<T: fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.contents {
            for col in row {
                write!(f, "{col:?}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Clone> Matrix<T> {
    fn new(c: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            contents: c.clone(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<T> {
        if y >= self.contents.len() || x >= self.contents[0].len() {
            return None;
        }

        Some(self.contents[y][x].clone())
    }

    fn num_rows(&self) -> usize {
        self.contents.len()
    }

    fn num_cols(&self) -> usize {
        self.contents[0].len()
    }

    fn row(&self, idx: usize) -> Option<Vec<T>> {
        if idx >= self.contents.len() {
            return None;
        }

        Some(self.contents[idx].clone())
    }

    fn col(&self, idx: usize) -> Option<Vec<T>> {
        if idx >= self.contents[0].len() {
            return None;
        }

        Some(self.contents.iter().map(|x| x[idx].clone()).collect())
    }

    fn insert_row(&mut self, idx: usize, content: Vec<T>) {
        self.contents.insert(idx, content.clone());
    }

    fn insert_col(&mut self, idx: usize, content: Vec<T>) {
        for (i, val) in content.iter().enumerate() {
            self.contents[i].insert(idx, val.clone());
        }
    }

    fn cols(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.contents[0].len()).map(|x| {
            self.contents
                .iter()
                .map(|row| row[x].clone())
                .collect::<Vec<T>>()
        })
    }

    fn rows(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        self.contents.iter().cloned()
    }
}

impl Matrix<char> {
    fn galaxy_positions(&self) -> Vec<[usize; 2]> {
        let mut ret = Vec::new();
        for (y, row) in self.contents.iter().enumerate() {
            for (x, elt) in row.iter().enumerate() {
                if elt == &'#' {
                    ret.push([x, y]);
                }
            }
        }

        ret
    }

    fn galaxy_pairs(&self) -> Vec<[[usize; 2]; 2]> {
        let mut ret = Vec::new();
        let pos = self.galaxy_positions();
        for first in &pos {
            for second in &pos {
                if first != second
                    && !ret.contains(&[*first, *second])
                    && !ret.contains(&[*second, *first])
                {
                    ret.push([*first, *second]);
                }
            }
        }

        ret
    }
}

fn distance(start: [usize; 2], end: [usize; 2]) -> usize {
    let start = [
        isize::try_from(start[0]).unwrap(),
        isize::try_from(start[1]).unwrap(),
    ];
    let end = [
        isize::try_from(end[0]).unwrap(),
        isize::try_from(end[1]).unwrap(),
    ];
    usize::try_from((end[0] - start[0]).abs() + (end[1] - start[1]).abs()).unwrap()
}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn parse_input(contents: &Vec<String>) -> Matrix<char> {
    Matrix::new(contents.iter().map(|x| x.chars().collect()).collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut matrix = parse_input(&contents);
    println!("{}", matrix);

    let mut empty_indices = Vec::new();
    for (i, row) in matrix.rows().enumerate() {
        if row.iter().all(|x| x == &'.') {
            empty_indices.push(i);
        }
    }

    for (i, idx) in empty_indices.iter().enumerate() {
        matrix.insert_row(*idx + i, (0..matrix.num_cols()).map(|_| '.').collect());
    }

    empty_indices.clear();
    for (i, col) in matrix.cols().enumerate() {
        if col.iter().all(|x| x == &'.') {
            empty_indices.push(i);
        }
    }

    for (i, idx) in empty_indices.iter().enumerate() {
        matrix.insert_col(*idx + i, (0..matrix.num_rows()).map(|_| '.').collect())
    }

    println!("{}", matrix);

    let pairs = matrix.galaxy_pairs();
    println!("{pairs:?}");
    println!("{}", pairs.len());
    let distances = pairs
        .iter()
        .map(|x| distance(x[0], x[1]))
        .collect::<Vec<usize>>();
    println!("{distances:?}");
    println!("{}", distances.iter().sum::<usize>());
}
