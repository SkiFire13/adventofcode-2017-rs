#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u32>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("Invalid number in input"))
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|row| {
            let (min, max) = row.iter().minmax().into_option().expect("Empty row");
            max - min
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|row|
            row
                .iter()
                .flat_map(|n| row.iter().map(move |i| (n / i, n % i)))
                .filter(|&(q, r)| q != 1 && r == 0)
                .map(|(q, _)| q)
                .exactly_one()
                .expect("Line without evenly divisible numbers")
        )
        .sum()
}
