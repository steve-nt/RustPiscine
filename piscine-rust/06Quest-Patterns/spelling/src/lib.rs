pub fn spell(n: u64) -> String {
    match n {
        0 => "zero".into(), // The exercise says only positive numbers, but it's good practice
        1..=19 => {
            let ones = [
                "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
                "eighteen", "nineteen",
            ];
            ones[n as usize].into()
        }
        20..=99 => {
            let tens = [
                "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
            ];
            let ten_word = tens[(n / 10) as usize];
            let rem = n % 10;
            
            if rem == 0 {
                ten_word.into()
            } else {
                // If there's a remainder (like 21), add a hyphen
                format!("{}-{}", ten_word, spell(rem))
            }
        }
        100..=999 => {
            let rem = n % 100;
            if rem == 0 {
                format!("{} hundred", spell(n / 100))
            } else {
                format!("{} hundred {}", spell(n / 100), spell(rem))
            }
        }
        1000..=999_999 => {
            let rem = n % 1000;
            if rem == 0 {
                format!("{} thousand", spell(n / 1000))
            } else {
                format!("{} thousand {}", spell(n / 1000), spell(rem))
            }
        }
        1_000_000 => "one million".into(),
        _ => String::new(), // Catch-all for numbers > 1_000_000
    }
}