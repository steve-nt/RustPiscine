pub fn is_luhn_formula(code: &str) -> bool {
    // Filter out spaces and collect the characters into a Vector
    let chars: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();

    // A valid number must have more than 1 digit
    if chars.len() <= 1 {
        return false;
    }

    let mut sum = 0;

    // Iterate through the characters from right to left
    // `enumerate()` gives us the index, starting at 0 for the rightmost digit
    for (i, c) in chars.iter().rev().enumerate() {
        // Attempt to convert the character to a base-10 digit
        match c.to_digit(10) {
            Some(mut digit) => {
                // Every second digit from the right has an odd index (1, 3, 5, ...)
                if i % 2 == 1 {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }
                sum += digit;
            }
            // If we encounter a character that isn't a number (e.g., a letter), it's invalid
            None => return false,
        }
    }

    // The formula is valid if the sum is evenly divisible by 10
    sum % 10 == 0
}