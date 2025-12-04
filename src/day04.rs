#![allow(unused_imports)]

use std::collections::VecDeque;
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
			if let Some(Cell::PaperRoll) = warehouse_map.get(row_offset.strict_add_unsigned(row), col_offset.strict_add_unsigned(col)) {
				adjacent_papers += 1;
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
	let mut warehouse_map = parse(input);

	// locate all paper rolls which must be checked for forklift accessibility
	let mut locations_to_check = warehouse_map.indexed_iter().filter_map(|(grid_coords, &cell)| {
		match cell {
			Cell::Empty => None,
			Cell::PaperRoll => Some(grid_coords),
		}
	}).collect::<VecDeque<_>>();

	let mut accessible_rolls_of_paper = 0;
	while let Some((row, col)) = locations_to_check.pop_front() {
		// double check that this location still contains a roll of paper,
		// as it may have been removed already
		if *warehouse_map.get(row, col).unwrap() != Cell::PaperRoll {
			continue;
		}

		// cache adjacent paper roll coordinates in case this roll is removed
		// initialize with capacity 4 as once the fourth roll is inserted,
		// this roll is inaccessible anyway and will not be removed
		let mut adjacent_papers = Vec::with_capacity(4);
		let mut adjacent_papers_count = 0;
		for (row_offset, col_offset) in Direction::iter().map(|d| d.get_offset()) {
			let adj_row = row_offset.strict_add_unsigned(row);
			let adj_col = col_offset.strict_add_unsigned(col);
			if let Some(Cell::PaperRoll) = warehouse_map.get(adj_row, adj_col) {
				adjacent_papers_count += 1;
				adjacent_papers.push((adj_row as usize, adj_col as usize));
			}
			if adjacent_papers_count >= 4 {
				break;
			}
		}

		if adjacent_papers_count < 4 {
			accessible_rolls_of_paper += 1;
			// remove this accessible roll of paper from the warehouse
			*warehouse_map.get_mut(row, col).unwrap() = Cell::Empty;
			// queue neighboring rolls of paper to be checked for accessibility again
			locations_to_check.append(&mut adjacent_papers.into());
		}
	}

	Ok(accessible_rolls_of_paper)
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
		assert_eq!(43, part2(TEST)?);
		Ok(())
	}
}