pub fn number_logic(num: u32) -> bool {
    
    let num_str = num.to_string();
    let power = num_str.len() as u32;

    
    let sum: u64 = num_str
        .chars()
        .map(|c| {
            
            let digit = c.to_digit(10).unwrap() as u64;
            
            digit.pow(power)
        })
        .sum();

    
    sum == num as u64
}