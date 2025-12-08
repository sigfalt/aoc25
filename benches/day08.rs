use std::fs;
use divan::{AllocProfiler, Bencher};

use aoc25::day08::*;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const DAY: &str = "08";

fn main() {
	divan::main();
}

#[divan::bench(name = "part1", min_time = 5)]
fn bench_part1(bencher: Bencher) {
	bencher.with_inputs(|| {
		fs::read_to_string(format!("input/{}.txt", DAY)).unwrap()
	}).bench_local_values(|input| {
		let _ = part1(input.as_str());
	})
}

#[divan::bench(name = "part2", min_time = 5)]
fn bench_part2(bencher: Bencher) {
	bencher.with_inputs(|| {
		fs::read_to_string(format!("input/{}.txt", DAY)).unwrap()
	}).bench_local_values(|input| {
		let _ = part2(input.as_str());
	})
}