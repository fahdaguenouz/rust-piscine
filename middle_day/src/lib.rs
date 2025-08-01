use chrono::{NaiveDate, Datelike, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let is_leap = NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some();
    let days = if is_leap { 366 } else { 365 };

    if days % 2 == 1 {
        let middle_day = days / 2 + 1;
        NaiveDate::from_yo_opt(year as i32, middle_day)
            .map(|date| date.weekday())
    } else {
        None
    }
}
