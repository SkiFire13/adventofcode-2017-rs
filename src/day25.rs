#[allow(unused_imports)]
use super::prelude::*;
type Input = (StateId, usize, HashMap<StateId, State>);

type StateId = char;

#[derive(Constructor, Copy, Clone, Debug)]
pub struct State {
    action_0: Action,
    action_1: Action,
}

#[derive(Constructor, Copy, Clone, Debug)]
pub struct Action {
    write: bool,
    move_right: bool,
    next_state_id: StateId,
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let initial_state_id = lines.next().unwrap().chars().nth(15).unwrap();
    let steps: usize = lines.next().unwrap()[36..]
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let states_idx = input.find("steps.").unwrap() + 8;

    let states = input[states_idx..]
        .split("\n\n")
        .map(|state_descr| {
            let state_id = state_descr[9..10].chars().next().unwrap();

            let mut offset = 0;

            let write_1 = &state_descr[63..64] == "1";
            let move_right = &state_descr[93..94] == "r";
            offset += if move_right { 1 } else { 0 };
            let next_state = state_descr[125 + offset..126 + offset]
                .chars()
                .next()
                .unwrap();
            let action_0 = Action::new(write_1, move_right, next_state);

            let write_1 = &state_descr[179 + offset..180 + offset] == "1";
            let move_right = &state_descr[209 + offset..210 + offset] == "r";
            offset += if move_right { 1 } else { 0 };
            let next_state = state_descr[241 + offset..242 + offset]
                .chars()
                .next()
                .unwrap();
            let action_1 = Action::new(write_1, move_right, next_state);

            (state_id, State::new(action_0, action_1))
        })
        .collect();

    (initial_state_id, steps, states)
}

pub fn part1(input: &Input) -> usize {
    let steps = input.1;
    let states = &input.2;

    let mut current_position: isize = 0;
    let mut current_state_id = input.0;

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for _ in 0..steps {
        let slot = {
            let (idx, vec) = if current_position >= 0 {
                (current_position as usize, &mut right_vec)
            } else {
                ((-current_position - 1) as usize, &mut left_vec)
            };
            while vec.len() <= idx {
                vec.push(false /*0*/);
            }
            &mut vec[idx]
        };
        let state = states[&current_state_id];
        let action = match *slot {
            false /*0*/ => state.action_0,
            true  /*1*/ => state.action_1,
        };

        *slot = action.write;
        current_position += if action.move_right { 1 } else { -1 };
        current_state_id = action.next_state_id;
    }

    left_vec
        .into_iter()
        .chain(right_vec.into_iter())
        .filter(|&b| b)
        .count()
}
