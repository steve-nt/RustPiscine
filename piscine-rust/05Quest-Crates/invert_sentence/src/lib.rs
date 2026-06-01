pub fn invert_sentence(string: &str) -> String {
    // Tokenize into alternating whitespace/word segments
    let mut segments: Vec<&str> = Vec::new();
    let mut chars = string.char_indices().peekable();
    let mut start = 0;

    while let Some((_i, c)) = chars.next() {
        let ws = c.is_whitespace();
        // Advance while same kind
        while let Some(&(_, nc)) = chars.peek() {
            if nc.is_whitespace() != ws {
                break;
            }
            chars.next();
        }
        let end = chars.peek().map(|&(j, _)| j).unwrap_or(string.len());
        segments.push(&string[start..end]);
        start = end;
    }

    // Collect words (non-whitespace segments) then reverse
    let words: Vec<&str> = segments
        .iter()
        .filter(|s| !s.starts_with(|c: char| c.is_whitespace()))
        .copied()
        .collect();
    let mut rev_words = words.iter().rev();

    segments
        .iter()
        .map(|s| {
            if s.starts_with(|c: char| c.is_whitespace()) {
                s.to_string()
            } else {
                rev_words.next().unwrap().to_string()
            }
        })
        .collect()
}