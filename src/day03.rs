#![allow(unused_imports)]

use std::cmp::max;
use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{line_ending, satisfy};
use nom::{IResult, Parser};
use nom::combinator::{all_consuming, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Joltage(u32);

#[derive(Clone, Debug)]
struct BatteryBank(Vec<Joltage>);

fn parse_joltage(input: &str) -> IResult<&str, Joltage> {
	satisfy(nom::AsChar::is_dec_digit)
		.map_opt(|chr| chr.to_digit(10))
		.map(|num| Joltage(num))
		.parse(input)
}

fn parse_battery_bank(input: &str) -> IResult<&str, BatteryBank> {
	many1(parse_joltage)
		.map(|bank| BatteryBank(bank))
		.parse(input)
}

fn parse(input: &str) -> Vec<BatteryBank> {
	let (_, res) = all_consuming(terminated(
		separated_list1(
			line_ending,
			parse_battery_bank
		),
		opt(line_ending)
	)).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let battery_banks = parse(input);

	let total_joltage: u32 = battery_banks.into_iter().map(|BatteryBank(bank)| {
		// find max joltage in the bank, excluding the last battery
		// short circuit for the first 9 joltage battery found
		let (mut max_ix, mut max_joltage) = (0, 0);
		for (ix, &Joltage(j)) in bank[..(bank.len() - 1)].iter().enumerate() {
			if j > max_joltage {
				max_ix = ix;
				max_joltage = j;
			}
			if max_joltage == 9 {
				break;
			}
		}
		(max_joltage * 10) + bank[(max_ix + 1)..].into_iter().max().map(|Joltage(j)| j).unwrap()
	}).sum();

	Ok(total_joltage as u64)
}

pub fn part2(input: &str) -> Result<u64> {
	let battery_banks = parse(input);

	const BATTERIES_PER_BANK: usize = 12;

	let total_joltage = battery_banks.into_iter()
		.map(|BatteryBank(bank)| get_max_joltage(&bank, BATTERIES_PER_BANK))
		.sum();

	Ok(total_joltage)
}

fn get_max_joltage(
	battery_bank: &[Joltage],
	batteries_to_use: usize,
) -> u64 {
	fn get_max_joltage_rec(
		remaining_bank: &[Joltage],
		remaining_batteries: usize,
		accumulated_joltage: u64
	) -> u64 {
		if remaining_batteries == 0 {
			return accumulated_joltage;
		}

		let selectable_bank = &remaining_bank[..(remaining_bank.len() - remaining_batteries + 1)];
		let (mut max_ix, mut max_joltage) = (0, 0);
		for (ix, &Joltage(j)) in selectable_bank.iter().enumerate() {
			if j > max_joltage {
				max_ix = ix;
				max_joltage = j;
			}
			if max_joltage == 9 {
				break;
			}
		}

		get_max_joltage_rec(
			&remaining_bank[(max_ix + 1)..],
			remaining_batteries - 1,
			(accumulated_joltage * 10) + max_joltage as u64
		)
	}

	get_max_joltage_rec(battery_bank, batteries_to_use, 0)
}

#[cfg(test)]
mod tests {
	use crate::day03::*;

	const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(357, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(3121910778619, part2(TEST)?);
		Ok(())
	}
}