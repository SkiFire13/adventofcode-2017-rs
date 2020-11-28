#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Group>, usize);

enum ParserState {
    Group(Group),
    Garbage,
    GarbageIgnoreNext,
}

pub struct Group(Vec<Group>);

pub fn input_generator(input: &str) -> Input {
    let mut stack_state = vec![ParserState::Group(Group(Vec::new()))];
    let mut garbage_characters = 0;

    for c in input.chars() {
        match (
            stack_state.last().expect("Invalid parser state: no state"),
            c,
        ) {
            (ParserState::GarbageIgnoreNext, _) => {
                stack_state.pop();
            }
            (ParserState::Garbage, '!') => {
                stack_state.push(ParserState::GarbageIgnoreNext);
            }
            (ParserState::Garbage, '>') => {
                stack_state.pop();
            }
            (ParserState::Garbage, _) => {
                garbage_characters += 1;
            }
            (ParserState::Group(_), '<') => {
                stack_state.push(ParserState::Garbage);
            }
            (ParserState::Group(_), ',') => {}
            (ParserState::Group(_), '{') => {
                stack_state.push(ParserState::Group(Group(Vec::new())));
            }
            (ParserState::Group(_), '}') => {
                let group = match stack_state.pop().unwrap() {
                    ParserState::Group(group) => group,
                    _ => unreachable!(),
                };
                match stack_state
                    .last_mut()
                    .expect("Invalid parser state: no prev state")
                {
                    ParserState::Group(prev_group) => {
                        prev_group.0.push(group);
                    }
                    _ => unreachable!(),
                }
            }
            _ => panic!("Invalid parser state: unknown character"),
        }
    }

    if stack_state.len() > 1 {
        panic!("Invalid final state: too many states");
    }

    let root_vec = match stack_state.pop().unwrap() {
        ParserState::Group(Group(root_vec)) => root_vec,
        _ => unreachable!(),
    };

    if root_vec.len() > 1 {
        println!("Info: multiple root groups");
    }

    (root_vec, garbage_characters)
}

pub fn part1(input: &Input) -> usize {
    fn total_weight_of(group: &Group, nest_level: usize) -> usize {
        1usize
            + nest_level
            + group
                .0
                .iter()
                .map(|group| total_weight_of(group, nest_level + 1))
                .sum::<usize>()
    }

    let (groups, _) = input;
    groups.iter().map(|group| total_weight_of(group, 0)).sum::<usize>()
}

pub fn part2(input: &Input) -> usize {
    let &(_, garbage_count) = input;
    garbage_count
}
