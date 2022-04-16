use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Offset, Local};
use chrono::format::strftime;
use std::borrow::Borrow;
use std::thread::current;
use chrono_tz::Tz;
use chrono_tz::UTC;

pub struct TimeFormatter{

}

impl TimeFormatter {


    /*
    Converts an rc3339 formatted utc datetime string to the desired timezone

    Sources:
        * https://docs.rs/chrono/0.4.11/chrono/struct.DateTime.html
        * https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        * https://docs.rs/chrono/latest/chrono/
        * https://docs.rs/chrono-tz/latest/chrono_tz/
     */
    pub fn convert(&self, utc_time: &str, desired_timezone: &str) -> Option<String> {

        let tz: Tz = desired_timezone.parse().unwrap();
        let mut parsed =
            DateTime::parse_from_rfc3339(utc_time);

        if parsed.is_ok() {
            return Some(parsed.unwrap().with_timezone(&UTC).with_timezone(&tz).to_rfc3339());
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::time_formatter::TimeFormatter;

    #[test]
    fn test_1() {
        let t_formatter = TimeFormatter{};
        let input_time_utc= "2022-03-30T05:18:14.660921+00:00";
        let expected_result = "2022-03-29T22:18:14.660921-07:00"; // only works in daylight savings time
        let actual_result = t_formatter.convert(input_time_utc, "US/Pacific");
        assert_eq!(expected_result, actual_result.unwrap());
    }
}
