#[allow(unused_imports)]
use super::prelude::*;
type Input = (PatternMap, PatternMap);
type PatternMap = HashMap<ArrayVec<[bool; 9]>, ArrayVec<[bool; 16]>>;

fn parse_patt<A: arrayvec::Array<Item=bool>>(raw: &str) -> ArrayVec<A> {
    raw.chars()
        .filter(|&c| c != '/')
        .map(|c| c == '#')
        .collect()
}

fn transform_patt<F: Fn(usize, usize, usize) -> usize>(
    mut patt_iter: impl ExactSizeIterator<Item = bool>,
    patt_dim: usize,
    idx_f: F,
) -> arrayvec::IntoIter<[bool; 9]> {
    let mut buf = ArrayVec::from([false; 9]);
    buf.truncate(patt_dim * patt_dim);

    for idx1 in 0..patt_dim {
        for idx2 in (0..patt_dim).rev() {
            buf[idx_f(idx1, idx2, patt_dim)] = patt_iter.next().unwrap();
        }
    }

    buf.into_iter()
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let patt = parse_patt(split.next().unwrap());
            split.next();
            let repl_patt = parse_patt(split.next().unwrap());
            (patt, repl_patt)
        })
        .flat_map(|(patt, repl_patt)| {
            let patt_dim = sqrt(patt.len());
            let patt_iter = patt.into_iter();
            let rotate = |iter| transform_patt(iter, patt_dim, |x, y, dim| dim * y + x);
            let flip = |iter| transform_patt(iter, patt_dim, |y, x, dim| dim * y + x);
            ArrayVec::from([
                patt_iter.clone(),                         // orig
                flip(patt_iter.clone()),                   // orig + flip
                rotate(patt_iter.clone()),                 // 1-rotate
                flip(rotate(patt_iter.clone())),           // 1-rotate + flip
                rotate(rotate(patt_iter.clone())),         // 2-rotate
                flip(rotate(rotate(patt_iter.clone()))),   // 2-rotate + flip
                rotate(rotate(rotate(patt_iter.clone()))), // 3-rotate
                flip(rotate(rotate(rotate(patt_iter)))),   // 3-rotate + flip
            ])
            .into_iter()
            .map(|new_patt_iter| new_patt_iter.collect::<ArrayVec<[bool; 9]>>())
            .map(move |new_patt| (new_patt, repl_patt.clone()))
        })
        .partition(|(patt, _)| patt.len() == 4)
}

fn apply_transformation(image: &mut Vec<bool>, patts2: &PatternMap, patts3: &PatternMap) {
    let image_dim = sqrt(image.len());
    let square_dim = if image_dim % 2 == 0 { 2 } else { 3 };

    let repl_patt_dim = square_dim + 1;
    let new_image_dim = image_dim * repl_patt_dim / square_dim;

    let patts = if square_dim == 2 { patts2 } else { patts3 };

    image.resize(new_image_dim * new_image_dim, false);
    for y in (0..image_dim).rev() {
        image.copy_within(image_dim  * y .. image_dim * (y + 1), new_image_dim * y);
    }

    for square_y in (0..image_dim / square_dim).rev() {
        for square_x in (0..image_dim / square_dim).rev() {
            let replace_patt = patts
                .iter()
                .find(|&(patt, _)| {
                    (0..square_dim)
                        .flat_map(|dy| (0..square_dim).map(move |dx| (dx, dy)))
                        .map(|(dx, dy)| (square_x * square_dim + dx, square_y * square_dim + dy))
                        .map(|(x, y)| image[new_image_dim * y + x])
                        .zip(patt)
                        .all(|(b1, &b2)| b1 == b2)
                })
                .map(|(_, replace_patt)| replace_patt)
                .unwrap();

            for dy in 0..repl_patt_dim {
                for dx in 0..repl_patt_dim {
                    let y = square_y * repl_patt_dim + dy;
                    let x = square_x * repl_patt_dim + dx;
                    image[y * new_image_dim + x] = replace_patt[repl_patt_dim * dy + dx];
                }
            }
        }
    }
}

fn pixel_on_after_iterations(input: &(PatternMap, PatternMap), iterations: usize) -> usize {
    let mut image = parse_patt::<[_; 9]>(".#./..#/###").to_vec();
    for _ in 0..iterations {
        apply_transformation(&mut image, &input.0, &input.1);
    }
    image.into_iter().filter(|&b| b).count()
}

pub fn part1(input: &Input) -> usize {
    pixel_on_after_iterations(input, 5)
}

pub fn part2(input: &Input) -> usize {
    pixel_on_after_iterations(input, 18)
}
