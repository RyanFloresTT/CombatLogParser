use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset};
use regex::Regex;

pub fn extract_timestamp(line: &str) -> DateTime<Utc> {
    let re = Regex::new(r"(\d{1,2})/(\d{1,2})/(\d{4})\s+(\d{2}):(\d{2}):(\d{2})\.(\d{3})-(\d+)").unwrap();
    if let Some(cap) = re.captures(line) {
        let month: u32 = cap[1].parse().unwrap_or(1);
        let day: u32 = cap[2].parse().unwrap_or(1);
        let year: i32 = cap[3].parse().unwrap_or(2025);
        let hour: u32 = cap[4].parse().unwrap_or(0);
        let min: u32 = cap[5].parse().unwrap_or(0);
        let sec: u32 = cap[6].parse().unwrap_or(0);
        let millis: u32 = cap[7].parse().unwrap_or(0);
        let tz_offset: i32 = -cap[8].parse::<i32>().unwrap_or(0);

        // Create DateTime with timezone offset
        let naive = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year, month, day).unwrap_or_default(),
            NaiveTime::from_hms_milli_opt(hour, min, sec, millis).unwrap_or_default()
        );

        // Apply timezone offset
        let offset = FixedOffset::east_opt(tz_offset * 3600).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
        DateTime::<Utc>::from(naive.and_local_timezone(offset).unwrap())
    } else {
        Utc::now()
    }
}
