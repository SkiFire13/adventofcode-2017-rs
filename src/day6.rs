#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|line| line.parse().expect("Invalid number in input"))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut banks = input.clone();
    let mut banks_seen = HashSet::new();

    while banks_seen.insert(banks.clone()) {
        let (blocks, Reverse(idx)) = banks.iter().copied()
            .enumerate()
            .map(|(idx, blocks)| (blocks, Reverse(idx)))
            .max()
            .expect("No banks");

        banks[idx] = 0;
        let new_blocks_per_bank = blocks / banks.len() as u32;
        for bank in &mut banks {
            *bank += new_blocks_per_bank;
        }

        let extra_blocks = blocks - new_blocks_per_bank * banks.len() as u32;

        for i in idx+1 ..= idx + extra_blocks as usize{
            let i = i % banks.len();
            banks[i] += 1;
        }
    }

    banks_seen.len()
}

pub fn part2(input: &Input) -> usize {
    let mut banks = input.clone();
    let mut banks_seen = HashMap::new();

    while !banks_seen.contains_key(&banks) {
        let cycle = banks_seen.len();
        banks_seen.insert(banks.clone(), cycle);

        let (blocks, Reverse(idx)) = banks.iter().copied()
            .enumerate()
            .map(|(idx, blocks)| (blocks, Reverse(idx)))
            .max()
            .expect("No banks");

        banks[idx] = 0;
        let new_blocks_per_bank = blocks / banks.len() as u32;
        for bank in &mut banks {
            *bank += new_blocks_per_bank;
        }

        let extra_blocks = blocks as usize - new_blocks_per_bank as usize * banks.len();

        for i in idx+1 ..= idx + extra_blocks {
            let i = i % banks.len();
            banks[i] += 1;
        }
    }

    banks_seen.len() - banks_seen[&banks]
}
