use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

// Enables parsing from a string, e.g., "A+".parse::<BloodType>()
impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A+" => Ok(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive }),
            "A-" => Ok(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative }),
            "B+" => Ok(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive }),
            "B-" => Ok(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative }),
            "AB+" => Ok(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive }),
            "AB-" => Ok(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative }),
            "O+" => Ok(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive }),
            "O-" => Ok(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative }),
            _ => Err(()),
        }
    }
}

// Enables {:?} formatting as "A+", "B-", etc.
impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.antigen {
            _ if self.rh_factor == RhFactor::Positive => "+",
            _ => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    // Determines if `self` can receive blood from `other`
    pub fn can_receive_from(self, other: Self) -> bool {
        // Rh Compatibility:
        // Receiver (self) is Positive -> can receive from Positive or Negative.
        // Donor (other) is Negative -> can give to Positive or Negative.
        let rh_compatible = self.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative;

        // Antigen Compatibility:
        // Receiver is AB -> can receive from anyone.
        // Donor is O -> can give to anyone.
        // Otherwise, antigens must match.
        let antigen_compatible = self.antigen == Antigen::AB || other.antigen == Antigen::O || self.antigen == other.antigen;

        rh_compatible && antigen_compatible
    }

    // Helper to return all 8 possible blood types
    fn all_types() -> Vec<BloodType> {
        vec![
            "A+".parse().unwrap(), "A-".parse().unwrap(),
            "B+".parse().unwrap(), "B-".parse().unwrap(),
            "AB+".parse().unwrap(), "AB-".parse().unwrap(),
            "O+".parse().unwrap(), "O-".parse().unwrap(),
        ]
    }

    // Returns all blood types that can give blood to `self`
    pub fn donors(self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|&donor| self.can_receive_from(donor))
            .collect()
    }

    // Returns all blood types that can receive blood from `self`
    pub fn recipients(self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|&recipient| recipient.can_receive_from(self))
            .collect()
    }
}