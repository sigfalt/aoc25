#![allow(unused_imports)]

use std::collections::VecDeque;
use ahash::AHashSet;
use anyhow::*;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending, u64, usize};
use nom::combinator::{all_consuming, opt, value};
use nom::{IResult, Parser};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct IndicatorLights(AHashSet<usize>);
impl IndicatorLights {
	fn apply_button(&self, button: &ButtonSchematic) -> Self {
		Self(&self.0 ^ &button.0)
	}
	
	// fn is_off(&self) -> bool {
	// 	self.0.is_empty()
	// }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ButtonSchematic(AHashSet<usize>);

#[derive(Debug, Clone, Eq, PartialEq)]
struct JoltageRequirements(Vec<u64>);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Machine(IndicatorLights, Vec<ButtonSchematic>, JoltageRequirements);

fn parse_indicator_lights(input: &str) -> IResult<&str, IndicatorLights> {
	delimited(
		char('['),
		many1(alt((
			value(false, char('.')),
			value(true, char('#'))
		))),
		char(']')
	).map(|lights|
		IndicatorLights(lights.into_iter().enumerate().filter_map(|(ix, state)| 
			if state { Some(ix) } else { None }
		).collect())
	).parse(input)
}

fn parse_button_schematic(input: &str) -> IResult<&str, ButtonSchematic> {
	delimited(
		char('('),
		separated_list1(char(','), usize),
		char(')')
	).map(|wires| ButtonSchematic(AHashSet::from_iter(wires))).parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, JoltageRequirements> {
	delimited(
		char('{'),
		separated_list1(char(','), u64),
		char('}')
	).map(|joltages| JoltageRequirements(joltages)).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
	(
		parse_indicator_lights,
		many1(preceded(
			char(' '),
			parse_button_schematic
		)),
		delimited(
			char(' '),
			parse_joltage,
			opt(line_ending)
		)
	).map(|(indicators, buttons, joltage)|
		Machine(indicators, buttons, joltage)
	).parse(input)
}

fn parse(input: &str) -> Vec<Machine> {
	let (_, res) = all_consuming(many1(parse_machine)).parse(input).unwrap();
	
	res
}

pub fn part1(input: &str) -> Result<u64> {
	let machines = parse(input);
	
	let button_presses = machines.into_iter().map(|Machine(target_lights, buttons, _)| {
		let mut search_nodes = VecDeque::from([(IndicatorLights::default(), AHashSet::<usize>::with_capacity(buttons.len()))]);
		while let Some((current_lights, pressed_buttons)) = search_nodes.pop_front() {
			for (button_ix, unpressed_button) in buttons.iter().enumerate().filter(|(ix, _)| !pressed_buttons.contains(ix)) {
				let new_lights = current_lights.apply_button(unpressed_button);
				if new_lights == target_lights {
					return pressed_buttons.len() as u64 + 1;
				}
				let mut new_buttons = pressed_buttons.clone();
				new_buttons.insert(button_ix);
				search_nodes.push_back((new_lights, new_buttons))
			}
		}
		panic!("no solution found after pressing all buttons");
	}).sum();
	
	Ok(button_presses)
}

pub fn part2(input: &str) -> Result<u64> {
	let _ = input;
	Ok(0)
}

#[cfg(test)]
mod tests {
	use crate::day10::*;

	const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

	#[test]
	fn test_part_one() -> Result<()> {
		assert_eq!(7, part1(TEST)?);
		Ok(())
	}

	#[test]
	fn test_part_two() -> Result<()> {
		assert_eq!(0, part2(TEST)?);
		Ok(())
	}
}