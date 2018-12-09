extern crate clap;
use clap::{App, Arg};

extern crate advent_of_code as advent;

use advent::five;
use advent::four;
use advent::one;
use advent::six;
use advent::three;
use advent::two;

const LATEST: usize = 6;

fn solve(day: usize) {
    match day {
        1 => one::solve(),
        2 => two::solve(),
        3 => three::solve(),
        4 => four::solve(),
        5 => five::solve(),
        6 => six::solve(),
        _ => unimplemented!(),
    }
}

fn main() {
    let matches = App::new("advent")
        .version("0.1.0")
        .about("Advent of Code 2018.")
        .author("Alex Hamilton")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("days")
                .value_name("DAY")
                .help("Specifies the day for which to run the solution.")
                .takes_value(true)
                .multiple(true),
        ).get_matches();
    let days = matches.values_of("day").map(|i| i.collect::<Vec<_>>());
    if days.is_some() {
        for day in days.unwrap() {
            solve(day.parse().expect("Failed to parse day as integer."));
        }
    } else {
        for day in 1..=LATEST {
            solve(day);
        }
    }
}
