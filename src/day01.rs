#![allow(unused_imports)]

use anyhow::*;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::char;
use nom::Parser;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

#[derive(Clone, Copy, Debug)]
struct DialRotation(i64);

fn parse_u64(input: &str) -> IResult<&str, u64> {
	map_res(digit1, |num: &str| num.parse()).parse(input)
}

fn parse_rotation(input: &str) -> IResult<&str, DialRotation> {
	alt((
		preceded(char('L'), parse_u64).map(|num| DialRotation(-(num as i64))),
		preceded(char('R'), parse_u64).map(|num| DialRotation(num as i64))
	)).parse(input)
}

fn parse(input: &str) -> Vec<DialRotation> {
	let (_, res) = all_consuming(separated_list1(
		line_ending,
		parse_rotation
	)).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let rotations = parse(input);

	const DIAL_START: i64 = 50;
	const DIAL_SIZE: i64 = 100;

	let mut dial = DIAL_START;
	let mut dial_hit_zero_count = 0;
	for DialRotation(rotation) in rotations {
		dial = (dial + rotation) % DIAL_SIZE;
		if dial == 0 { dial_hit_zero_count += 1 }
	}

	Ok(dial_hit_zero_count)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day01::*;

	const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

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