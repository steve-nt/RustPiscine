pub fn num_to_ordinal(x: u32) -> String {
    // First, isolate the last two digits to check for the "teen" exceptions.
    let suffix = match x % 100 {
        11 | 12 | 13 => "th",
        // If it's not a teen, we determine the suffix by the last digit.
        _ => match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    // Combine the number and the suffix into a new String
    format!("{}{}", x, suffix)
}