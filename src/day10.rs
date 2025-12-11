#![allow(unused_imports, unused)]

use std::collections::VecDeque;
use std::iter::{once, repeat, zip};
use std::ops::{Add, Mul};
use ahash::AHashSet;
use anyhow::*;
use itertools::{chain, Itertools};
use nom::branch::alt;
use nom::character::complete::{char, line_ending, u64, usize};
use nom::combinator::{all_consuming, opt, value};
use nom::{IResult, Parser};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use pathfinding::prelude::astar;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct IndicatorLights(AHashSet<usize>);
impl IndicatorLights {
	fn apply_button(&self, button: &ButtonSchematic) -> Self {
		Self(&self.0 ^ &button.0)
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ButtonSchematic(AHashSet<usize>);
impl ButtonSchematic {
	fn to_joltage(&self, joltage_rank: usize) -> JoltageState {
		let mut joltage = vec![0; joltage_rank];
		self.0.iter().for_each(|&ix| {
			joltage[ix] += 1;
		});
		JoltageState(joltage)
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct JoltageState(Vec<usize>);
impl JoltageState {
	fn with_size(size: usize) -> Self {
		Self(vec![0; size])
	}
	
	const fn rank(&self) -> usize {
		self.0.len()
	}
	
	fn apply_button(&self, button: &ButtonSchematic) -> Self {
		let mut new_joltage = self.clone();
		button.0.iter().for_each(|&ix| {
			new_joltage.0[ix] += 1;
		});
		new_joltage
	}
}
impl Add for JoltageState {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0.iter().enumerate().map(|(ix, &joltage)| {
			joltage + rhs.0[ix]
		}).collect_vec())
	}
}
impl Mul<usize> for JoltageState {
	type Output = Self;

	fn mul(self, rhs: usize) -> Self::Output {
		Self(self.0.iter().map(|&joltage| {
			joltage * rhs
		}).collect_vec())
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Machine(IndicatorLights, Vec<ButtonSchematic>, JoltageState);

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

fn parse_joltage(input: &str) -> IResult<&str, JoltageState> {
	delimited(
		char('{'),
		separated_list1(char(','), usize),
		char('}')
	).map(|joltages| JoltageState(joltages)).parse(input)
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
	let machines = parse(input);
	
	// fn generate_counting_iter_fn(max_val: usize, digits_left: usize) -> Box<dyn Iterator<Item = Vec<usize>>> {
	// 	if digits_left == 1 {
	// 		Box::new(once(vec![max_val]))
	// 	} else {
	// 		Box::new((0..=max_val).flat_map(move |curr_val| {
	// 			let head = once(curr_val);
	// 			generate_counting_iter_fn(max_val - curr_val, digits_left - 1).into_iter()
	// 				.map(move |tail| chain(head.clone(), tail).collect())
	// 		}))
	// 	}
	// }

	let min_steps = machines.into_iter().map(|Machine(_, buttons, target_joltage)| {
		// each joltage can only be incremented by a max of 1 per button press
		// the least number of button presses possible is equal to the highest joltage reading
		// let &min_button_presses = target_joltage.0.iter().max().unwrap();
		// let num_buttons = buttons.len();
		// let buttons_pressed = generate_counting_iter_fn(min_button_presses, num_buttons);
		
		let (path, steps) = astar(
			&vec![0; buttons.len()],
			|node| {
				let target_node = target_joltage.clone();
				let state = node.iter().enumerate().map(|(ix, &button_presses)| {
					buttons[ix].to_joltage(target_node.rank()) * button_presses
				}).reduce(|acc, j| acc + j).unwrap();
				
				let curr_node = node.clone();
				buttons.iter().enumerate().filter_map(move |(ix, button)| {
					let mut new_node = curr_node.clone();
					new_node[ix] += 1;
					let new_state = state.apply_button(button);
					
					if new_state.0.iter().enumerate()
						.any(|(ix, &node_joltage)| node_joltage > target_node.0[ix]) {
						None
					} else {
						Some((new_node, 1))
					}
				})
			},
			|node| {
				let state = node.iter().enumerate().map(|(ix, &button_presses)| {
					buttons[ix].to_joltage(target_joltage.rank()) * button_presses
				}).reduce(|acc, j| acc + j).unwrap();
				target_joltage.0.iter().enumerate().map(|(ix, &target_j)| {
					target_j.abs_diff(state.0[ix])
				}).sum()
			},
			|node| {
				let state = node.iter().enumerate().map(|(ix, &button_presses)| {
					buttons[ix].to_joltage(target_joltage.rank()) * button_presses
				}).reduce(|acc, j| acc + j).unwrap();
				state == target_joltage
			}
		).expect("no solution found for target joltage");
		
		steps as u64
	}).sum();
	
	Ok(min_steps)
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
		assert_eq!(33, part2(TEST)?);
		Ok(())
	}
}