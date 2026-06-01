pub fn get_diamond(c: char) -> Vec<String> {
    // Calculate the distance from 'A'. For 'A' this is 0, 'B' is 1, 'C' is 2, etc.
    let n = (c as u8 - b'A') as usize;
    let mut top_half = Vec::new();

    // Build the top half (including the middle row)
    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let outer_spaces = " ".repeat(n - i);

        // Pattern matching / if-else to handle the tip vs the body of the diamond
        let row = if i == 0 {
            format!("{}{}{}", outer_spaces, letter, outer_spaces)
        } else {
            let inner_spaces = " ".repeat(2 * i - 1);
            format!("{}{}{}{}{}", outer_spaces, letter, inner_spaces, letter, outer_spaces)
        };
        
        top_half.push(row);
    }

    // Clone the top half to build the full diamond
    let mut diamond = top_half.clone();
    
    // Reverse the top half, skip the first element (which is the middle row),
    // and append the rest to form the bottom half.
    for row in top_half.into_iter().rev().skip(1) {
        diamond.push(row);
    }

    diamond
}