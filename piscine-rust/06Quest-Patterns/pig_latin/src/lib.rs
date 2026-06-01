pub fn pig_latin(text: &str) -> String {
    // Helper closure to check if a character is a Latin vowel
    let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U');

    // Handle empty strings safely
    if text.is_empty() {
        return String::new();
    }

    let mut chars = text.chars();
    let c1 = chars.next();
    let c2 = chars.next();
    let c3 = chars.next();

    // Rule 1: If it begins with a vowel, just add "ay"
    if let Some(first_char) = c1 {
        if is_vowel(first_char) {
            return format!("{}ay", text);
        }
    }

    // Rule 3: If it begins with a consonant followed by "qu"
    // e.g., "square" -> c1='s', c2='q', c3='u'
    if let (Some(first), Some('q' | 'Q'), Some('u' | 'U')) = (c1, c2, c3) {
        if !is_vowel(first) {
            // Find the byte split index: the length of the first char + 2 bytes for 'q' and 'u'
            let split_idx = first.len_utf8() + 2;
            return format!("{}{}ay", &text[split_idx..], &text[..split_idx]);
        }
    }

    // Rule 2: Starts with a consonant. Find the index of the first vowel.
    if let Some(vowel_idx) = text.find(is_vowel) {
        format!("{}{}ay", &text[vowel_idx..], &text[..vowel_idx])
    } else {
        // Fallback for words without any vowels (e.g., "rhythm")
        format!("{}ay", text)
    }
}