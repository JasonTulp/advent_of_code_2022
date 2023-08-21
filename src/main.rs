use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

mod types;
// use types::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day = 1;
    if args.len() > 1 {
        day = args[1].parse().unwrap();
    }
    day_selector(day);
}

/// Selector to choose which day to execute
fn day_selector(day: u8) {
    match day {
        1 => day1::calculate_calories(),
        2 => day2::rock_paper_scissors(),
        3 => day3::rucksack_organisation(),
        4 => day4::camp_cleanup(),
        5 => day5::supply_stacks(),
        6 => day6::find_packet_marker(),
        _ => ()
    }
}

/// Helper function to read a file
pub fn read_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

