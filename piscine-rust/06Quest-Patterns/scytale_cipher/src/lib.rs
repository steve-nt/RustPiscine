pub fn scytale_cipher(message: &str, i: usize) -> String {
    // Handle edge cases: an empty string or 0 wraps
    if i == 0 || message.is_empty() {
        return String::new();
    }

    // Collect the string into a Vector of chars to safely handle Unicode indices
    let chars: Vec<char> = message.chars().collect();
    let num_chars = chars.len();
    
    // Calculate the number of rows needed (ceil division)
    let rows = (num_chars + i - 1) / i;
    
    let mut result = String::with_capacity(rows * i);

    // Read column by column
    for col in 0..i {
        for row in 0..rows {
            let idx = row * i + col;
            
            if idx < num_chars {
                result.push(chars[idx]);
            } else {
                // If we go out of bounds, it's the padded area of the strip
                result.push(' ');
            }
        }
    }

    // The output example for size 8 removes the trailing padded spaces
    result.trim_end().to_string()
}