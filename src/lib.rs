use chrono::{
    NaiveDate, NaiveDateTime, NaiveTime,
    prelude::*,
};
use chrono_tz::Tz;
use either::Either;
use std::{
    collections::HashMap,
    path::Path,
    error::Error,
};

pub fn parse_dt_parts(parts: &[String], tz_abbr: &HashMap<String, String>) -> Result<(NaiveDateTime, Either<Tz, Local>), &'static str> {
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

        if let Some(tz) = try_parse_timezone(part, tz_abbr) && let None = timezone {
            timezone = Some(tz);
        } else if let Some(d) = try_parse_date(&part) && let None = date {
            date = Some(d);
        } else if let None = time {
            let mut t = None;

            if i + 1 < parts.len() {
                t = try_parse_time(&format!("{}{}", part, parts[i + 1]));
            }
            
            let p = t.is_some();
            t = t.or(try_parse_time(part));

            if t.is_some() {
                time = t;
                pass = p;
            } else {
                Err("Invalid datetime")?;
            }
        } else {
            Err("Invalid datetime")?;
        }
    }
    
    let date = date.unwrap_or(now.date_naive());
    let time = time.unwrap_or(now.time());
    let datetime = NaiveDateTime::new(date, time);

    if let Some(tz) = timezone {
        Ok((datetime, Either::Left(tz)))
    } else {
        Ok((datetime, Either::Right(Local)))
    }
}

fn try_parse_timezone(timezone: &str, tz_abbr: &HashMap<String, String>) -> Option<Tz> {
    Tz::from_str_insensitive(
        tz_abbr.get(timezone)
            .map(|x| x.as_str())
            .unwrap_or(timezone)
    ).ok()
}

fn try_parse_time(time: &str) -> Option<NaiveTime> {
    if time.len() == 3 || time.len() == 4 {
        return NaiveTime::parse_from_str(&format!("00{time}"), "%M%I%P").ok();
    }

    NaiveTime::parse_from_str(&time, "%H:%M:%S")
        .or(NaiveTime::parse_from_str(&time, "%H:%M"))
        .or(NaiveTime::parse_from_str(&time, "%I:%M:%S%P"))
        .or(NaiveTime::parse_from_str(&time, "%I:%M%P"))
        .ok()
}

fn try_parse_date(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(&date,"%m-%d")
        .or(NaiveDate::parse_from_str(&date, "%Y-%m-%d"))
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