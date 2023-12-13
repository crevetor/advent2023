use std::env;
use std::fmt;
use std::fs;
use std::process;

#[derive(Debug, Eq, PartialEq)]
enum ReflectionType {
    Vertical,
    Horizontal,
}

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
    fn potential_reflections(&self) -> (Vec<usize>, Vec<usize>) {
        let mut col_refs = Vec::new();
        let mut row_refs = Vec::new();
        for x in 0..self.num_cols() - 1 {
            if self
                .compare_with_smudge(&self.col(x).unwrap(), &self.col(x + 1).unwrap())
                .0
            {
                col_refs.push(x);
            }
        }

        for y in 0..self.num_rows() - 1 {
            if self
                .compare_with_smudge(&self.row(y).unwrap(), &self.row(y + 1).unwrap())
                .0
            {
                row_refs.push(y);
            }
        }
        (col_refs, row_refs)
    }

    fn reflection(&self, idx: usize, reflection: ReflectionType) -> Option<usize> {
        let (left, right, endidx) = match reflection {
            ReflectionType::Horizontal => (
                self.rows().collect::<Vec<Vec<char>>>(),
                self.rows().collect::<Vec<Vec<char>>>(),
                self.num_rows() - 1,
            ),
            ReflectionType::Vertical => (
                self.cols().collect::<Vec<Vec<char>>>(),
                self.cols().collect::<Vec<Vec<char>>>(),
                self.num_cols() - 1,
            ),
        };
        let mut left_idx = idx;
        let mut right_idx = idx + 1;
        let mut smudge_used = false;

        loop {
            let (same, smudge) = self.compare_with_smudge(&left[left_idx], &right[right_idx]);
            if !same || (same && smudge && smudge_used) {
                return None;
            }

            if same && smudge {
                smudge_used = true;
            }

            if left_idx == 0 || right_idx == endidx {
                break;
            }

            left_idx -= 1;
            right_idx += 1;
        }

        if !smudge_used {
            return None;
        }

        match reflection {
            ReflectionType::Horizontal => Some(100 * (idx + 1)),
            ReflectionType::Vertical => Some(idx + 1),
        }
    }

    fn summary(&self) -> usize {
        let (vert, horiz) = self.potential_reflections();
        let vert_ref: Vec<usize> = vert
            .iter()
            .filter_map(|x| self.reflection(*x, ReflectionType::Vertical))
            .collect();
        let horiz_ref: Vec<usize> = horiz
            .iter()
            .filter_map(|x| self.reflection(*x, ReflectionType::Horizontal))
            .collect();

        if vert_ref.len() == 1 {
            vert_ref[0]
        } else if horiz_ref.len() == 1 {
            horiz_ref[0]
        } else {
            panic!("WTF");
        }
    }

    fn compare_with_smudge(&self, left: &Vec<char>, right: &Vec<char>) -> (bool, bool) {
        let mut smudge_used = false;
        for (l, r) in left.iter().zip(right.iter()) {
            if l != r && smudge_used {
                return (false, false);
            }
            if l != r {
                smudge_used = true;
            }
        }
        (true, smudge_used)
    }
}

fn parse_input(contents: &Vec<String>) -> Vec<Matrix<char>> {
    contents
        .split(|x| x.is_empty())
        .map(|x| Matrix::new(x.iter().map(|x| x.chars().collect()).collect()))
        .collect()
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
    let matrices = parse_input(&contents);
    //println!("{matrices:?}");

    let vals: Vec<usize> = matrices.iter().map(|x| x.summary()).collect();
    println!("{vals:?}");
    println!("{}", vals.iter().sum::<usize>());
}
