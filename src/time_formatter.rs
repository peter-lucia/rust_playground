use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Offset, Local};
use chrono::format::strftime;
use std::borrow::Borrow;
use std::thread::current;

pub struct TimeFormatter{

}

impl TimeFormatter {


    /*
    Converts an isoformatted, timezone aware datetime string to local time
    Example:
        Input: "2022-03-30T05:18:14.660921+00:00";
        Output: "2022-03-29T22:18:14.660921-07:00";

    Sources:
        * https://docs.rs/chrono/0.4.11/chrono/struct.DateTime.html
        * https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        * https://docs.rs/chrono/latest/chrono/
     */
    pub fn convert_to_local(&self, utc_time_with_offset: &str) -> String {

        let current_time = Local::now();
        let tz = current_time.timezone();
        let local_timezone = current_time.format("%Z").to_string();
        println!("Detected local timezone: {0}", local_timezone);

        let mut parsed =
            DateTime::parse_from_rfc3339(utc_time_with_offset);


        if parsed.is_ok() {
            return parsed.unwrap().with_timezone(&tz).to_rfc3339();
        }

        println!("Trying backup parsing method...");

        let fmt = "%+";
        parsed = DateTime::parse_from_str(
            utc_time_with_offset,
            fmt
        );

        return parsed.unwrap().with_timezone(&tz).to_rfc3339();

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
        let actual_result = t_formatter.convert_to_local(input_time_utc);
        assert_eq!(expected_result, actual_result);
    }
}
