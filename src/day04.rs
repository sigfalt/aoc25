#![allow(unused_imports)]
use anyhow::*;
use grid::Grid;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, opt, value};
use nom::{IResult, Parser};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
	Empty,
	PaperRoll
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
	North,
	Northeast,
	East,
	Southeast,
	South,
	Southwest,
	West,
	Northwest
}
impl Direction {
	pub fn get_offset(&self) -> (isize, isize) {
		match self {
			Direction::North => (0, -1),
			Direction::Northeast => (1, -1),
			Direction::East => (1, 0),
			Direction::Southeast => (1, 1),
			Direction::South => (0, 1),
			Direction::Southwest => (-1, 1),
			Direction::West => (-1, 0),
			Direction::Northwest => (-1, -1),
		}
	}
	pub fn iter() -> impl Iterator<Item = Direction> {
		[
			Direction::North,
			Direction::Northeast,
			Direction::East,
			Direction::Southeast,
			Direction::South,
			Direction::Southwest,
			Direction::West,
			Direction::Northwest
		].into_iter()
	}
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
	alt((
		value(Cell::Empty, char('.')),
		value(Cell::PaperRoll, char('@'))
	)).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Cell>> {
	terminated(
		many1(parse_cell),
		opt(line_ending)
	).parse(input)
}

fn parse(input: &str) -> Grid<Cell> {
	let (_, res) = all_consuming(many1(parse_row)).parse(input).unwrap();

	Grid::from(res)
}

pub fn part1(input: &str) -> Result<u64> {
	let warehouse_map = parse(input);

	let accessible_rolls_of_paper = warehouse_map.indexed_iter().map(|((row, col), &cell)| {
		if cell != Cell::PaperRoll {
			return 0;
		}

		// a roll of paper is accessible if:
		// there are FEWER than FOUR rolls of paper in the eight adjacent cells
		let mut adjacent_papers = 0;
		for (row_offset, col_offset) in Direction::iter().map(|d| d.get_offset()) {
			match warehouse_map.get(row_offset.strict_add_unsigned(row), col_offset.strict_add_unsigned(col)) {
				Some(&Cell::PaperRoll) => adjacent_papers += 1,
				_ => {}
			}
			if adjacent_papers >= 4 {
				break;
			}
		}

		if adjacent_papers < 4 { 1 } else { 0 }
	}).sum();

	Ok(accessible_rolls_of_paper)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day04::*;

	const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(13, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}