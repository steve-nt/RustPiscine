use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        if value == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut digits = Vec::new();

        // We use slices &[RomanDigit] for the numeral mappings to avoid 
        // allocating vectors inside the array definition.
        let conversions: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900,  &[C, M]),
            (500,  &[D]),
            (400,  &[C, D]),
            (100,  &[C]),
            (90,   &[X, C]),
            (50,   &[L]),
            (40,   &[X, L]),
            (10,   &[X]),
            (9,    &[I, X]),
            (5,    &[V]),
            (4,    &[I, V]),
            (1,    &[I]),
        ];

        // Greedily subtract the largest possible values
        for (val, numeral) in conversions.iter() {
            while value >= *val {
                value -= *val;
                digits.extend_from_slice(numeral);
            }
        }

        RomanNumber(digits)
    }
}