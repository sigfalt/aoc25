#![allow(unused_imports)]

use std::ops::RangeInclusive;
use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{char, line_ending, u64};
use nom::combinator::{all_consuming, map_res, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::Parser;
use nom::sequence::{separated_pair, terminated};
use num::Integer;

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
	separated_pair(u64, char('-'), u64)
		.map(|(start, end)| start..=end)
		.parse(input)
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
	let (_, res) = all_consuming(
		terminated(
			separated_list1(
				char(','),
				parse_range
			),
			opt(line_ending)
		)
	).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let ranges = parse(input);

	let mut invalid_id_sum = 0;
	for range in ranges {
		let range_start = *range.start();
		let range_end = *range.end();
		let range_start_string = range_start.to_string();

		let start_len = range_start_string.len();
		let invalid_start = if start_len.is_even() {
			// number can be evenly chopped into two lexicographic halves
			String::from(&range_start_string[..(start_len/2)])
		} else {
			// only numbers with an even amount of digits can be invalid
			// the smallest number that would be greater than the range start...
			// would be 1 followed by (start_len/2) number of 0's
			String::from("1") + &String::from("0").repeat(start_len/2)
		};

		// may start checking below the range start, incorrectly leading to an early exit
		let mut invalid_prefix = if invalid_start.repeat(2).parse::<u64>().unwrap() < range_start {
			(invalid_start.parse::<u64>().unwrap() + 1).to_string()
		} else {
			invalid_start
		};

		loop {
			let invalid_num = invalid_prefix.repeat(2).parse::<u64>().unwrap();
			if invalid_num > range_end {
				break;
			}
			invalid_id_sum += invalid_num;
			invalid_prefix = (invalid_prefix.parse::<u64>().unwrap() + 1).to_string();
		}
	}

	Ok(invalid_id_sum)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day02::*;

	const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(1227775554, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}