#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Component>;


#[derive(FromStr, Copy, Clone, PartialEq, Eq, Hash)]
#[display("{0}/{1}")]
pub struct Component(usize, usize);

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn max_bridge_by<T: Ord + Copy>(
    start: usize,
    input: &[Component],
    seen: &mut Vec<Component>,
    default: T,
    f: fn(Component, T) -> T,
) -> T {
    input
        .iter()
        .copied()
        .filter(|&comp| comp.0 == start || comp.1 == start)
        .map(|comp| {
            // `seen` should be so small that it should be faster than using an HashSet
            if seen.contains(&comp) {
                return default;
            }
            seen.push(comp);
            let ris = max_bridge_by(start ^ comp.0 ^ comp.1, input, seen, default, f);
            seen.pop();
            f(comp, ris)
        })
        .max()
        .unwrap_or(default)
}

pub fn part1(input: &Input) -> usize {
    max_bridge_by(
        0,
        input,
        &mut Vec::with_capacity(input.len()),
        0,
        |comp, ris| comp.0 + comp.1 + ris,
    )
}

pub fn part2(input: &Input) -> usize {
    max_bridge_by(
        0,
        input,
        &mut Vec::with_capacity(input.len()),
        (0, 0),
        |comp, ris| (1 + ris.0, comp.0 + comp.1 + ris.1),
    )
    .1
}
