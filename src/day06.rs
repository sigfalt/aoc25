#![allow(unused_imports)]
use nom::Parser;
use anyhow::*;
use grid::Grid;
use itertools::{izip, Itertools};
use nom::branch::alt;
use nom::bytes::complete::take_until1;
use nom::character::complete::{char, line_ending, satisfy, space0, space1, u64};
use nom::combinator::{all_consuming, opt, value};
use nom::IResult;
use nom::multi::{many1, many1_count};
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

fn parse_part1(input: &str) -> (Vec<Vec<u64>>, Vec<MathOp>) {
	let (_, res) = all_consuming(pair(
		many1(parse_number_row),
		parse_operation_row
	)).parse(input).unwrap();

	res
}

pub fn part1(input: &str) -> Result<u64> {
	let (number_rows, operation_row) = parse_part1(input);

	let number_grid = Grid::from(number_rows);
	let res = izip!(number_grid.iter_cols(), operation_row).map(|(col_nums, op)| {
		match op {
			MathOp::Add => col_nums.sum::<u64>(),
			MathOp::Mul => col_nums.product(),
		}
	}).sum();

	Ok(res)
}

fn parse_digit_line(input: &str) -> IResult<&str, Vec<char>> {
	terminated(
		many1(alt((
			satisfy(nom::AsChar::is_space),
			satisfy(nom::AsChar::is_dec_digit)
		))),
		line_ending
	).parse(input)
}

fn parse_operation_line(input: &str) -> IResult<&str, Vec<(MathOp, usize)>> {
	terminated(
		many1(pair(
			alt((
				value(MathOp::Add, char('+')),
				value(MathOp::Mul, char('*'))
			)),
			many1_count(char(' '))
		)),
		opt(line_ending)
	).parse(input)
}

fn parse_part2(input: &str) -> (Vec<Vec<char>>, Vec<(MathOp, usize)>) {
	let (_, res) = all_consuming(pair(
		many1(parse_digit_line),
		parse_operation_line
	)).parse(input).unwrap();

	res
}

pub fn part2(input: &str) -> Result<u64> {
	let (digit_rows, sized_ops) = parse_part2(input);
	
	let digit_grid = Grid::from(digit_rows);
	let mut digit_grid = digit_grid.iter_cols().rev();
	
	let mut sized_ops = sized_ops.into_iter().rev();
	
	let mut process_sized_op = |(op, operand_size)| -> u64 {
		let operands = digit_grid.by_ref().take(operand_size)
			.map(|digits| {
				let digit_str = digits.collect::<String>();
				digit_str.trim().parse::<u64>().unwrap()
			});
		let problem_solution = match op {
			MathOp::Add => operands.sum::<u64>(),
			MathOp::Mul => operands.product(),
		};
		digit_grid.next();
		problem_solution
	};

	let mut worksheet_sum = 0;
	
	let (first_op, first_op_size) = sized_ops.next().unwrap();
	worksheet_sum += process_sized_op((first_op, first_op_size + 1));
	
	worksheet_sum += sized_ops.map(|sized_op| process_sized_op(sized_op)).sum::<u64>();
	
	Ok(worksheet_sum)
}

#[cfg(test)]
mod tests {
	use crate::day06::*;

	const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(4277556, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(3263827, part2(TEST)?);
		Ok(())
	}
}