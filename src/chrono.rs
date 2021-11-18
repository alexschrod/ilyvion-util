//! Various chrono utilities

use chrono::NaiveDate;

/// Calculates the number of days in any given month
pub fn days_from_month_in_year(month: u32, year: i32) -> i64 {
    debug_assert!((1..=12).contains(&month));

    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}
