pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|token| {
            if token.ends_with('k') {
                // Remove the 'k', parse the remainder as a float, multiply by 1000, and cast to u32
                let num_str = &token[..token.len() - 1];
                let value = num_str.parse::<f64>().unwrap() * 1000.0;
                Box::new(value as u32)
            } else {
                // If there's no 'k', just parse it directly as a u32
                Box::new(token.parse::<u32>().unwrap())
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter()
        .map(|boxed_val| *boxed_val) // Dereference the box to extract the u32 value
        .collect()
}