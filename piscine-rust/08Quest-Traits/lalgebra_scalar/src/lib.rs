use std::ops::{Add, Div, Mul, Sub};

// We define the supertraits that any `Scalar` must also implement.
// `<Output = Self>` ensures that adding two scalars returns the same scalar type.
pub trait Scalar: 
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized 
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

// A macro to easily implement the Scalar trait for multiple numeric types
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

// Implement Scalar for the requested integer types
impl_scalar!(u32, 0, 1);
impl_scalar!(u64, 0, 1);
impl_scalar!(i32, 0, 1);
impl_scalar!(i64, 0, 1);

// Implement Scalar for the requested floating-point types
impl_scalar!(f32, 0.0, 1.0);
impl_scalar!(f64, 0.0, 1.0);