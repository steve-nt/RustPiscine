use std::ops::{Add, Sub, Mul, Div};

// 1. SCALAR TRAIT 

pub trait Scalar: 
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar {
    ($t:ty, $zero:expr, $one:expr) => {
        impl Scalar for $t {
            type Item = $t;

            fn zero() -> Self::Item {
                $zero
            }

            fn one() -> Self::Item {
                $one
            }
        }
    };
}

impl_scalar!(u32, 0, 1);
impl_scalar!(u64, 0, 1);
impl_scalar!(i32, 0, 1);
impl_scalar!(i64, 0, 1);
impl_scalar!(f32, 0.0, 1.0);
impl_scalar!(f64, 0.0, 1.0);

// 2. VECTOR LOGIC (Current exercise)

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        // Return None if vectors are of different lengths
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let result: Vec<T> = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        // Return None if vectors are of different lengths
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut iter = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a * b);

        let mut sum = iter.next()?;

        for val in iter {
            sum = sum + val;
        }

        Some(sum)
    }
}