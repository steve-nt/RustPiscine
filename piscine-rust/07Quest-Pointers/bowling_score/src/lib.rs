#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u32>,
    frame: u32,
    roll_in_frame: u32,
    pins_standing: u32,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            frame: 1, // Start at frame 1
            roll_in_frame: 1, // Start at roll 1 of the frame
            pins_standing: 10, // 10 pins are placed at the end of the lane
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), Error> {
        // If the 10th frame is fully completed, the game is over
        if self.frame > 10 {
            return Err(Error::GameComplete);
        }
        
        // Cannot knock down more pins than are currently standing
        if pins > self.pins_standing {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Record the valid roll
        self.rolls.push(pins);

        // State Machine to advance game progress
        if self.frame < 10 {
            if self.roll_in_frame == 1 {
                if pins == 10 {
                    // Strike! Skip the second roll and move to the next frame.
                    self.frame += 1;
                    self.roll_in_frame = 1;
                    self.pins_standing = 10;
                } else {
                    // Open frame so far, set up the second roll.
                    self.roll_in_frame = 2;
                    self.pins_standing -= pins;
                }
            } else {
                // Second roll completed, move to the next frame.
                self.frame += 1;
                self.roll_in_frame = 1;
                self.pins_standing = 10;
            }
        } else {
            // 10th Frame Special Rules
            if self.roll_in_frame == 1 {
                if pins == 10 {
                    // Strike! Earns 2 fill balls. Rack resets to 10 pins.
                    self.roll_in_frame = 2;
                    self.pins_standing = 10;
                } else {
                    // Normal first roll.
                    self.roll_in_frame = 2;
                    self.pins_standing -= pins;
                }
            } else if self.roll_in_frame == 2 {
                let first_roll = self.rolls[self.rolls.len() - 2];
                
                if first_roll == 10 {
                    // The first roll was a strike. This is Fill Ball #1.
                    if pins == 10 {
                        // Strike on Fill Ball #1! Rack resets for Fill Ball #2.
                        self.roll_in_frame = 3;
                        self.pins_standing = 10;
                    } else {
                        // Not a strike, remaining pins stay for Fill Ball #2.
                        self.roll_in_frame = 3;
                        self.pins_standing -= pins;
                    }
                } else if first_roll + pins == 10 {
                    // Spare! Earns 1 fill ball. Rack resets to 10 pins.
                    self.roll_in_frame = 3;
                    self.pins_standing = 10;
                } else {
                    // Open 10th frame. Game is over.
                    self.frame = 11;
                }
            } else if self.roll_in_frame == 3 {
                // Second Fill Ball completed. Game is over.
                self.frame = 11;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u32> {
        // According to instructions, if the game isn't complete, we return None.
        if self.frame <= 10 {
            return None;
        }

        let mut total = 0;
        let mut roll_index = 0;

        // Calculate score frame-by-frame
        for _ in 0..10 {
            if self.rolls[roll_index] == 10 {
                // Strike: 10 + next two rolls
                total += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                roll_index += 1; // Strikes only consume 1 roll locally in the frame
            } else if self.rolls[roll_index] + self.rolls[roll_index + 1] == 10 {
                // Spare: 10 + next one roll
                total += 10 + self.rolls[roll_index + 2];
                roll_index += 2; // Spares consume 2 rolls
            } else {
                // Open frame: sum of the two rolls
                total += self.rolls[roll_index] + self.rolls[roll_index + 1];
                roll_index += 2; // Open frames consume 2 rolls
            }
        }

        Some(total)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}