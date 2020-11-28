#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<HexDirection>;

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexDirection {
    #[display("n")]
    North,
    #[display("nw")]
    NorthWest,
    #[display("ne")]
    NorthEast,
    #[display("s")]
    South,
    #[display("sw")]
    SouthWest,
    #[display("se")]
    SouthEast,
}

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|direction| direction.parse().expect("Invalid input"))
        .collect()
}

fn positions_iter(
    input: impl Iterator<Item = HexDirection>,
) -> impl Iterator<Item = (isize, isize)> {
    let (mut pos_x, mut pos_y) = (0isize, 0isize);

    input.map(move |direction| {
        match direction {
            HexDirection::North => {
                pos_x += 1;
                pos_y += 1;
            }
            HexDirection::NorthWest => {
                pos_y += 1;
            }
            HexDirection::NorthEast => {
                pos_x += 1;
            }
            HexDirection::South => {
                pos_x -= 1;
                pos_y -= 1;
            }
            HexDirection::SouthWest => {
                pos_x -= 1;
            }
            HexDirection::SouthEast => {
                pos_y -= 1;
            }
        }

        (pos_x, pos_y)
    })
}

fn distance_from_hex_origin(pos_x: isize, pos_y: isize) -> usize {
    if (pos_x > 0) == (pos_y > 0) {
        std::cmp::max(pos_x.abs(), pos_y.abs()) as usize
    } else {
        (pos_x.abs() + pos_y.abs()) as usize
    }
}

pub fn part1(input: &Input) -> usize {
    positions_iter(input.iter().cloned())
        .last()
        .map(|(x, y)| distance_from_hex_origin(x, y))
        .unwrap_or(0)
}

pub fn part2(input: &Input) -> usize {
    positions_iter(input.iter().cloned())
        .map(|(x, y)| distance_from_hex_origin(x, y))
        .max()
        .unwrap_or(0)
}
