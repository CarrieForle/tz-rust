use chrono::{Local, TimeZone};
use either::Either;
use tz_rust::*;
use clap::{ValueEnum, Parser};
use std::{
    error::Error,
    collections::HashMap,
    fmt::Display,
};

#[derive(Parser)]
#[command(name = "tz")]
#[command(version = "0.1.0")]
#[command(about = "Timezone converter")]
pub struct Cli {
    #[arg(short, long)]
    #[arg(help = MODE_HELP)]
    #[arg(default_value_t = Mode::To)]
    pub mode: Mode,
    #[arg(value_name = "DATETIME_PARTS")]
    #[arg(help = "Datetime with timezone")]
    #[arg(long_help = DT_LONG_HELP)]
    pub dt: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    From,
    To,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::From => {
                "from"
            },
            Mode::To => {
                "to"
            }
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tz_abbr = read_tz_abbr("tz.txt").unwrap_or(HashMap::new());
    let cli = Cli::parse();

    if cli.dt.is_empty() {
        println!("{}", Local::now().format("%F %T %s"));
    } else {
        let dt = parse_dt_parts(&cli.dt, &tz_abbr);
        match dt {
            Ok((dt, Either::Left(tz))) => match cli.mode {
                Mode::From => {
                    let dt = tz.from_local_datetime(&dt)
                        .unwrap()
                        .with_timezone(&Local);
                    println!("{}", dt.format("%F %T %s"));
                }
                Mode::To => {
                    let dt = Local.from_local_datetime(&dt)
                        .unwrap()
                        .with_timezone(&tz);
                    println!("{}", dt.format("%F %T %Z %s"));
                }
            }
            Ok((dt, Either::Right(_))) => {
                println!("{}", dt.format("%F %T %s"));
            }
            Err(e) => {
                Err(e)?;
            }
        }
    }

    Ok(())
}

const MODE_HELP: &'static str = r#"Conversion direction.
- from: From timezone to localtime.
- to: From localtime to timezone.
"#;

const DT_LONG_HELP: &'static str = 
r#"[DATETIME_PART] is [DATE] [TIME] [TIMEZONE] separated by space in any order.
[DATE] is of format "yyyy-mm-dd". Year is optional.
[TIME] is of format "HH:MM:SS [am|pm]": You can optionally suffixed with "am" or "pm", in which case it is parsed as 12-hour format and minute and second are optional. In the case where neither "am" nor "pm" are specified, it is parsed in 24-hour format and only second is optional.
[TIMEZONE] is one of the timezone defined in IANA timezone database or an alias, which can be created by including `tz.txt` inside the program directory."#;