#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Command>;

#[derive(FromStr, Clone, Copy)]
pub enum Command {
    #[display("s{0}")]
    Spin(usize),
    #[display("x{0}/{1}")]
    Exchange(u8, u8),
    #[display("p{0}/{1}")]
    Partner(char, char),
}

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|command| command.parse().expect("Invalid input"))
        .collect()
}

fn apply_commands(state: &mut VecDeque<u8>, commands: &[Command]) {
    for &command in commands {
        match command {
            Command::Spin(size) => {
                state.rotate_right(size);
            }
            Command::Exchange(idx1, idx2) => {
                state.swap(idx1 as usize, idx2 as usize);
            }
            Command::Partner(a, b) => {
                let idx1 = state.iter().position(|&c| c == a as u8).expect("Invalid input");
                let idx2 = state.iter().position(|&c| c == b as u8).expect("Invalid input");
                state.swap(idx1, idx2);
            }
        }
    }
}

fn state_to_string(state: &VecDeque<u8>) -> String {
    state.iter().map(|&b| b as char).collect()
}

pub fn part1(input: &Input) -> String {
    let mut state = VecDeque::with_capacity(16);
    state.extend("abcdefghijklmnop".bytes());

    apply_commands(&mut state, input);

    state_to_string(&state)
}

pub fn part2(input: &Input) -> String {
    let mut state = VecDeque::with_capacity(16);
    state.extend("abcdefghijklmnop".bytes());

    let mut seen = HashMap::new();
    
    for idx in 0..1_000_000_000 {
        let s = state_to_string(&state);

        if let Some(&seen_idx) = seen.get(&s) {
            let final_idx = seen_idx + (1_000_000_000 - seen_idx) % (seen.len() - seen_idx);
            return seen.into_iter()
                .find(|&(_, idx)| idx == final_idx)
                .map(|(s, _)| s)
                .unwrap();
        } else {
            seen.insert(s, idx);
        }
        
        apply_commands(&mut state, input);
    }

    state_to_string(&state)
}
