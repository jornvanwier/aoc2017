use std::ops::{Index, IndexMut};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T>
where
    T: Clone,
{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(length: usize, data: Vec<T>) -> Self {
        Self {
            rows: length,
            cols: length,
            data,
        }
    }

    pub fn from_2d_array(mut input: Vec<Vec<T>>) -> Self {
        let length = input.len();
        let data = input.iter_mut().fold(Vec::new(), |mut b, a| {
            b.append(a);
            b
        });
        Self::new(length, data)
    }

    fn flatten_index(&self, index: (usize, usize)) -> usize {
        let (x, y) = index;
        x + y * self.cols
    }

    pub fn flip(&self) -> Self {
        let mut clone = self.clone();
        for row in 0..self.rows {
            let from = row * self.cols;
            let length = self.cols;
            reverse_segment(from, length, &mut clone.data)
        }
        clone
    }

    pub fn symmetric(&self) -> Self {
        let mut new = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let a = self.flatten_index((col, row));
                let b = self.flatten_index((row, col));
                new.insert(a, self.data.get(b).unwrap().clone());
            }
        }

        Self::new(self.size(), new)
    }

    pub fn rotate_cw(&self) -> Self {
        self.clone().symmetric().flip()
    }

    pub fn size(&self) -> usize {
        assert_eq!(self.cols, self.rows);
        self.rows
    }

    pub fn divide(&mut self, size: usize) -> Matrix<Matrix<T>> {
        let new_size = self.size() / size;

        let mut data = Vec::new();
        for new_col in 0..new_size {
            for new_row in 0..new_size {
                // 🤔
                let col = new_col * size;
                let row = new_row * size;

                let mut inner_data = Vec::new();
                for inner_row in 0..size {
                    let start = self.flatten_index((col, row + inner_row));
                    let end = self.flatten_index((col + size, row + inner_row));
                    inner_data.append(&mut self.data[start..end].to_vec())
                }

                data.push(Matrix::new(size, inner_data));
            }
        }

        Matrix::<Matrix<T>>::new(new_size, data)
    }
}

impl<T> Matrix<Matrix<T>>
where
    T: Clone + fmt::Debug + PartialEq,
    Matrix<T>: Display,
{
    pub fn join(&mut self) -> Matrix<T> {
        let orig = self.clone();

        let sub_size = self.data[0].size();
        let size = self.data[0].size() * self.size();

        let mut data = Vec::new();
        for row in 0..size {
            for col in 0..size {
                let matrix = &self[(row / sub_size, col / sub_size)];
                // let matrix = &self[(col / sub_size, row / sub_size)];
                let sub_col = col % sub_size;
                let sub_row = row % sub_size;

                data.push(matrix[(sub_col, sub_row)].clone());
            }
        }

        let result = Matrix::new(size, data);

        // assert_eq!(orig, result.clone().divide(sub_size));

        result
    }
}

impl<T> Index<(usize, usize)> for Matrix<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.flatten_index(index)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.flatten_index(index);
        &mut self.data[index]
    }
}

impl Display for Matrix<bool> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", if self[(col, row)] { '#' } else { '.' })?;
            }
            writeln!(f, "")?;
        }

        write!(f, "")
    }
}

fn reverse_segment<T>(from: usize, length: usize, collection: &mut Vec<T>) {
    use std::num::Wrapping;
    let list_length = collection.len();
    let mut take_length = length;
    for position in from..from + take_length / 2 {
        let a = position % list_length;
        let b = (position + take_length - 1) % list_length;
        collection.swap(a, b);
        take_length = (Wrapping(take_length) - Wrapping(2)).0
    }
}
