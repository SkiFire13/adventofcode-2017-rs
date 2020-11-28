#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<String>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.split_whitespace().map(str::to_string).collect()).collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|words| {
            words
                .iter()
                .all(|word1| words.iter().filter(|&word2| word1 == word2).count() == 1)
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|words| {
            words
                .iter()
                .map(|word| {
                    word.bytes().fold([0u8; 26], |mut count, char| {
                        count[(char - b'a') as usize] += 1;
                        count
                    })
                })
                .collect::<Vec<_>>()
        })
        .filter(|words| {
            words
                .iter()
                .all(|word1| words.iter().filter(|&word2| word1 == word2).count() == 1)
        })
        .count()
}
