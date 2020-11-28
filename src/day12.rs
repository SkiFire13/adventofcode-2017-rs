#[allow(unused_imports)]
use super::prelude::*;
type Input = HashMap<u32, Vec<u32>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split(" <-> ");
            let id = line_split
                .next()
                .expect("Invalid input")
                .parse()
                .expect("Invalid input");
            let cons = line_split
                .next()
                .expect("Invalid input")
                .split(", ")
                .map(|con| con.parse().expect("Invalid input"))
                .collect();
            (id, cons)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut group0 = HashSet::new();
    let mut to_explore = vec![0u32];

    while let Some(next) = to_explore.pop() {
        if group0.insert(next) {
            to_explore.extend(input[&next].iter().cloned())
        }
    }

    group0.len()
}

pub fn part2(input: &Input) -> usize {
    let mut groups: Vec<HashSet<u32>> = Vec::new();

    for item in input.keys().copied() {
        let mut new_group = HashSet::new();

        for group_idx in (0..groups.len()).rev() {
            let set = &groups[group_idx];
            if input[&item].iter().any(|con| set.contains(con)) {
                new_group.extend(groups.swap_remove(group_idx));
            }
        }
        new_group.insert(item);
        groups.push(new_group);
    }

    groups.len()
}
