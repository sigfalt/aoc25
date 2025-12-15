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
	let device_output_list = parse(input);

	const START_NODE: &str = "svr";
	const DAC_NODE: &str = "dac";
	const FFT_NODE: &str = "fft";
	const TARGET_NODE: &str = "out";

	let device_output_index = device_output_list.iter().map(
		|OutputList(device, _)| device.clone()
	).chain(vec![Device(String::from(TARGET_NODE))]).collect_vec();

	let reverse_index_mapping = AHashMap::from_iter(device_output_index
		.iter().enumerate().map(|(ix, device)| {(device.clone(), ix)}));

	let device_output_map = device_output_list.into_iter()
		.chain(vec![OutputList(Device(String::from(TARGET_NODE)), vec![])])
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
	let dac_index = reverse_index_mapping.get(&Device(String::from(DAC_NODE))).unwrap();
	let fft_index = reverse_index_mapping.get(&Device(String::from(FFT_NODE))).unwrap();
	let target_index = reverse_index_mapping.get(&Device(String::from(TARGET_NODE))).unwrap();
	
	// want num of paths from start to target including both dac and fft (in any order)
	// count paths for subsets of the total desired path, then combine as appropriate
	
	// count(start -> dac/fft -> fft/dac -> target)
	// = count(start -> dac -> fft -> target) + count(start -> fft -> dac -> target)
	
	// count(start -> dac -> fft -> target)
	// = count(start -> dac) * count(dac -> fft) * count(fft -> target)

	let successors = |&curr_index: &&usize| device_output_map[*curr_index].iter();
	let equals = |target_index: &usize| {
		let target_index = target_index.clone();
		move |&&curr_index: &&usize| curr_index == target_index
	};
	
	let start_dac_paths = count_paths(start_index, successors, equals(dac_index));
	let dac_fft_paths = count_paths(dac_index, successors, equals(fft_index));
	let fft_target_paths = count_paths(fft_index, successors, equals(target_index));
	let start_dac_fft_paths = start_dac_paths * dac_fft_paths * fft_target_paths;
	
	let start_fft_paths = count_paths(start_index, successors, equals(fft_index));
	let fft_dac_paths = count_paths(fft_index, successors, equals(dac_index));
	let dac_target_paths = count_paths(dac_index, successors, equals(target_index));
	let start_fft_dac_paths = start_fft_paths * fft_dac_paths * dac_target_paths;

	Ok((start_dac_fft_paths + start_fft_dac_paths) as u64)
}

#[cfg(test)]
mod tests {
	use crate::day11::*;

	const TEST1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
	
	const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(5, part1(TEST1)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(2, part2(TEST2)?);
		Ok(())
	}
}