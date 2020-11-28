#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(String, i32, String, CmpOp, i32)>;

#[derive(Clone, Copy, PartialEq, Eq, FromStr)]
pub enum CmpOp {
    #[display("<")]
    Less,
    #[display("<=")]
    LessEqual,
    #[display(">")]
    Greater,
    #[display(">=")]
    GreaterEqual,
    #[display("==")]
    Equal,
    #[display("!=")]
    NotEqual,
}

impl CmpOp {
    fn cmp(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            CmpOp::Less => lhs < rhs,
            CmpOp::LessEqual => lhs <= rhs,
            CmpOp::Greater => lhs > rhs,
            CmpOp::GreaterEqual => lhs >= rhs,
            CmpOp::Equal => lhs == rhs,
            CmpOp::NotEqual => lhs != rhs,
        }
    }
}

#[derive(FromStr)]
#[display(style = "lowercase")]
enum Op { Inc, Dec }

#[derive(FromStr)]
#[from_str(regex = r"(?P<dest_reg>\S+) (?P<op>\S+) (?P<qnt>\S+) if (?P<cmp_reg>\S+) (?P<cmp_op>\S+) (?P<cmp_qnt>\S+)")]
struct InputLine {
    dest_reg: String,
    op: Op,
    qnt: i32,
    cmp_reg: String,
    cmp_op: CmpOp,
    cmp_qnt: i32,
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let line: InputLine = line.parse().expect("Invalid input");
            let add_qnt = match line.op {
                Op::Inc => line.qnt,
                Op::Dec => -line.qnt,
            };
            (line.dest_reg, add_qnt, line.cmp_reg, line.cmp_op, line.cmp_qnt)
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut registers = HashMap::new();
    for &(ref dest_reg, add_qnt, ref cmp_reg, cmp_op, cmp_qnt) in input.iter() {
        let cmp_reg_value = *registers.entry(cmp_reg).or_insert(0);
        if cmp_op.cmp(cmp_reg_value, cmp_qnt) {
            *registers.entry(dest_reg).or_insert(0) += add_qnt;
        }
    }

    registers.values().max().cloned().unwrap_or(0)
}

pub fn part2(input: &Input) -> i32 {
    let mut registers = HashMap::new();
    input.iter()
        .map(|&(ref dest_reg, add_qnt, ref cmp_reg, cmp_op, cmp_qnt)| {
            let cmp_reg_value = *registers.entry(cmp_reg).or_insert(0);
            if cmp_op.cmp(cmp_reg_value, cmp_qnt) {
                *registers.entry(dest_reg).or_insert(0) += add_qnt;
            }
            *registers.entry(dest_reg).or_insert(0)
        })
        .max()
        .unwrap_or(0)
}
