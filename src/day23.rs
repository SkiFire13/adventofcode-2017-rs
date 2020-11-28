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
    fn actual_value(&self, registers: &[i64; 8]) -> i64 {
        match *self {
            Parameter::Register(reg) => registers[reg.as_idx()],
            Parameter::Number(num) => num,
        }
    }
}

#[derive(FromStr, Debug, Copy, Clone)]
#[display("{} {0} {1}", style = "lowercase")]
pub enum Instruction {
    Set(Register, Parameter),
    Sub(Register, Parameter),
    Mul(Register, Parameter),
    Jnz(Parameter, Parameter),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

fn execute_cycle(instructions: &[Instruction], pc: &mut i64, registers: &mut [i64; 8]) -> bool {
    let mut is_mul = false;

    if *pc < 0 || *pc >= instructions.len() as i64 {
        return is_mul;
    }
    match instructions[*pc as usize] {
        Instruction::Set(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] = value;
        }
        Instruction::Sub(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] -= value;
        }
        Instruction::Mul(reg, param) => {
            let value = param.actual_value(&registers);
            registers[reg.as_idx()] *= value;
            is_mul = true;
        }
        Instruction::Jnz(param1, param2) => {
            let value1 = param1.actual_value(&registers);
            let value2 = param2.actual_value(&registers);
            if value1 != 0 {
                *pc += value2;
                return is_mul;
            }
        }
    }

    *pc += 1;

    is_mul
}

pub fn part1(input: &Input) -> u32 {
    let instructions = input;
    let mut pc = 0;
    let mut registers = [0; 8];

    let mut count_mul = 0;

    loop {
        if pc < 0 || pc >= instructions.len() as i64 {
            break;
        }
        let is_mul = execute_cycle(instructions, &mut pc, &mut registers);
        count_mul += is_mul as u32;
    }

    count_mul
}

pub fn part2(input: &Input) -> usize {
    let instructions = input;
    let mut pc = 0;
    let mut registers = [0; 8];
    registers[Register('a').as_idx()] = 1;

    // Tried to make it less input specific

    // Gets the values of registers `b` and `c`
    loop {
        let prev_pc = pc;
        execute_cycle(instructions, &mut pc, &mut registers);
        if pc < prev_pc {
            break;
        }
    }
    let b: u32 = registers[Register('b').as_idx()] as u32;
    let c: u32 = registers[Register('c').as_idx()] as u32;

    // Gets the value of the `step`
    let step = instructions
        .iter()
        .filter_map(|&instr| match instr {
            Instruction::Sub(Register('b'), Parameter::Number(i)) => Some((-i) as usize),
            _ => None,
        })
        .last()
        .unwrap();

    (b..=c)
        .step_by(step)
        .filter(|&n| {
            if n % 2 == 0 || n % 3 == 0 {
                return true;
            }
            let s = sqrt(n);
            for f in (5..=s).step_by(6) {
                if n % f == 0 || n % (f + 2) == 0 {
                    return true;
                }
            }
            false
        })
        .count()
}
