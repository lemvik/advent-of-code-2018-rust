extern crate chrono;
extern crate common;

use chrono::{NaiveDate, NaiveDateTime};
use common::errors::ParseError;
use std::cmp::Ordering;
use std::error::Error;
use std::str;

#[derive(Clone, Debug, Eq)]
pub enum GuardLogEntry {
    ReportingForDuty(u32, NaiveDateTime),
    FallsAsleep(NaiveDateTime),
    WakesUp(NaiveDateTime),
}

impl GuardLogEntry {
    pub fn date(&self) -> &NaiveDateTime {
        match self {
            GuardLogEntry::ReportingForDuty(_, ref date) => date,
            GuardLogEntry::FallsAsleep(ref date) => date,
            GuardLogEntry::WakesUp(ref date) => date,
        }
    }

    pub fn parse(line: &str) -> std::result::Result<GuardLogEntry, ParseError> {
        let (date_chunk, descr_chunk) =
            common::parsers::split_pair(line, ']').ok_or("Failed to split on [")?;
        let date = NaiveDateTime::parse_from_str(&date_chunk[1..], "%Y-%m-%d %H:%M")
            .map_err(|e| ParseError::create(e.description()))?;

        let description = descr_chunk.trim();

        if description.starts_with("falls asleep") {
            return Ok(GuardLogEntry::FallsAsleep(date));
        }

        if description.starts_with("wakes up") {
            return Ok(GuardLogEntry::WakesUp(date));
        }

        if description.starts_with("Guard ") {
            let parts = description.split_whitespace();
            let id_part = parts.skip(1).next().ok_or("Failed to locate ID part.")?;
            let id = id_part[1..].parse::<u32>()?;
            return Ok(GuardLogEntry::ReportingForDuty(id, date));
        }

        Err(ParseError::create("Failed to parse entry description."))
    }
}

impl PartialEq for GuardLogEntry {
    fn eq(&self, other: &GuardLogEntry) -> bool {
        self.date().eq(other.date())
    }
}

impl Ord for GuardLogEntry {
    fn cmp(&self, other: &GuardLogEntry) -> Ordering {
        self.date().cmp(other.date())
    }
}

impl PartialOrd for GuardLogEntry {
    fn partial_cmp(&self, other: &GuardLogEntry) -> Option<Ordering> {
        self.date().partial_cmp(other.date())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            GuardLogEntry::parse("[1518-11-01 00:55] wakes up"),
            Ok(GuardLogEntry::WakesUp(
                NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 55, 0)
            ))
        );
        assert_eq!(
            GuardLogEntry::parse("[1518-11-01 23:58] Guard #99 begins shift"),
            Ok(GuardLogEntry::ReportingForDuty(
                99,
                NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0)
            ))
        );
        assert_eq!(
            GuardLogEntry::parse("[1518-11-01 23:58] falls asleep"),
            Ok(GuardLogEntry::FallsAsleep(
                NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0)
            ))
        );
    }
}
