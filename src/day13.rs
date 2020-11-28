#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u32, u32)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split(": ");
            let depth = line_split
                .next()
                .expect("Invalid input")
                .parse()
                .expect("Invalid input");
            let range = line_split
                .next()
                .expect("Invalid input")
                .parse()
                .expect("Invalid input");
            (depth, range)
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .copied()
        .filter(|(pos, range)| pos % (2 * range - 2) == 0)
        .map(|(pos, range)| pos * range)
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    // More efficient solution than the bruteforce one
    let mut modulus = 1;
    let mut possible_offsets = HashSet::new();
    possible_offsets.insert(0);

    for (pos, range) in input.iter().cloned() {
        if range != 0 && range != 1 {
            let old_modulus = modulus;
            let drange = 2 * (range - 1);
            let (gcd, lcm) = gcd_lcm(drange, old_modulus);
            let factor = drange / gcd;
            modulus = lcm;

            // Chances are that this will resize so it will allocate anyway
            // so trying to recycle the old instance is useless
            possible_offsets = possible_offsets
                .into_iter()
                .flat_map(|offset| (0..factor).map(move |i| old_modulus * i + offset))
                .filter(|&offset| (offset + pos) % drange != 0)
                .collect();
        }
    }

    possible_offsets
        .iter()
        .cloned()
        .min()
        .expect("No possible solutions")
}
