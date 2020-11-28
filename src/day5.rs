#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<i32>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid number in input"))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut jump_table = input.clone();
    let mut curr_idx = 0;
    let mut steps = 0;

    while curr_idx < jump_table.len() {
        let jump_value = jump_table[curr_idx];
        jump_table[curr_idx] += 1;
        curr_idx = (curr_idx as i32 + jump_value) as usize;
        steps += 1;
    }

    steps
}

pub fn part2(input: &Input) -> usize {
    let mut jump_table = input.clone();
    let mut curr_idx = 0;
    let mut steps = 0;

    while curr_idx < jump_table.len() {
        let jump_value = jump_table[curr_idx];
        jump_table[curr_idx] += if jump_value >= 3 { -1 } else { 1 };
        curr_idx = (curr_idx as i32 + jump_value) as usize;
        steps += 1;
    }

    steps
}
