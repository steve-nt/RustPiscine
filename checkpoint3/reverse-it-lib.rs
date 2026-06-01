pub fn reverse_it(v: i32) -> String {
    let s = v.to_string();
    let (is_negative, abs_str) = if s.starts_with('-') {
          (true, &s[1..])
    } else {
        (false, s.as_str())
    };
    let reversed: String = abs_str.chars().rev().collect();
    let mut result = String::new();
    if is_negative {
        result.push('-');      
    }
    result.push_str(&reversed);
    result.push_str(abs_str);
    result
    
}

/*
pub fn reverse_it(v: i32) -> String {
    // Convert the number to a string
    let s = v.to_string();
    
    // Check if it's negative and extract the absolute numeric part
    let (is_negative, abs_str) = if s.starts_with('-') {
        (true, &s[1..])
    } else {
        (false, s.as_str())
    };

    // Reverse the characters of the absolute part
    let reversed: String = abs_str.chars().rev().collect();
    
    // Construct the final string
    let mut result = String::new();
    if is_negative {
        result.push('-');
    }
    
    // Append the reversed string followed by the original absolute string
    result.push_str(&reversed);
    result.push_str(abs_str);
    
    result
}
*/