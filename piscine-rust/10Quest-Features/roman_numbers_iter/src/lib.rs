#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanDigit {
    pub fn value(self) -> u32 {
        match self {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        use RomanDigit::*;
        let mut digits = Vec::new();
        
        // Added subtraction rules (e.g., 900 = CM, 4 = IV)
        let values = [
            (1000, vec![M]),
            (900, vec![C, M]),
            (500, vec![D]),
            (400, vec![C, D]),
            (100, vec![C]),
            (90, vec![X, C]),
            (50, vec![L]),
            (40, vec![X, L]),
            (10, vec![X]),
            (9, vec![I, X]),
            (5, vec![V]),
            (4, vec![I, V]),
            (1, vec![I]),
        ];

        for (val, rep) in &values {
            while n >= *val {
                digits.extend(rep.iter().cloned());
                n -= val;
            }
        }
        RomanNumber(digits)
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        let mut sum = 0;
        let mut prev = 0;
        
        // Iterate backwards to easily apply subtraction rules
        for digit in self.0.iter().rev() {
            let val = digit.value();
            if val < prev {
                sum -= val; // e.g., 'I' comes before 'V'
            } else {
                sum += val;
            }
            prev = val;
        }
        sum
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next numerical value, update self, and return
        let next_val = self.to_u32().checked_add(1)?;
        *self = RomanNumber::from(next_val);
        Some(self.clone())
    }
}