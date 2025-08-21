use chrono::Local;
use chrono::TimeZone;
use either::Either;
use tz_rust::*;
use clap::Parser;
use std::error::Error;
use std::collections::HashMap;
use std::fmt::Display;
use clap::ValueEnum;

#[derive(Parser)]
#[command(name = "tz")]
#[command(version = "0.1.0")]
#[command(about = "Timezone converter")]
pub struct Cli {
    #[arg(short, long, default_value_t = Mode::To)]
    pub mode: Mode,
    #[arg(value_name = "DATETIME_PARTS")]
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