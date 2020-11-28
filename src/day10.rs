#[allow(unused_imports)]
use super::prelude::*;
type Input = String;

fn reverse(slice: &mut [u8], start_pos: usize, length: usize) {
    if length != 0 && length != 1 {
        let start_pos = start_pos % slice.len();
        let end_pos = (start_pos + length) % slice.len();
        if start_pos < end_pos {
            slice[start_pos..end_pos].reverse();
        } else {
            let mut start_pos = start_pos;
            let mut end_pos = (end_pos + slice.len() - 1) % slice.len();
            let mut stop = false;
            while !stop {
                slice.swap(start_pos, end_pos);
                start_pos = (start_pos + 1) % slice.len();
                if start_pos == end_pos {
                    stop = true;
                }
                end_pos = (end_pos + slice.len() - 1) % slice.len();
                if start_pos == end_pos {
                    stop = true;
                }
            }
        }
    }
}

fn sparse_hash(input: impl IntoIterator<Item = u8>) -> ArrayVec<[u8; 256]> {
    let mut list = (0..=255).collect::<ArrayVec<_>>();

    let mut current_position = 0;
    let mut skip = 0;

    for length in input.into_iter().map(|b| b as usize) {
        reverse(&mut list, current_position, length);
        current_position = (current_position + length + skip) % list.len();
        skip = (skip + 1) % list.len();
    }

    list
}

fn knot_hash_str(input: impl IntoIterator<Item = u8> + Clone) -> String {
    knot_hash_bytes(input)
        .into_iter()
        .flat_map(|byte| ArrayVec::from([byte / 16, byte % 16]))
        .map(|digit| match digit {
            0..=9 => (b'0' + digit) as char,
            10..=15 => (b'a' + digit - 10) as char,
            _ => unreachable!(),
        })
        .collect()
}

pub fn knot_hash_bytes(input: impl IntoIterator<Item = u8> + Clone) -> ArrayVec<[u8; 16]> {
    sparse_hash(
        std::iter::repeat_with(|| {
            input
                .clone()
                .into_iter()
                .chain([17, 31, 73, 47, 23].iter().copied())
        })
        .take(64)
        .flatten(),
    )
    .chunks_exact(16)
    .map(|chunk| chunk.iter().copied().fold(0, |acc, elem| acc ^ elem))
    .collect()
}

pub fn input_generator(input: &str) -> Input { input.to_string() }

pub fn part1(input: &Input) -> u16 {
    let result = sparse_hash(
        input
            .split(',')
            .map(|n| n.parse::<u8>().expect("Invalid input")),
    );

    result[0] as u16 * result[1] as u16
}

pub fn part2(input: &Input) -> String {
    knot_hash_str(input.bytes())
}
