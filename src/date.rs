use chrono::{Datelike, Duration, NaiveDate};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    naive_date: NaiveDate,
}
impl Date {
    pub fn new(captures: &regex::Captures) -> Self {
        Date {
            naive_date: NaiveDate::from_ymd(
                captures.name("year").unwrap().as_str().parse().unwrap(),
                captures.name("month").unwrap().as_str().parse().unwrap(),
                captures.name("day").unwrap().as_str().parse().unwrap(),
            ),
        }
    }
    pub fn add(&mut self, days: u32, months: u32, years: u32) {
        let month0_sum = self.naive_date.month0() + months;
        let mut new_naive_date = NaiveDate::from_ymd(
            self.naive_date.year() + (years + (month0_sum / 12)) as i32,
            (month0_sum % 12) + 1,
            self.naive_date.day(),
        );
        new_naive_date += Duration::days(days as i64);
        self.naive_date = new_naive_date;
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.naive_date.day(),
            self.naive_date.month(),
            self.naive_date.year()
        )
    }
}
