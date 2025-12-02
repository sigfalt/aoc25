use std::fs;
use anyhow::*;

pub mod day01;
pub mod day02;

pub fn start_day(day: &str) -> Result<String> {
	println!("Advent of Code 2025 - Day {:0>2}", day);

	Ok(fs::read_to_string(format!("input/{}.txt", day))?)
}