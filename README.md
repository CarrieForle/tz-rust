# rust-tz

**A** timezone conversion CLI tool.

This is a rewrite of my [old tools](https://github.com/CarrieForle/time-utility).

# Usage

```sh
$ tz --help
```

```
Timezone converter

Usage: tz.exe [OPTIONS] [DATETIME_PARTS]...

Arguments:
  [DATETIME_PARTS]...
          [DATETIME_PART] is [DATE] [TIME] [TIMEZONE] separated by space in any order.
          [DATE] is of format "yyyy-mm-dd" or "yyyy/mm/dd". Year is optional.
          [TIME] is of format "HH:MM:SS [am|pm]": You can optionally suffixed with "am" or "pm", in which case it is parsed as 12-hour format and minute and second are optional. In the case where neither "am" nor "pm" are specified, it is parsed in 24-hour format and only second is optional.
          [TIMEZONE] is one of the timezone defined in IANA timezone database or an alias. Alias which can be created by including `tz.txt` inside the program directory. See https://github.com/CarrieForle/tz-rust

Options:
  -m, --mode <MODE>
          Possible values:
          - from: From timezone to localtime
          - to:   From localtime to timezone
          - ts:   Get timestamp[default: to]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

EXAMPLES:
11-12 20:00 utc
2023/12/10 8:25 America/Los_Angeles
```

## Timezone Abbreviation

Due to timezone abbreviation is not unique (For example: CST can be Central Standard Time or China Standard Time), this is optionally supported by including `tz.txt` file, where you can define alias (and thus abbreviation) of timezones. See the sample in the repo.

# Download

Only binary for Windows 64-bit are provided. For other operating system you need to build from the source code.

# Build

Rust 2024 edition and cargo are required.

```sh
cargo build -r
```