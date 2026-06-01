pub fn talking(text: &str) -> &str {
    
    let trimmed = text.trim();
    
    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_question = trimmed.ends_with('?');
    
    
    let is_yelling = trimmed.chars().any(char::is_alphabetic) 
                  && !trimmed.chars().any(char::is_lowercase);

    
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting",
    }
}