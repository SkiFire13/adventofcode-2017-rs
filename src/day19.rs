#[allow(unused_imports)]
use super::prelude::*;
type Input = (HashMap<(usize, usize), char>, usize);


pub fn input_generator(input: &str) -> Input {
    let mut start_x = 0;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter(|&(_, c)| c != ' ')
        .inspect(|&((x, y), _)| {
            if y == 0 {
                start_x = x;
            }
        })
        .collect();

    (map, start_x)
}

fn navigate(map: &HashMap<(usize, usize), char>, start_x: usize, mut callback: impl FnMut(char)) {
    let (mut pos_x, mut pos_y) = (start_x as isize, 0isize);
    let (mut dir_x, mut dir_y) = (0, 1);

    while let Some(&c) = map.get(&(pos_x as usize, pos_y as usize)) {
        callback(c);

        if c == '+' {
            let new_dir = [(dir_y, dir_x), (-dir_y, -dir_x)]
                .iter()
                .cloned()
                .find(|&(new_dir_x, new_dir_y)| {
                    map.contains_key(&((pos_x + new_dir_x) as usize, (pos_y + new_dir_y) as usize))
                })
                .unwrap();
            dir_x = new_dir.0;
            dir_y = new_dir.1;
        }

        pos_x += dir_x;
        pos_y += dir_y;
    }
}

pub fn part1(input: &Input) -> String {
    let &(ref map, start_x) = input;
    let mut letters = String::new();

    navigate(map, start_x, |c| {
        if c != '|' && c != '-' && c != '+' {
            letters.push(c)
        }
    });

    letters
}

pub fn part2(input: &Input) -> usize {
    let &(ref map, start_x) = input;
    let mut steps = 0;
    navigate(map, start_x, |_| steps += 1);
    steps
}
