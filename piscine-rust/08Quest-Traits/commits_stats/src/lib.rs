use std::collections::HashMap;
use chrono::DateTime;

/// Returns a hash map with the number of commits per week.
/// A week is represented by the ISO year followed by the ISO week number without padding (e.g., "2020-W1").
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();

    if data.is_array() {
        for item in data.members() {
            // The date string is located at commit -> author -> date
            if let Some(date_str) = item["commit"]["author"]["date"].as_str() {
                // Parse the date as an RFC 3339 formatted string
                if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                    // Format the date into "YYYY-Www" 
                    // %G represents the ISO week-numbering year
                    // %-V represents the ISO week number without zero-padding
                    let week_key = date.format("%G-W%-V").to_string();
                    *week_counts.entry(week_key).or_insert(0) += 1;
                }
            }
        }
    }

    week_counts
}

/// Returns a hash map with the number of commits per author.
/// Authors are identified by their GitHub login.
pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();

    if data.is_array() {
        for item in data.members() {
            // The GitHub login is located at author -> login
            if let Some(login) = item["author"]["login"].as_str() {
                *author_counts.entry(login.to_string()).or_insert(0) += 1;
            }
        }
    }

    author_counts
}