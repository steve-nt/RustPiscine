use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        todo!()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/*
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        // Iterate over the slice of slices and convert each inner slice to a Vec<i32>
        let matrix_vec = slice.iter().map(|row| row.to_vec()).collect();
        Matrix(matrix_vec)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            write!(f, "(")?;
            for (j, val) in row.iter().enumerate() {
                write!(f, "{}", val)?;
                // Print a space after every element except the last one in the row
                if j < row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, ")")?;
            
            // Add a newline after every row except the very last one
            if i < self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
*/