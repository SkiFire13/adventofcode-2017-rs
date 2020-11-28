#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec2, HashMap<Vec2, bool>);
type Vec2 = (isize, isize);

pub fn input_generator(input: &str) -> Input {
    let mut size_x = 0;
    let mut size_y = 0;
    let map = input
        .lines()
        .enumerate()
        .inspect(|&(y, line)| {
            size_x = line.len();
            size_y = y + 1;
        })
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c == '#'))
        })
        .collect();

    ((size_x as isize, size_y as isize), map)
}

pub fn part1(input: &Input) -> usize {
    let &((size_x, size_y), ref map) = input;
    let mut map = map.clone();

    let mut curr_pos = (size_x / 2, size_y / 2);
    let mut curr_dir = (0, -1);

    let mut n_burst_infect = 0;

    for _ in 0..10000 {
        let infected = map.entry(curr_pos).or_insert(false);
        if *infected {
            // Infected
            curr_dir = (-curr_dir.1, curr_dir.0); // turn right
        } else {
            // Clean
            curr_dir = (curr_dir.1, -curr_dir.0); // turn left
            n_burst_infect += 1; // infect this
        }
        *infected = !*infected;
        curr_pos = (curr_pos.0 + curr_dir.0, curr_pos.1 + curr_dir.1);
    }

    n_burst_infect
}

pub fn part2(input: &Input) -> usize {
    let &((size_x, size_y), ref map) = input;
    let mut map = map
        .iter()
        .map(|(&k, &v)| (k, 2 * v as u8))
        .collect::<HashMap<_, _>>();

    let mut curr_pos = (size_x / 2, size_y / 2);
    let mut curr_dir = (0, -1);

    let mut n_burst_infect = 0;

    for _ in 0..10000000 {
        let infected = map.entry(curr_pos).or_insert(0);
        match *infected {
            0 => {
                // Clean
                curr_dir = (curr_dir.1, -curr_dir.0); // turn left
            }
            1 => {
                // Weakened, no turn
                n_burst_infect += 1; // infect this
            }
            2 => {
                // Infected
                curr_dir = (-curr_dir.1, curr_dir.0); // turn right
            }
            3 => {
                // Flagged
                curr_dir = (-curr_dir.0, -curr_dir.1); // reverse
            }
            _ => unreachable!(),
        }
        *infected = (*infected + 1) % 4;
        curr_pos = (curr_pos.0 + curr_dir.0, curr_pos.1 + curr_dir.1);
    }

    n_burst_infect
}
