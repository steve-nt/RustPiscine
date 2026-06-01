#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop iterating if we have reached 1 (or 0)
        if self.v <= 1 {
            return None;
        }
        
        // 1. Save the current state to yield it
        let current_item = Collatz { v: self.v };
        
        // 2. Update self.v for the next iteration
        self.v = if self.v % 2 == 0 { 
            self.v / 2 
        } else { 
            3 * self.v + 1 
        };
        
        // 3. Return the saved state
        Some(current_item)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}

/*
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    // Change the Item type to Collatz
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        // Return early if we have reached 1 (or if starting at 0)
        if self.v <= 1 {
            return None;
        }
        
        // Calculate the next step
        self.v = if self.v % 2 == 0 { 
            self.v / 2 
        } else { 
            3 * self.v + 1 
        };
        
        // Yield the current struct
        // Since Collatz derives Copy and Clone, we can just return a new instance
        Some(Collatz { v: self.v })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}




#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.v <= 1 {
            return None;
        }
        self.v = if self.v % 2 == 0 { self.v / 2 } else { 3 * self.v + 1 };
        Some(self.v)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}
*/