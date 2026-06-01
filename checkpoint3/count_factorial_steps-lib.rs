pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial == 0 || factorial == 1{
      return 0;
    }

    let mut current: u64 = 1;
    let mut step: u64 = 1;

    while current < factorial {
        step = step + 1;

        match current.checked_mul(step) {
            Some(next_val) => current = next_val,
            None => return 0,            
        }
    }
    if current == factorial {
        step
    } else {
        0
    }
}




/*
pub fn count_factorial_steps(factorial: u64) -> u64 {
    // Return 0 if the argument is 0 or 1 as requested by the instructions
    if factorial == 0 || factorial == 1 {
        return 0;
    }

    let mut current: u64 = 1;
    let mut step: u64 = 1;

    // Keep multiplying until our running product matches or exceeds the target
    while current < factorial {
        step += 1;
        
        // Use checked_mul to prevent panic if a very large non-factorial u64 is passed.
        // The maximum factorial that fits in a u64 is 20! 
        match current.checked_mul(step) {
            Some(next_val) => current = next_val,
            None => return 0, // Overflow means the target number is not a valid factorial
        }
    }

    // If our calculated factorial exactly matches the input, return the steps.
    // Otherwise, it means the input falls between two factorials, so return 0.
    if current == factorial {
        step
    } else {
        0
    }
}
*/