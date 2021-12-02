use std::{collections::HashMap, env, fs, path::Path, process};

const YEAR: i32 = 2021;
static COOKIE_ENV_VAR_NAME: &str = "AOC_SESSION_ID";

#[derive(Eq, Hash, PartialEq)]
struct Problem {
    day: i32,
    part: i32,
}

fn main() {
    let mut methods: HashMap<Problem, fn(String) -> Result<String, String>> = HashMap::new();
    methods.insert(Problem { day: 1, part: 1 }, day01_part1);

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: aoc <day> <part>");
        process::exit(1);
    }
    let prob = Problem {
        day: args[1].parse().unwrap(),
        part: args[2].parse().unwrap(),
    };
    let meth = methods.get(&prob).expect("unimplemented day");

    let input = get_input(prob.day).unwrap();
    let result = meth(input).unwrap();
    println!("{}", result);
}

fn get_input(day: i32) -> Result<String, String> {
    let cache_dir = Path::new("./.cache");
    if !cache_dir.exists() {
        fs::create_dir(cache_dir).unwrap();
    }
    let day_file = cache_dir.join(format!("day{:02}.txt", day));
    if day_file.exists() {
        return Ok(fs::read_to_string(day_file).unwrap());
    }
    let text = download_input(day).unwrap();
    std::fs::write(day_file, text.as_str()).unwrap();
    Ok(text)
}

fn download_input(day: i32) -> Result<String, String> {
    let cookie = env::var(COOKIE_ENV_VAR_NAME).unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let resp = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", cookie))
        .send()
        .unwrap();
    match resp.error_for_status() {
        Err(e) => Err(format!("{}", e.status().unwrap().to_string())),
        Ok(r) => Ok(r.text().unwrap()),
    }
}

fn day01_part1(input: String) -> Result<String, String> {
    Ok(input)
}
