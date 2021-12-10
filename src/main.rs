mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    env, fs,
    path::Path,
    process::{self, exit},
};

const YEAR: i32 = 2021;
static COOKIE_ENV_VAR_NAME: &str = "AOC_SESSION_ID";

#[derive(Eq, Hash, PartialEq)]
struct Problem {
    day: i32,
    part: i32,
}

type PartFunc = fn(&str) -> Result<String>;

/// Hacky macro to slightly simplify the process of importing and using the file
/// for a new day's problem.
macro_rules! add_day {
    ($hm:ident, $module:ident) => {
        $hm.insert(
            stringify!($module)
                .strip_prefix("day")
                .unwrap()
                .parse()
                .unwrap(),
            ($module::part1, $module::part2),
        )
    };
}

fn main() {
    match main_impl() {
        Ok(s) => println!("{}", s),
        Err(err) => {
            println!("{}", err.to_string());
            exit(1);
        }
    }
}

fn main_impl() -> Result<String> {
    let mut methods: HashMap<i32, (PartFunc, PartFunc)> = HashMap::new();
    add_day!(methods, day01);
    add_day!(methods, day02);
    add_day!(methods, day03);
    add_day!(methods, day04);
    add_day!(methods, day05);
    add_day!(methods, day06);
    add_day!(methods, day07);
    add_day!(methods, day08);
    add_day!(methods, day09);

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: aoc <day> <part>");
        process::exit(1);
    }
    let day: i32 = args[1].parse().unwrap();
    let parts = methods.get(&day).expect("unimplemented day");
    let part: i32 = args[2].parse().unwrap();
    let meth = if part == 1 {
        parts.0
    } else if part == 2 {
        parts.1
    } else {
        return Err(anyhow::anyhow!("invalid part {}", part));
    };

    let input = get_input(day).context("failed to load input")?;
    meth(&input)
}

fn get_input(day: i32) -> Result<String> {
    let cache_dir = Path::new("./.cache");
    if !cache_dir.exists() {
        fs::create_dir(cache_dir)?;
    }
    let day_file = cache_dir.join(format!("day{:02}.txt", day));
    if day_file.exists() {
        return fs::read_to_string(day_file).context("failed to write to cache");
    }
    let text = download_input(day)?;
    std::fs::write(day_file, text.as_str())?;
    Ok(text)
}

fn download_input(day: i32) -> Result<String> {
    let cookie =
        env::var(COOKIE_ENV_VAR_NAME).context(format!("${} must be set", COOKIE_ENV_VAR_NAME))?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let resp = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", cookie))
        .send()?
        .error_for_status()?;
    resp.text().context("failed to decode response")
}
