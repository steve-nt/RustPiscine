/*
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    /// Returns true if self can receive blood from other blood type.
    pub fn can_receive_from(self, other: Self) -> bool {
        // Rh Factor Rule:
        // - Positive can receive from Positive and Negative
        // - Negative can ONLY receive from Negative
        let rh_compatible = match (other.rh_factor, self.rh_factor) {
            (RhFactor::Positive, RhFactor::Negative) => false,
            _ => true,
        };

        // Antigen Rule:
        // - O can give to everyone
        // - AB can receive from everyone
        // - Exact matches always work
        let antigen_compatible = match (other.antigen, self.antigen) {
            (Antigen::O, _) => true,
            (_, Antigen::AB) => true,
            (a, b) if a == b => true,
            _ => false,
        };

        rh_compatible && antigen_compatible
    }

    /// Helper function to generate all possible blood types
    fn all_blood_types() -> Vec<Self> {
        let antigens = [Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        let mut all = Vec::new();
        for &rh_factor in &rh_factors {
            for &antigen in &antigens {
                all.push(BloodType { antigen, rh_factor });
            }
        }
        all
    }

    /// Returns all the blood types that can give blood to self.
    pub fn donors(self) -> Vec<Self> {
        Self::all_blood_types()
            .into_iter()
            .filter(|&other| self.can_receive_from(other))
            .collect()
    }

    /// Returns all the blood types that can receive blood from self.
    pub fn recipients(self) -> Vec<Self> {
        Self::all_blood_types()
            .into_iter()
            .filter(|&other| other.can_receive_from(self))
            .collect()
    }
}
*/