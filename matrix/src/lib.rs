use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    contents: Vec<Vec<T>>,
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.contents {
            for col in row {
                write!(f, "{col}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: Clone> FromIterator<Vec<T>> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut ret = Matrix {
            contents: Vec::new(),
        };

        for elt in iter {
            ret.contents.push(elt);
        }

        ret
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(c: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            contents: c.clone(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if y >= self.contents.len() || x >= self.contents[0].len() {
            return None;
        }

        Some(self.contents[y][x].clone())
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if y >= self.contents.len() || x >= self.contents[0].len() {
            return None;
        }

        Some(&mut self.contents[y][x])
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.contents[y][x] = val;
    }

    pub fn num_rows(&self) -> usize {
        self.contents.len()
    }

    pub fn num_cols(&self) -> usize {
        self.contents[0].len()
    }

    pub fn row(&self, idx: usize) -> Option<Vec<T>> {
        if idx >= self.contents.len() {
            return None;
        }

        Some(self.contents[idx].clone())
    }

    pub fn col(&self, idx: usize) -> Option<Vec<T>> {
        if idx >= self.contents[0].len() {
            return None;
        }

        Some(self.contents.iter().map(|x| x[idx].clone()).collect())
    }

    pub fn insert_row(&mut self, idx: usize, content: Vec<T>) {
        self.contents.insert(idx, content.clone());
    }

    pub fn insert_col(&mut self, idx: usize, content: Vec<T>) {
        for (i, val) in content.iter().enumerate() {
            self.contents[i].insert(idx, val.clone());
        }
    }

    pub fn cols(&self) -> impl DoubleEndedIterator<Item = Vec<T>> + '_ {
        (0..self.contents[0].len()).map(|x| {
            self.contents
                .iter()
                .map(|row| row[x].clone())
                .collect::<Vec<T>>()
        })
    }

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = Vec<T>> + '_ {
        self.contents.iter().cloned()
    }
}
