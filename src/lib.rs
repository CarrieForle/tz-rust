use chrono::{
    NaiveDate, NaiveDateTime, NaiveTime,
    prelude::*,
};
use chrono_tz::Tz;
use std::{
    collections::HashMap,
    path::Path,
    error::Error,
};

pub fn parse_dt_parts(parts: &[String], tz_abbr: &HashMap<String, String>) -> Result<(NaiveDateTime, Option<Tz>), &'static str> {
    let now = Local::now();
    let mut timezone: Option<Tz> = None;
    let mut date = None;
    let mut time = None; 
    let mut pass = false;

    for (i, part) in parts.iter().enumerate() {
        if pass {
            pass = false;
            continue;
        }

        if timezone.is_none() && let Some(tz) = try_parse_timezone(part, tz_abbr) {
            timezone = Some(tz);
        } else if date.is_none() && let Some(d) = try_parse_date(part, &now) {
            date = Some(d);
        } else if time.is_none() {
            let mut t = None;

            if i + 1 < parts.len() {
                t = try_parse_time(&format!("{}{}", part, parts[i + 1]));
            }
            
            if t.is_some() {
                time = t;
                pass = true;
            } else {
                t = t.or(try_parse_time(part));

                if t.is_some() {
                    time = t;
                } else {
                    Err("Invalid datetime")?;
                }
            }
        } else {
            Err("Invalid datetime")?;
        }
    }
    
    let date = date.unwrap_or(now.date_naive());
    let time = time.unwrap_or(now.time());
    let datetime = NaiveDateTime::new(date, time);
    
    Ok((datetime, timezone))
}

fn try_parse_timezone(timezone: &str, tz_abbr: &HashMap<String, String>) -> Option<Tz> {
    Tz::from_str_insensitive(
        tz_abbr.get(timezone)
            .map(|x| x.as_str())
            .unwrap_or(timezone)
    ).ok()
}

fn try_parse_time(time: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time, "%H:%M:%S")
        .or(NaiveTime::parse_from_str(time, "%H:%M"))
        .or(NaiveTime::parse_from_str(time, "%H%M"))
        .or(NaiveTime::parse_from_str(time, "%I:%M:%S%P"))
        .or(NaiveTime::parse_from_str(time, "%I:%M%P"))
        .ok()
}

fn try_parse_date(date: &str, now: &DateTime<Local>) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(&format!("{}-{date}", now.year()), "%Y-%m-%d")
        .or(NaiveDate::parse_from_str(date, "%Y-%m-%d"))
        .or(NaiveDate::parse_from_str(date, "%y-%m-%d"))
        .or(NaiveDate::parse_from_str(&format!("{}/{date}", now.year()), "%Y/%m/%d"))
        .or(NaiveDate::parse_from_str(date, "%Y/%m/%d"))
        .or(NaiveDate::parse_from_str(date, "%y/%m/%d"))
        .ok()
}

pub fn read_tz_abbr<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut map = HashMap::new();
    const ERROR: &str = "Invalid format";

    for line in contents.lines() {
        let (key, val) = line.split_once('=').ok_or(ERROR)?;
        map.insert(key.to_string(), val.to_string());
    }

    Ok(map)
}