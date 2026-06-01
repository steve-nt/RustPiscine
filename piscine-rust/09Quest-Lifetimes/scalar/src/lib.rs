pub trait Scalar {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_scalar {
    ($($t:ty, $z:expr, $o:expr);*) => {
        $(impl Scalar for $t {
            fn zero() -> Self { $z }
            fn one() -> Self { $o }
        })*
    };
}

impl_scalar!(
    u8, 0, 1;
    u16, 0, 1;
    u32, 0, 1;
    u64, 0, 1;
    i8, 0, 1;
    i16, 0, 1;
    i32, 0, 1;
    i64, 0, 1;
    f32, 0.0, 1.0;
    f64, 0.0, 1.0
);

pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}