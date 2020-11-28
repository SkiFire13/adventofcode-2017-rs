#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Asteroid>;

#[derive(FromStr, Clone, Copy, Add, AddAssign, PartialEq, Eq, Hash)]
#[display("<{x},{y},{z}>")]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(FromStr, Clone, Copy)]
#[display("p={pos}, v={vel}, a={acc}")]
pub struct Asteroid {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .cloned()
        .map(|ast| (ast.pos.abs(), ast.vel.abs(), ast.acc.abs()))
        .enumerate()
        .min_by_key(|&(_, (pt, vt, at))| (at, vt, pt))
        .map(|(i, _)| i)
        .expect("Input was empty")
}

pub fn part2(input: &Input) -> usize {
    let mut asteroids = input.clone();
    
    let mut positions = HashMap::new();
    let mut tmp_vec = Vec::new();

    loop {
        asteroids.iter_mut().for_each(|ast| {
            ast.vel += ast.acc;
            ast.pos += ast.vel;
        });

        positions.clear();
        for ast in asteroids.iter() {
            *positions.entry(ast.pos).or_insert(0usize) += 1;
        }

        asteroids.retain(|ast| positions[&ast.pos] == 1);
        
        let mut component_ordered = |comp: fn(Vec3) -> i32| {
            let iter = asteroids
                .iter()
                .cloned()
                .map(|ast| (comp(ast.pos), comp(ast.vel), comp(ast.acc)));
            tmp_vec.extend(iter);
            tmp_vec.sort();
            tmp_vec
                .drain(..)
                .tuple_windows()
                .all(|(cmp1, cmp2)| cmp1.0 <= cmp2.0 && cmp1.1 <= cmp2.1 && cmp1.2 <= cmp2.2)
        };

        if component_ordered(|v| v.x) && component_ordered(|v| v.y) && component_ordered(|v| v.z) {
            return asteroids.len();
        }
    }
}
