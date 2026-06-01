pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            // Match lowercase letters
            'a'..='z' => {
                let base = 'a' as i32;
                let offset = c as i32 - base;
                // rem_euclid correctly wraps negative numbers (e.g., -1 becomes 25)
                let new_offset = (offset + key as i32).rem_euclid(26);
                (base + new_offset) as u8 as char
            }
            // Match uppercase letters
            'A'..='Z' => {
                let base = 'A' as i32;
                let offset = c as i32 - base;
                let new_offset = (offset + key as i32).rem_euclid(26);
                (base + new_offset) as u8 as char
            }
            // Any other character (numbers, punctuation, spaces) remains unchanged
            _ => c,
        })
        .collect()
}