use bowling_score::*;

fn main() -> Result<(), Error> {
    let mut game = BowlingGame::new();
    game.roll(0)?; // frame 1
    game.roll(10)?; // frame 1: spare
    game.roll(10)?; // frame 2: strike
    game.roll(5)?; // frame 3
    game.roll(5)?; // frame 3: spare
    game.roll(10)?; // frame 4: strike
    game.roll(10)?; // frame 5: strike
    game.roll(10)?; // frame 6: strike
    game.roll(10)?; // frame 7: strike
    game.roll(10)?; // frame 8: strike
    game.roll(10)?; // frame 9: strike
    game.roll(10)?; // frame 10: strike
    game.roll(2)?; // fill ball 1
    game.roll(8)?; // fill ball 2
    println!("{:?}", game.score());

    let mut perfect_game = BowlingGame::new();
    for _ in 0..12 {
        perfect_game.roll(10)?;
    }
    println!("{:?}", perfect_game.score());
    Ok(())
}