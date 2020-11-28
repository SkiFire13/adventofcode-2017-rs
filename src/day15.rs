#[allow(unused_imports)]
use super::prelude::*;
type Input = (u64, u64);


pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let first_line = lines.next().expect("Invalid input");
    let first = first_line[24..].parse().expect("Invalid input");

    let second_line = lines.next().expect("Invalid input");
    let second = second_line[24..].parse().expect("Invalid input");

    (first, second)
}

fn generator_iter(starting_value: u64, factor: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(starting_value), move |prev| {
        Some((prev * factor) % 2147483647)
    })
    .skip(1)
}

const LAST_16_BIT_MASK: u64 = (1 << 16) - 1;

pub fn part1(input: &Input) -> usize {
    let &(input_a, input_b) = input;
    let gen_a = generator_iter(input_a, 16807);
    let gen_b = generator_iter(input_b, 48271);

    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(a, b)| a & LAST_16_BIT_MASK == b & LAST_16_BIT_MASK)
        .count()
}

pub fn part2(input: &Input) -> usize {
    let &(input_a, input_b) = input;
    let gen_a = generator_iter(input_a, 16807).filter(|&n| n % 4 == 0);
    let gen_b = generator_iter(input_b, 48271).filter(|&n| n % 8 == 0);

    gen_a
        .zip(gen_b)
        .take(5_000_000)
        .filter(|&(a, b)| a & LAST_16_BIT_MASK == b & LAST_16_BIT_MASK)
        .count()
}
