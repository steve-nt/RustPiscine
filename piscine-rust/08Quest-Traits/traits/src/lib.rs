use std::fmt;

#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
    pub strength: f64,
    pub score: u32,
    pub money: u32,
    pub weapons: Vec<&'a str>,
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Player<'_> {
    pub fn eat(&mut self, food: impl Food) {
        self.strength += food.gives();
    }
}

// Implement the Display trait to format the Player struct as requested
impl fmt::Display for Player<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "Strength: {}, Score: {}, Money: {}",
            self.strength, self.score, self.money
        )?;
        // Use write! instead of writeln! for the last line to avoid an extra trailing newline
        write!(f, "Weapons: {:?}", self.weapons)
    }
}

pub trait Food {
    fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        // Fruit gives 4 units of strength per kilogram
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        // Calculate the exact weight of fat and protein
        let fat_weight = self.weight_in_kg * self.fat_content;
        let protein_weight = self.weight_in_kg - fat_weight;
        
        // Fat gives 9 units, protein gives 4 units
        (fat_weight * 9.0) + (protein_weight * 4.0)
    }
}