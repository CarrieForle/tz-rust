use chrono::{Local, LocalResult, NaiveDateTime, TimeZone};
use tz_rust::*;
use clap::{ValueEnum, Parser};
use std::{
    collections::HashMap, env, error::Error, fmt::Display
};

#[derive(Parser)]
#[command(name = "tz", version, about)]
#[command(after_long_help = AFTER_HELP)]
pub struct Cli {
    #[arg(short, long)]
    #[arg(default_value_t = Mode::To)]
    pub mode: Mode,
    #[arg(value_name = "DATETIME_PARTS")]
    #[arg(help = "Datetime with timezone")]
    #[arg(long_help = DT_LONG_HELP)]
    pub dt: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    /// From timezone to localtime.
    From,
    /// From localtime to timezone.
    To,
    /// Get timestamp.
    #[value(name = "ts")]
    Timestamp,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::From => {
                "from"
            },
            Mode::To => {
                "to"
            },
            Mode::Timestamp => {
                "ts"
            }
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tz_abbr = env::current_exe()
        .map(|mut p| {
            let _ = p.pop();
            p.push("tz.txt");
            read_tz_abbr(p).unwrap_or(HashMap::new())
        })?;

    let cli = Cli::parse();

    if cli.dt.is_empty() {
        let localtime = Local::now();

        if let Mode::Timestamp = cli.mode {
            println!("{}", localtime.format("%s"));
        } else {
            println!("{}", localtime.format("%F %T %s"));
        }
    } else {
        let dt = parse_dt_parts(&cli.dt, &tz_abbr);

        match dt {
            Ok((dt, Some(tz))) => match cli.mode {
                Mode::From => {
                    convert_and_print_dt(dt, &tz, &Local, "%F %T %s");
                }
                Mode::To => {
                    if tz == chrono_tz::UTC {
                        convert_and_print_dt(dt, &Local, &tz, "%F %T %Z %s");
                    } else {
                        convert_and_print_dt(dt, &Local, &tz, "%F %T %Z%z %s");
                    }
                }
                Mode::Timestamp => {
                    convert_and_print_dt(dt, &Local, &Local, "%s");
                }
            }
            Ok((dt, None)) => {
                println!("{}", dt.format("%F %T %s"));
            }
            Err(e) => {
                Err(e)?;
            }
        }
    }

    Ok(())
}

fn convert_and_print_dt<Tz1, Tz2>(dt: NaiveDateTime, tz_from: &Tz1, tz_to: &Tz2, format: &str) 
    where 
        Tz1: TimeZone,
        Tz2: TimeZone,
        Tz2::Offset: Display,
{
    let dt_res = tz_from.from_local_datetime(&dt);
    
    // There are Ambiguous (fold) and None (gap)
    // because some Timezone forward or roll back 
    // their clock (e.g., due to DST).
    //
    // This makes certain datetime not exist (gap)
    // (e.g., 14:00~14:59 does not exist when DST
    // begins, forwarding the clock by 1 hour),
    // or have two possible timestamps (fold)
    // (e.g., two 13:00~:13:59 when DST ends,
    // rolling the clock back by 1 hour).
    match dt_res {
        LocalResult::Single(dt) => {
            let dt = dt.with_timezone(tz_to);
            println!("{}", dt.format(format));
        }
        LocalResult::Ambiguous(dt_early, dt_late) => {
            let dt_early = dt_early.with_timezone(tz_to);
            let dt_late = dt_late.with_timezone(tz_to);

            println!("There are two datetimes (fold):");
            println!("{}", dt_early.format(format));
            println!("{}", dt_late.format(format));
        }
        LocalResult::None => {
            println!("The datetime does not exist (gap).");
        }
    }
}

const DT_LONG_HELP: &str = 
r#"[DATETIME_PART] is [DATE] [TIME] [TIMEZONE] separated by space in any order.
[DATE] is of format "yyyy-mm-dd" or "yyyy/mm/dd". Year is optional.
[TIME] is of format "HH:MM:SS [am|pm]": You can optionally suffixed with "am" or "pm", in which case it is parsed as 12-hour format and minute and second are optional. In the case where neither "am" nor "pm" are specified, it is parsed in 24-hour format and only second is optional.
[TIMEZONE] is one of the timezone defined in IANA timezone database or an alias. Alias which can be created by including `tz.txt` inside the program directory. See https://github.com/CarrieForle/tz-rust"#;

const AFTER_HELP: &str = 
r#"EXAMPLES:
11-12 20:00 utc
2023/12/10 8:25 America/Los_Angeles"#;