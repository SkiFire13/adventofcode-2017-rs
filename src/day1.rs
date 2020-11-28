#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

fn match_n_ahead(input: &[u8], n: usize) -> u32 {
    input
        .iter()
        .cloned()
        .zip(
            input
                .iter()
                .cloned()
                .skip(n)
                .chain(input.iter().cloned().take(n)),
        )
        .filter(|(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as u32)
        .sum()
}

pub fn input_generator(input: &str) -> Input {
    input.as_bytes().to_vec()
}

pub fn part1(input: &Input) -> u32 {
    match_n_ahead(input, 1)
}

pub fn part2(input: &Input) -> u32 {
    match_n_ahead(input, input.len() / 2)
}
