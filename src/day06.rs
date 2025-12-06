#![allow(unused_imports)]
use nom::Parser;
use anyhow::*;
use grid::Grid;
use itertools::{izip, Itertools};
use nom::branch::alt;
use nom::character::complete::{char, line_ending, space0, space1, u64};
use nom::combinator::{all_consuming, opt, value};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{pair, preceded, terminated};

fn parse_number_row(input: &str) -> IResult<&str, Vec<u64>> {
	terminated(
		many1(preceded(space0, u64)),
		pair(space0, line_ending)
	).parse(input)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum MathOp {
	Add,
	Mul
}

fn parse_operation_row(input: &str) -> IResult<&str, Vec<MathOp>> {
	terminated(
		many1(terminated(
			alt((
				value(MathOp::Add, char('+')),
				value(MathOp::Mul, char('*'))
			)),
			space0
		)),
		opt(line_ending)
	).parse(input)
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<MathOp>) {
	let (_, res) = all_consuming(pair(
		many1(parse_number_row),
		parse_operation_row
	)).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let (number_rows, operation_row) = parse(input);

	let number_grid = Grid::from(number_rows);
	let res = izip!(number_grid.iter_cols(), operation_row).map(|(col_nums, op)| {
		match op {
			MathOp::Add => col_nums.sum::<u64>(),
			MathOp::Mul => col_nums.product(),
		}
	}).sum();

	Ok(res)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day06::*;

	const TEST: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(4277556, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}