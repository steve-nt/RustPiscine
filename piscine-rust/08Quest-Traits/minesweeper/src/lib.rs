pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(rows);
    
    // Convert strings to byte arrays for efficient indexing 
    // (safe since the board only uses ASCII characters '*' and ' ')
    let board: Vec<&[u8]> = minefield.iter().map(|r| r.as_bytes()).collect();

    for r in 0..rows {
        let cols = board[r].len();
        let mut row_str = String::with_capacity(cols);

        for c in 0..cols {
            if board[r][c] == b'*' {
                // If it's a mine, just push the mine
                row_str.push('*');
            } else {
                // If it's empty, count adjacent mines
                let mut count = 0;
                
                // Define the safe search area (3x3 grid around the current cell)
                let min_r = r.saturating_sub(1);
                let max_r = (r + 1).min(rows - 1);
                let min_c = c.saturating_sub(1);
                let max_c = (c + 1).min(cols - 1);

                for i in min_r..=max_r {
                    for j in min_c..=max_c {
                        if board[i][j] == b'*' {
                            count += 1;
                        }
                    }
                }

                // Append the count if > 0, otherwise keep the space
                if count > 0 {
                    // Convert the u8 count into a character
                    row_str.push(std::char::from_digit(count as u32, 10).unwrap());
                } else {
                    row_str.push(' ');
                }
            }
        }
        result.push(row_str);
    }

    result
}