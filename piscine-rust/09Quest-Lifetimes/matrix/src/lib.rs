// Depending on your project's structure, you may need to import the Scalar trait.
pub(crate) use scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar + Copy> Matrix<W, H, T> {
    /// Returns a matrix of size W x H with all the positions filled by T::zero()
    pub fn zero() -> Self {
        Matrix([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar + Copy> Matrix<S, S, T> {
    /// Returns the identity matrix of size S x S
    pub fn identity() -> Self {
        let mut mat = [[T::zero(); S]; S];
        for i in 0..S {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}