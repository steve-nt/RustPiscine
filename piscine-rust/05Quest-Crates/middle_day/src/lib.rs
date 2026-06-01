use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    
    
    if NaiveDate::from_yo_opt(year as i32, 366).is_some() {
        return None;
    }

    
    
    NaiveDate::from_yo_opt(year as i32, 183).map(|date| date.weekday())
}