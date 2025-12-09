#![allow(unused_imports)]
use anyhow::*;
use geo::coord;
use geo::geometry::{Polygon, Rect};
use itertools::Itertools;
use nom::character::complete::{char, line_ending, u64};
use nom::combinator::{all_consuming, opt};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::Parser;
use geo::prelude::*;
use num::ToPrimitive;
use ordered_float::NotNan;

#[derive(Debug, Copy, Clone)]
struct Point(u64, u64);
impl Point {
	fn area(&self, other: &Point) -> u64 {
		let Point(self_x, self_y) = self;
		let Point(other_x, other_y) = other;
		(self_x.abs_diff(*other_x) + 1) * (self_y.abs_diff(*other_y) + 1)
	}
}

fn parse_point(input: &str) -> IResult<&str, Point> {
	terminated(
		separated_pair(u64, char(','), u64),
		opt(line_ending)
	).map(|(a, b)| Point(a, b)).parse(input)
}

fn parse(input: &str) -> Vec<Point> {
	let (_, res) = all_consuming(many1(parse_point)).parse(input).unwrap();
	
	res
}

pub fn part1(input: &str) -> Result<u64> {
	let red_tiles = parse(input);
	
	let res = red_tiles.iter().tuple_combinations().map(|(a, b)| a.area(b)).max().unwrap();
	
	Ok(res)
}

pub fn part2(input: &str) -> Result<u64> {
	let red_tiles = parse(input);
	let red_tile_coords = red_tiles.into_iter()
		.map(|Point(x, y)| coord! {x: x as f64, y: y as f64})
		.collect_vec();
	
	let red_polygon = Polygon::new(
		red_tile_coords.clone().into(),
		vec![]
	);
	
	let res = red_tile_coords.iter().tuple_combinations()
		.filter_map(|(&a, &b)| {
			let rect = Rect::new(a, b);
			if red_polygon.covers(&rect) {
				Some((rect.height().to_u64().unwrap() + 1) * (rect.width().to_u64().unwrap() + 1))
			} else {
				None
			}
		}).max().unwrap();
	
	Ok(res)
}

#[cfg(test)]
mod tests {
	use crate::day09::*;

	const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(50, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(24, part2(TEST)?);
		Ok(())
	}
}