#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;


#[derive(FromStr, Debug, Copy, Clone)]
#[display("{0}")]
pub struct Register(char);

impl Register {
    fn as_idx(&self) -> usize {
        self.0 as usize - b'a' as usize
    }
}

#[derive(FromStr, Debug, Copy, Clone)]
#[display("{0}")]
pub enum Parameter {
    Number(i64),
    Register(Register),
}

impl Parameter {
    fn actual_value(&self, registers: &[i64; 26]) -> i64 {
        match *self {
            Parameter::Register(reg) => registers[reg.as_idx()],
            Parameter::Number(num) => num,
        }
    }
}

#[derive(FromStr, Debug, Copy, Clone)]
#[display("{} {0} {1}", style = "lowercase")]
pub enum Instruction {
    #[display("{} {0}")]
    Snd(Parameter),
    Set(Register, Parameter),
    Add(Register, Parameter),
    Mul(Register, Parameter),
    Mod(Register, Parameter),
    #[display("{} {0}")]
    Rcv(Register),
    Jgz(Parameter, Parameter),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

fn execute_cyle(
    instructions: &[Instruction],
    pc: &mut i64,
    registers: &mut [i64; 26],
    snd_deque: &mut VecDeque<i64>,
    rcv_deque: &mut VecDeque<i64>,
) {
    if *pc < 0 || *pc >= instructions.len() as i64 {
        return;
    }
    match instructions[*pc as usize] {
        Instruction::Snd(param) => {
            snd_deque.push_back(param.actual_value(&registers));
        }
        Instruction::Set(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] = value;
        }
        Instruction::Add(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] += value;
        }
        Instruction::Mul(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] *= value;
        }
        Instruction::Mod(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] %= value;
        }
        Instruction::Rcv(reg) => {
            if let Some(sound) = rcv_deque.pop_front() {
                registers[reg.as_idx()] = sound;
            } else {
                return;
            }
        }
        Instruction::Jgz(param1, param2) => {
            let value1 = param1.actual_value(&registers);
            let value2 = param2.actual_value(&registers);
            if value1 > 0 {
                *pc += value2;
                return;
            }
        }
    }

    *pc += 1;
}

pub fn part1(input: &Input) -> i64 {
    let mut pc = 0;
    let mut registers = [0i64; 26];
    let mut snd = VecDeque::new();
    let mut rcv = VecDeque::new();

    loop {
        if let Instruction::Rcv(reg) = input[pc as usize] {
            if registers[reg.as_idx()] == 0 {
                pc += 1;
                continue;
            } else {
                return snd.pop_back().unwrap();
            }
        }

        execute_cyle(input, &mut pc, &mut registers, &mut snd, &mut rcv);
    }
}

pub fn part2(input: &Input) -> u32 {
    let mut p1_send = 0;

    let mut pc0 = 0;
    let mut deque0 = VecDeque::new();
    let mut registers0 = [0; 26];
    registers0[Register('p').as_idx()] = 0;

    let mut pc1 = 0;
    let mut deque1 = VecDeque::new();
    let mut registers1 = [0; 26];
    registers1[Register('p').as_idx()] = 1;

    loop {
        let (prev_pc0, prev_pc1) = (pc0, pc1);

        execute_cyle(input, &mut pc0, &mut registers0, &mut deque0, &mut deque1);

        let deque1_old_len = deque1.len();

        execute_cyle(input, &mut pc1, &mut registers1, &mut deque1, &mut deque0);

        if deque1.len() != deque1_old_len {
            p1_send += 1;
        }

        if prev_pc0 == pc0 && prev_pc1 == pc1 {
            return p1_send;
        }
    }
}
