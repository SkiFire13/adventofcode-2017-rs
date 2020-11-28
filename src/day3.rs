#[allow(unused_imports)]
use super::prelude::*;
type Input = usize;

fn ring_and_pos(abs_pos: usize) -> (usize, usize) {
    let sqrt = (abs_pos as f64).sqrt().ceil() as usize;
    let sqrt = 1 + 2 * (sqrt / 2);

    let ring = (sqrt - 1) / 2;
    let ring_pos = sqrt * sqrt - abs_pos;

    (ring, ring_pos)
}

fn abs_from_ring_and_pos(ring: usize, ring_pos: usize) -> usize {
    (2 * ring - 1).pow(2) + ring_pos
}

pub fn input_generator(input: &str) -> Input {
    input
        .parse()
        .expect("Input is not a number")
}

pub fn part1(input: &Input) -> usize {
    let input = *input;

    let (ring, ring_pos) = ring_and_pos(input);

    let seg_type = (ring_pos / ring) % 2;
    let seg_pos = ring_pos % ring;
    let seg_dist = seg_type * (seg_pos) + (1 - seg_type) * (ring - seg_pos);

    ring + seg_dist
}

pub fn part2(input: &Input) -> usize {
    let input = *input;

    let mut spiral = vec![0, 1, 1, 2, 4, 5, 10, 11, 23, 25];

    loop {
        let idx = spiral.len();

        let (ring, _) = ring_and_pos(idx);
        let ring_pos = idx - (2 * ring - 1).pow(2);

        let prev = spiral[idx - 1];
        let mut next = prev;

        match ring_pos % (2 * ring) {
            0 => {
                // Angle
                let angle_num = ring_pos / (2 * ring);
                let inner_angle_pos = (2 * (ring - 1)) * angle_num;
                let inner_angle_abs_pos = abs_from_ring_and_pos(ring - 1, inner_angle_pos);
                // Sum angle of the inner ring
                next += spiral[inner_angle_abs_pos];

                if angle_num == 4 {
                    let next_wrap_ring_abs = abs_from_ring_and_pos(ring, 1);
                    // If this is last angle then it touch the first element of this ring too
                    next += spiral[next_wrap_ring_abs];
                }
            }
            1 => {
                // After angle

                let angle_num = ring_pos / (2 * ring);
                let inner_angle_pos = (2 * (ring - 1)) * angle_num;
                let inner_angle_abs_pos = abs_from_ring_and_pos(ring - 1, inner_angle_pos);

                if angle_num != 0 {
                    // If angle_num == 0 then there's no angle before
                    // The inner angle is also the prev element we already counted
                    // And it doesn't touch the prev-prev element

                    // Sum angle of the inner ring
                    next += spiral[inner_angle_abs_pos];

                    // Sum prev-prev element, meaning the element before the angle this element is after
                    next += spiral[idx - 2];
                }

                // Sum element in the inner ring after the angle
                let next_inner_pos = (inner_angle_pos + 1) % (8 * (ring - 1));
                let next_inner_abs_pos = abs_from_ring_and_pos(ring - 1, next_inner_pos);
                next += spiral[next_inner_abs_pos];
            }
            r if r == 2 * ring - 1 => {
                // Before angle

                let angle_num = (ring_pos + 1) / (2 * ring);
                let inner_angle_pos = (2 * (ring - 1)) * angle_num;
                let inner_angle_abs_pos = abs_from_ring_and_pos(ring - 1, inner_angle_pos);

                // Sum angle of the inner ring
                next += spiral[inner_angle_abs_pos];

                let prev_inner_abs_pos = abs_from_ring_and_pos(ring - 1, inner_angle_pos - 1);
                // Sums element before the inner angle
                next += spiral[prev_inner_abs_pos];

                if angle_num == 4 {
                    // If this is the last angle then the element touches the first element of this ring
                    let first_ring_abs = abs_from_ring_and_pos(ring, 1);
                    // Sums first element of the ring
                    next += spiral[first_ring_abs];
                }
            }
            ring_rel_post => {
                // Not near angle

                let angle_num = ring_pos / (2 * ring);
                let corr_inner_pos = (2 * (ring - 1)) * angle_num + (ring_rel_post - 1);
                let corr_inner_abs_pos = abs_from_ring_and_pos(ring - 1, corr_inner_pos);

                // Sum parallel element in inner
                next += spiral[corr_inner_abs_pos];

                // Sum next of parallel element in inner
                next += spiral[corr_inner_abs_pos + 1];

                // The indexes of a ring go from 1 to 8*ring
                // Gets the index in the inner ring of the element before the parallel element
                // The parallel could have index 1 so the prev has index 8*ring
                let prev_corr_inner_pos = if corr_inner_pos - 1 != 0 {
                    corr_inner_pos - 1
                } else {
                    8 * (ring - 1)
                };
                let prev_corr_inner_abs_pos = abs_from_ring_and_pos(ring - 1, prev_corr_inner_pos);
                // Sum of prev of parallel in inner
                next += spiral[prev_corr_inner_abs_pos];
            }
        }

        if next > input {
            return next;
        }

        spiral.push(next);
    }
}
