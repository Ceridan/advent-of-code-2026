use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

const YEAR: u16 = 2026;

fn main() {
    let day_arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Day should be provided as an argument.");
        process::exit(1);
    });

    let day_num: u8 = day_arg.parse().unwrap_or_else(|_| {
        eprintln!("Invalid day: {day_arg}");
        process::exit(1);
    });

    let day_padded = format!("{day_num:02}");
    let code_file = format!("src/bin/day{day_padded}.rs");
    let input_file = format!("inputs/day{day_padded}.txt");

    if Path::new(&code_file).exists() {
        eprintln!("File already exists: {code_file}");
        process::exit(1);
    }

    let session = env::var("AOC_SESSION").unwrap_or_else(|_| {
        eprintln!("AOC_SESSION environment variable is not set.");
        process::exit(1);
    });

    // Download input.
    let url = format!("https://adventofcode.com/{YEAR}/day/{day_num}/input");
    let mut response = ureq::get(&url)
        .header("Cookie", &format!("session={session}"))
        .call()
        .unwrap_or_else(|err| {
            eprintln!("Cannot download input: {err}");
            process::exit(1);
        });

    let input = response.body_mut().read_to_string().unwrap_or_else(|err| {
        eprintln!("Error reading response body: {err}");
        process::exit(1);
    });

    // Create directories if missing.
    fs::create_dir_all("src/bin").unwrap_or_else(|err| {
        eprintln!("Error creating src/bin directory: {err}");
        process::exit(1);
    });
    fs::create_dir_all("inputs").unwrap_or_else(|err| {
        eprintln!("Error creating inputs directory: {err}");
        process::exit(1);
    });

    // Code file template.
    let code_template = format!(
        r#"use advent_of_code_2026::io;

fn part1(_input: &str) -> i64 {{
    0
}}

fn part2(_input: &str) -> i64 {{
    0
}}

fn main() {{
    let input = io::read_text("inputs/day{day_padded}.txt");
    println!("Day {day_padded}, part 1: {{}}", part1(&input));
    println!("Day {day_padded}, part 2: {{}}", part2(&input));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const SAMPLE: &str = "\
line1
line2
";

    #[test]
    fn test_part1() {{
        assert_eq!(part1(SAMPLE), 0);
    }}

    #[test]
    fn test_part2() {{
        assert_eq!(part2(SAMPLE), 0);
    }}
}}
"#
    );

    fs::write(&code_file, code_template).unwrap_or_else(|err| {
        eprintln!("Error writing {code_file}: {err}");
        process::exit(1);
    });

    fs::write(&input_file, input).unwrap_or_else(|err| {
        eprintln!("Error writing {input_file}: {err}");
        process::exit(1);
    });

    // Add to git.
    let status = Command::new("git")
        .args(["add", &code_file, &input_file])
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Error while executing \"git add\": {err}");
            process::exit(1);
        });

    if !status.success() {
        eprintln!("Error while executing \"git add\"");
        process::exit(1);
    }

    println!("Done.");
}
