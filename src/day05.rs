#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use ahash::AHashMap;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FreshnessChange {
	Start,
	End
}

pub fn part2(input: &str) -> Result<u64> {
	let (freshness_ranges, _) = parse(input);

	let mut consolidated_fresh_ranges = BTreeMap::new();
	freshness_ranges.iter().flat_map(|fresh_range| vec![
		(fresh_range.start(), FreshnessChange::Start),
		(fresh_range.end(), FreshnessChange::End)
	]).for_each(|(ingredient_id, freshness)| {
		consolidated_fresh_ranges.entry(ingredient_id)
			.or_insert(vec![])
			.push(freshness);
	});

	let mut fresh_ingredient_id_count = 0;
	let mut active_freshness_ranges = 0;
	let mut freshness_start_id = None;
	consolidated_fresh_ranges.into_iter().for_each(|(ingredient_id, freshness_changes)| {
		let was_fresh = active_freshness_ranges > 0;
		active_freshness_ranges += freshness_changes.iter().map(|freshness| {
			match freshness {
				FreshnessChange::Start => 1,
				FreshnessChange::End => -1,
			}
		}).sum::<i32>();
		if was_fresh {
			if active_freshness_ranges == 0 {
				assert_ne!(freshness_start_id, None);
				fresh_ingredient_id_count += ingredient_id - freshness_start_id.unwrap() + 1;
				freshness_start_id = None;
			}
		} else {
			if active_freshness_ranges > 0 {
				assert_eq!(freshness_start_id, None);
				freshness_start_id = Some(ingredient_id);
			} else {
				// wasn't fresh before but still isn't fresh now
				// range of 1
				fresh_ingredient_id_count += 1;
			}
		}
	});

	Ok(fresh_ingredient_id_count)
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
		assert_eq!(14, part2(TEST)?);
		Ok(())
	}
}