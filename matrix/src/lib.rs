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

impl<T: Clone + PartialEq> Matrix<T> {
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

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<([usize; 2], T)> {
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

    pub fn get_neighbors_wraparound(&self, x: i32, y: i32) -> Vec<([i32; 2], T)> {
        let coords = [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]];
        let mut ret = Vec::new();
        if x == -10 {
            println!("");
        }
        for coord in coords {
            let mut inside_coord = coord;
            if coord[0] <= -i32::try_from(self.num_cols()).unwrap() {
                inside_coord[0] = i32::try_from(self.num_cols()).unwrap() - 1
                    + (coord[0] % (i32::try_from(self.num_cols()).unwrap()));
            } else if coord[0] < 0 {
                inside_coord[0] = i32::try_from(self.num_cols()).unwrap() + coord[0];
            } else if coord[0] >= self.num_cols().try_into().unwrap() {
                inside_coord[0] = (coord[0]) % (i32::try_from(self.num_cols()).unwrap());
            }

            if coord[1] <= -i32::try_from(self.num_rows()).unwrap() {
                inside_coord[1] = (i32::try_from(self.num_rows()).unwrap()) - 1
                    + (coord[1] % (i32::try_from(self.num_rows()).unwrap()));
            } else if coord[1] < 0 {
                inside_coord[1] = i32::try_from(self.num_cols()).unwrap() + coord[1];
            } else if coord[1] >= self.num_rows().try_into().unwrap() {
                inside_coord[1] = (coord[1]) % (i32::try_from(self.num_rows()).unwrap());
            }
            ret.push((
                coord,
                self.get(
                    inside_coord[0].try_into().unwrap(),
                    inside_coord[1].try_into().unwrap(),
                )
                .unwrap(),
            ));
        }
        ret
    }

    pub fn find(&self, needle: T) -> Option<[usize; 2]> {
        for (y, row) in self.rows().enumerate() {
            if let Some(x) = row.iter().position(|x| x == &needle) {
                return Some([x, y]);
            }
        }
        None
    }
}
