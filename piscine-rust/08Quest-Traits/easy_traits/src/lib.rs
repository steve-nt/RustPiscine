pub trait AppendStrExt {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self;
    fn append_number(&mut self, nb_to_append: f64) -> &mut Self;
    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStrExt for String {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self {
        self.push_str(str_to_append);
        self // Return the mutable reference to enable method chaining
    }

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self {
        // Convert the float to a string first, then append it
        self.push_str(&nb_to_append.to_string());
        self
    }

    fn remove_punctuation_marks(&mut self) -> &mut Self {
        // `retain` keeps only the characters that return `true` for the closure
        self.retain(|c| c != '.' && c != ',' && c != '?' && c != '!');
        self
    }
}