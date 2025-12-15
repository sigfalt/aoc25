#![allow(unused_imports)]

use ahash::AHashMap;
use anyhow::*;
use itertools::Itertools;
use nom::bytes::tag;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::{all_consuming, map, opt};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::Parser;
use nom::sequence::{separated_pair, terminated};
use pathfinding::prelude::count_paths;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Device(String);

#[derive(Debug, Clone)]
struct OutputList(Device, Vec<Device>);

fn parse_device(input: &str) -> IResult<&str, Device> {
	map(alpha1, |label| Device(String::from(label))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, OutputList> {
	map(
		terminated(
			separated_pair(
				parse_device, 
				tag(": "), 
				separated_list1(space1, parse_device)
			),
			opt(line_ending)
		), 
		|(device, outputs)| OutputList(device, outputs)
	).parse(input)
}

fn parse(input: &str) -> Vec<OutputList> {
	let (_, res) = all_consuming(many1(parse_line)).parse(input).unwrap();
	
	res
}

pub fn part1(input: &str) -> Result<u64> {
	let device_output_list = parse(input);

	const START_NODE: &str = "you";
	const TARGET_NODE: &str = "out";
	
	let device_output_index = device_output_list.iter().map(
		|OutputList(device, _)| device.clone()
	).chain(vec![Device(String::from(TARGET_NODE))]).collect_vec();
	
	let reverse_index_mapping = AHashMap::from_iter(device_output_index
		.iter().enumerate().map(|(ix, device)| {(device.clone(), ix)}));
	
	let device_output_map = device_output_list.into_iter()
		.map(|OutputList(device, mapping)| {
			let &device_ix = reverse_index_mapping.get(&device).unwrap();
			let output_ix_list = mapping.into_iter().map(|output_device|
				reverse_index_mapping.get(&output_device).unwrap().clone()
			).collect_vec();
			
			(device_ix, output_ix_list) 
		})
		.sorted_unstable_by_key(|&(ix, _)| ix)
		.map(|(_, output_list)| output_list)
		.collect_vec();
	
	let start_index = reverse_index_mapping.get(&Device(String::from(START_NODE))).unwrap();
	let target_index = reverse_index_mapping.get(&Device(String::from(TARGET_NODE))).unwrap();
	
	let num_paths = count_paths(
		start_index,
		|&curr_index| device_output_map[*curr_index].iter(),
		|&curr_index| curr_index == target_index
	);
	
	Ok(num_paths as u64)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day11::*;

	const TEST: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(5, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}