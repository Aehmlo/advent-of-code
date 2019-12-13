use clap::{App, Arg};
use std::str::FromStr;

const ALL: &'static str = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25";

mod one;

fn input(day: u8) -> Option<String> {
    let path = format!("../inputs/{}.txt", day);
    std::fs::read_to_string(path).ok()
}

fn run(day: u8) -> Option<[String; 2]> {
    input(day).and_then(|input| match day {
        1 => one::run(input).into(),
        _ => None,
    })
}

fn main() {
    let matches = App::new("aoc-2019")
        .arg(
            Arg::with_name("days")
                .short("d")
                .long("day")
                .takes_value(true),
        )
        .get_matches();
    matches
        .value_of("days")
        .unwrap_or(&ALL)
        .split(",")
        .map(u8::from_str)
        .map(Result::unwrap)
        .map(|day| (day, run(day)))
        .for_each(|(day, result)| {
            if let Some(result) = result {
                println!("Day {}", day);
                println!("Part 1: {}", result[0]);
                println!("Part 2: {}", result[1]);
            }
        });
}
