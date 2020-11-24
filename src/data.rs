use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use dtparse::parse;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    date: DateTime<Utc>,
    title: String,
    content: String,
}

impl Event {
    pub fn new() -> Event {
        Event {
            date: Utc::now(),
            title: "".to_string(),
            content: "".to_string(),
        }
    }

    fn parse_from_str(s: &str) -> Result<DateTime<Utc>, dtparse::ParseError> {
        let time = parse(s);
        match time {
            Ok((naive, _)) => Ok(DateTime::from(Local.from_local_datetime(&naive).unwrap())),
            // Ok((naive, Some(offset))) => Ok(),
            // TODO:ignore the time zone now
            Err(e) => Err(e),
        }
    }

    pub fn from_str(time: &str, title: &str) -> Event {
        let correct_time = Event::parse_from_str(time).unwrap();
        Event {
            date: correct_time,
            title: title.to_string(),
            content: "".to_string(),
        }
    }

    pub fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0:x}\t{1}\t{2}\t{3}",
            self.calculate_hash(),
            self.date
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M")
                .to_string(),
            self.title,
            self.content
        )
    }
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
        self.content.hash(state);
        self.date.hash(state);
    }
}

#[test]
fn event_can_be_create() {
    let e = Event::new();
}

#[test]
fn parse_is_correct() {
    let dt = Event::parse_from_str("05-04-20");
    assert_eq!(dt, Ok(Utc.ymd(2020, 5, 3).and_hms(16, 0, 0)));
    let dt2 = Event::parse_from_str("today");
    println!("{:?}", dt2);
}
