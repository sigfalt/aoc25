#![allow(unused_imports)]

use std::collections::BTreeMap;
use anyhow::*;
use itertools::Itertools;
use nom::character::complete::{char, line_ending, u64};
use nom::{IResult, Parser};
use nom::combinator::{all_consuming, opt};
use nom::multi::many1;
use ordered_float::NotNan;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
	x: u64,
	y: u64,
	z: u64,
}
impl Point {
	fn distance(&self, other: &Point) -> f64 {
		let (x_diff, y_diff, z_diff) = (
			self.x.abs_diff(other.x),
			self.y.abs_diff(other.y),
			self.z.abs_diff(other.z)
		);
		f64::sqrt(((x_diff * x_diff) + (y_diff * y_diff) + (z_diff * z_diff)) as f64)
	}
}

fn parse_point(input: &str) -> IResult<&str, Point> {
	(
		u64,
		char(','),
		u64,
		char(','),
		u64,
		opt(line_ending),
	).map(|(x, _, y, _, z, _)| Point{x, y, z}).parse(input)
}

fn parse(input: &str) -> Vec<Point> {
	let (_, res) = all_consuming(many1(parse_point)).parse(input).unwrap();
	
	res
}

pub fn part1(input: &str) -> Result<u64> {
	part1_internal(input, 1000)
}

fn part1_internal(input: &str, wires_to_connect: usize) -> Result<u64> {
	let points = parse(input);

	let mut distances = points.iter().enumerate().tuple_combinations::<(_, _)>()
		.map(|((a_ix, a), (b_ix, b))| (NotNan::new(a.distance(b)).unwrap(), (a_ix, b_ix)))
		.collect_vec();
	distances.sort_unstable_by_key(|(dist, _)| *dist);

	let mut circuit_ids = vec![None; points.len()];
	let mut circuit_sizes = vec![1; points.len()];
	for (_, (a, b)) in distances.into_iter().take(wires_to_connect) {
		let mut get_root = |ix: usize| -> usize {
			fn rec(ix: usize, circuit_ids: &mut Vec<Option<usize>>) -> usize {
				if let Some(root) = circuit_ids[ix] {
					let new_root = rec(root, circuit_ids);
					circuit_ids[ix] = Some(new_root);
					new_root
				} else {
					ix
				}
			}
			rec(ix, &mut circuit_ids)
		};
		
		let min_id = a.min(b);
		let max_id = a.max(b);
		// are these points already part of other circuits? get root points
		let min_root = get_root(min_id);
		let max_root = get_root(max_id);
		// if this has joined two circuits that were previously separate
		if min_root != max_root {
			// set the new root and circuit sizes
			circuit_ids[max_root] = Some(min_root);
			circuit_sizes[min_root] += circuit_sizes[max_root];
			circuit_sizes[max_root] = 0;
		}
	}
	
	circuit_sizes.sort_unstable();
	Ok(circuit_sizes.into_iter().rev().take(3).product())
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day08::*;

	const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(40, part1_internal(TEST, 10)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}