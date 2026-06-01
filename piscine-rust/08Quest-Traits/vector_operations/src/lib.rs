use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

// Implement Addition for ThreeDVector
impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        ThreeDVector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

// Implement Subtraction for ThreeDVector
impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        ThreeDVector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}