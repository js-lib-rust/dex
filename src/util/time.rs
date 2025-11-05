use std::time::{SystemTime, UNIX_EPOCH};
use log::error;

pub struct DateTime {
    millis: u128,
}

impl DateTime {
    const MILLIS_PER_DAY: u128 = 24 * 60 * 60 * 1000;
    const UNIX_EPOCH_WEEKDAY: u32 = 4;

    pub fn now() -> Self {
        let millis = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(error) => {
                error!("fail to load system time: {error}");
                0
            }
        };
        Self {
            millis: Self::local(millis),
        }
    }

    fn local(millis: u128) -> u128 {
        let days_since_epoch = Self::days_since_epoch(millis);
        let (year, month, day) = Self::date_components(days_since_epoch);

        let offset_hours = if Self::is_dst_active(year, month, day) {
            3 // UTC+3 during summer time
        } else {
            2 // UTC+2 during standard time
        };

        let offset_millis = offset_hours * 60 * 60 * 1000;
        millis + offset_millis
    }

    pub fn iso8601(&self) -> String {
        let secs = (self.millis / 1000) as u64;
        let mut remaining_secs = secs % 86400;

        let hours = remaining_secs / 3600;
        remaining_secs %= 3600;

        let minutes = remaining_secs / 60;
        let seconds = remaining_secs % 60;

        let days = (secs / 86400) as u32;
        let (year, month, day) = Self::date_components(days);

        // ISO 8601: YYYY-MM-DDTHH:MM:SS.MILLISZ
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            self.millis % 1000
        )
    }
}

impl DateTime {
    fn days_since_epoch(millis: u128) -> u32 {
        (millis / DateTime::MILLIS_PER_DAY) as u32
    }

    fn is_dst_active(year: u32, month: u32, day: u32) -> bool {
        let dst_start_day = Self::last_sunday_of_month(year, 3);
        let dst_end_day = Self::last_sunday_of_month(year, 10);

        if month < 3 || month > 10 {
            false
        } else if month > 3 && month < 10 {
            true
        } else if month == 3 {
            day >= dst_start_day
        } else {
            // month == 10
            day < dst_end_day
        }
    }

    fn last_sunday_of_month(year: u32, month: u32) -> u32 {
        let last_day = Self::days_in_month(year, month);

        // calculate days since epoch for the last day of the month
        let mut days_since_epoch = 0;
        for y in 1970..year {
            days_since_epoch += if Self::is_leap_year(y) { 366 } else { 365 };
        }
        for m in 1..month {
            days_since_epoch += Self::days_in_month(year, m);
        }
        days_since_epoch += last_day - 1; // -1 because we want 0-indexed for calculation

        let weekday = Self::day_of_week(days_since_epoch);

        // If last day is Sunday (0), that's our date
        // Otherwise, subtract days to get to the previous Sunday
        if weekday == 0 {
            last_day
        } else {
            last_day - weekday
        }
    }

    fn day_of_week(days_since_epoch: u32) -> u32 {
        (days_since_epoch + DateTime::UNIX_EPOCH_WEEKDAY) % 7
    }

    fn date_components(days: u32) -> (u32, u32, u32) {
        let mut year = 1970 as u32;
        let mut remaining_days = days;

        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if remaining_days < days_in_year {
                break;
            }
            remaining_days -= days_in_year;
            year += 1;
        }

        let mut month = 1;
        while month <= 12 {
            let days_in_current_month = Self::days_in_month(year, month);
            if remaining_days < days_in_current_month {
                break;
            }
            remaining_days -= days_in_current_month;
            month += 1;
        }

        (year, month, remaining_days + 1) // +1 because days are 1-indexed
    }

    fn days_in_month(year: u32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month"),
        }
    }

    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

#[cfg(test)]
mod test {
    use crate::util::time::DateTime;

    impl From<u128> for DateTime {
        fn from(millis: u128) -> Self {
            Self { millis }
        }
    }

    #[test]
    fn zero_utc_date_time_to_iso8601() {
        let date_time = DateTime::from(0);
        assert_eq!(date_time.iso8601(), "1970-01-01T00:00:00.000Z");
    }

    #[test]
    fn zero_utc_to_local_millis() {
        let millis = DateTime::local(0);
        assert_eq!(millis, 7200_000);
    }

    #[test]
    fn zero_local_to_iso8601() {
        let date_time = DateTime::from(DateTime::local(0));
        assert_eq!(date_time.iso8601(), "1970-01-01T02:00:00.000Z");
    }

    #[test]
    fn date_time_to_iso8601() {
        let date_time = DateTime::from(226359000_000);
        assert_eq!(date_time.iso8601(), "1977-03-04T21:30:00.000Z");
    }
}
