#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    /// Initializes a new game session state.
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    /// Returns a tuple referencing the player who is currently in the lead.
    /// If they are tied, returns None.
    pub fn read_winner(&self) -> Option<&(String, u32)> {
        if self.p1.1 > self.p2.1 {
            Some(&self.p1)
        } else if self.p2.1 > self.p1.1 {
            Some(&self.p2)
        } else {
            None // It's a tie (or 0-0)
        }
    }

    /// Increments the score of the given player.
    /// Does nothing if the game is already finished or if the player name doesn't match.
    pub fn update_score(&mut self, user_name: &str) {
        let win_score = (self.nb_games / 2) + 1;
        
        // A game is finished if someone has already won, OR if the maximum number of games has been reached.
        let is_finished = self.p1.1 >= win_score 
            || self.p2.1 >= win_score 
            || (self.p1.1 + self.p2.1) >= self.nb_games;

        if is_finished {
            return;
        }

        // Increment the correct player's score
        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }

    /// Takes ownership of the `GameSession` (consuming it) and returns the formatted deletion string.
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
