#[allow(unused_imports)]
use super::prelude::*;
type Input = usize;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Invalid input")
}

pub fn part1(input: &Input) -> u16 {
    let &input = input;
    let mut state = VecDeque::with_capacity(1 + 2017);
    state.push_back(0);

    for i in 1..=2017 {
        state.rotate_left(input % state.len());
        state.push_back(i);
    }

    state.front().cloned().unwrap()
}

pub fn part2(input: &Input) -> u32 {
    let &input = input;
    std::iter::successors(Some((0, 0)), |&(i, prev_pos)| {
        Some((i + 1, 1u32 + (prev_pos + input as u32) % (i + 1)))
    })
    .take(1 + 50_000_000)
    .filter(|&(_, pos)| pos == 1)
    .map(|(i, _)| i)
    .last()
    .unwrap()
}
