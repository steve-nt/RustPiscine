use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T: Copy + Default + Add<Output = T> + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_a = self.0.len();
        let cols_a = self.0.first().map_or(0, |r| r.len());
        let rows_b = rhs.0.len();
        let cols_b = rhs.0.first().map_or(0, |r| r.len());

        if cols_a != rows_b {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_b]; rows_a];
        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result[i][j] = result[i][j] + self.0[i][k] * rhs.0[k][j];
                }
            }
        }
        Some(Matrix(result))
    }
}
