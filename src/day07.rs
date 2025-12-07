#![allow(unused_imports)]

use ahash::AHashSet;
use anyhow::*;
use grid::Grid;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, opt, value};
use nom::{IResult, Parser};
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
	Start,
	Splitter,
	Empty
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
	alt((
		value(Cell::Start, char('S')),
		value(Cell::Splitter, char('^')),
		value(Cell::Empty, char('.')),
	)).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
	terminated(
		many1(parse_cell),
		opt(line_ending)
	).parse(input)
}

fn parse(input: &str) -> Grid<Cell> {
	let (_, res) = all_consuming(many1(parse_line)).parse(input).unwrap();
	
	Grid::from(res)
}

pub fn part1(input: &str) -> Result<u64> {
	let grid = parse(input);
	
	let mut beam_splits = 0;
	let mut active_beams = AHashSet::new();
	for row in grid.iter_rows() {
		let mut new_beams = AHashSet::new();
		for (index, &cell) in row.enumerate() {
			match cell {
				Cell::Start => {
					new_beams.insert(index);
				},
				Cell::Splitter => {
					if active_beams.remove(&index) {
						new_beams.insert(index - 1);
						new_beams.insert(index + 1);
						beam_splits += 1;
					}
				},
				Cell::Empty => {},
			};
			active_beams = active_beams.union(&new_beams).cloned().collect();
		}
	}
	
	Ok(beam_splits)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day07::*;

	const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(21, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}