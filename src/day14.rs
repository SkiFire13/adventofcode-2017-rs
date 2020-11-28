#[allow(unused_imports)]
use super::prelude::*;
use crate::day10::knot_hash_bytes;
type Input = String;

fn generate_hash_input(input: &str, i: u8) -> impl Iterator<Item = u8> + Clone + '_ {
    input.bytes().chain(ArrayVec::from(['-' as u8])).chain(
        ArrayVec::from([i / 100, (i / 10) % 10, i % 10])
            .into_iter()
            .skip_while(move |&b| b == 0)
            .chain(std::iter::once(0).filter(move |_| i == 0))
            .map(|b| b'0' + b),
    )
}

pub fn input_generator(input: &str) -> Input {
    input.to_string()
}

pub fn part1(input: &Input) -> u32 {
    (0..128)
        .flat_map(|i| knot_hash_bytes(generate_hash_input(input, i)))
        .map(u8::count_ones)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let grid = (0..128u8)
        .map(|i| knot_hash_bytes(generate_hash_input(input, i)))
        .flatten()
        .flat_map(|byte| {
            ArrayVec::from([
                (byte >> 7) & 1 != 0,
                (byte >> 6) & 1 != 0,
                (byte >> 5) & 1 != 0,
                (byte >> 4) & 1 != 0,
                (byte >> 3) & 1 != 0,
                (byte >> 2) & 1 != 0,
                (byte >> 1) & 1 != 0,
                (byte >> 0) & 1 != 0,
            ])
        })
        .collect::<Vec<_>>();

    let mut groupmap = vec![0; grid.len()];
    let mut removed_groups = 0;
    let mut group_id_counter = 1;

    for (x, y) in (0..128)
        .cartesian_product(0..128)
        .filter(|&(x, y)| grid[y * 128 + x])
    {
        let neighbours_groups = ArrayVec::from([(-1, 0), (0, -1)])
            .into_iter()
            .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(nx, ny)| 0 <= nx && 0 <= ny)
            .map(|(nx, ny)| (ny * 128 + nx) as usize)
            .filter(|&p| grid[p])
            .map(|p| groupmap[p])
            .collect::<ArrayVec<[usize; 2]>>();

        groupmap[y * 128 + x] = neighbours_groups
            .get(0)
            .copied()
            .unwrap_or(group_id_counter);

        match neighbours_groups.len() {
            0 => group_id_counter += 1,
            2 if neighbours_groups[0] != neighbours_groups[1] => {
                removed_groups += 1;
                groupmap[..y * 128 + x]
                    .iter_mut()
                    .filter(|&&mut gi| gi == neighbours_groups[1])
                    .for_each(|gi| *gi = neighbours_groups[0]);
            }
            _ => {}
        }
    }

    group_id_counter - 1 - removed_groups
}
