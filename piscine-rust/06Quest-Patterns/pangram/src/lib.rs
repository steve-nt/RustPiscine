pub fn is_pangram(s: &str) -> bool {
    
    let mut seen = [false; 26];

    for c in s.chars() {
        
        match c.to_ascii_lowercase() {            
            
            lower_c @ 'a'..='z' => {
                let index = lower_c as usize - 'a' as usize;
                seen[index] = true;
            }
            
            _ => {}
        }
    }
    
    seen.iter().all(|&b| b)
}