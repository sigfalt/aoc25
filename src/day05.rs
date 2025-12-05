#![allow(unused_imports)]

use std::ops::RangeInclusive;
use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{char, line_ending, u64};
use nom::{IResult, Parser};
use nom::combinator::{all_consuming, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
	separated_pair(u64, char('-'), u64)
		.map(|(start, end)| start..=end)
		.parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
	many1(terminated(parse_range, line_ending)).parse(input)
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<u64>> {
	many1(
		terminated(
			u64,
			opt(line_ending)
		)
	).parse(input)
}

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
	let (_, res) = all_consuming(
		separated_pair(
			parse_ranges,
			line_ending,
			parse_ingredients
		)
	).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let (freshness_ranges, ingredients) = parse(input);

	let fresh_ingredients = ingredients.iter()
		.filter(|&ingredient|
			freshness_ranges.iter().any(|range| range.contains(ingredient))
		).count();

	Ok(fresh_ingredients as u64)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day05::*;

	const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(3, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}