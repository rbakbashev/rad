//! Discrete Interval Encoding Tree (DIET).
//! A direct (and quite ugly) translation of diet.hs from "Diets for Fat Sets", JFP98.

use std::cmp::{max, min};

#[derive(Clone, Debug, PartialEq)]
pub enum Diet {
    Empty,
    Node(i16, i16, Box<Diet>, Box<Diet>),
}

fn split_max(d: Diet) -> (Diet, (i16, i16)) {
    let Diet::Node(x, y, l, r) = d else {
        unreachable!()
    };

    match *r {
        Diet::Empty => (*l, (x, y)),
        Diet::Node(..) => {
            let (d, ni) = split_max(*r);
            let d = Box::new(d);
            (Diet::Node(x, y, l, d), ni)
        }
    }
}

fn split_min(d: Diet) -> (Diet, (i16, i16)) {
    let Diet::Node(x, y, l, r) = d else {
        unreachable!()
    };

    match *l {
        Diet::Empty => (*r, (x, y)),
        Diet::Node(..) => {
            let (d, ni) = split_min(*l);
            let d = Box::new(d);
            (Diet::Node(x, y, d, r), ni)
        }
    }
}

fn adjacent(x: i16, y: i16) -> bool {
    (x + 1) == y
}

fn join_left(d: &Diet) -> Diet {
    let Diet::Node(x, y, l, r) = d else {
        unreachable!()
    };

    match **l {
        Diet::Empty => d.clone(),
        Diet::Node(..) => {
            let (nl, (nx, ny)) = split_max(*l.clone());
            let nl = Box::new(nl);

            if adjacent(ny, *x) {
                Diet::Node(nx, *y, nl, r.clone())
            } else {
                d.clone()
            }
        }
    }
}

fn join_right(d: &Diet) -> Diet {
    let Diet::Node(x, y, l, r) = d else {
        unreachable!()
    };

    match **r {
        Diet::Empty => d.clone(),
        Diet::Node(..) => {
            let (nr, (nx, ny)) = split_min(*r.clone());
            let nr = Box::new(nr);

            if adjacent(*y, nx) {
                Diet::Node(*x, ny, l.clone(), nr)
            } else {
                d.clone()
            }
        }
    }
}

pub fn insert(z: i16, d: &Diet) -> Diet {
    match d {
        Diet::Empty => Diet::Node(z, z, Box::new(Diet::Empty), Box::new(Diet::Empty)),
        Diet::Node(x, y, l, r) => {
            if z < *x {
                if adjacent(z, *x) {
                    let j = Diet::Node(z, *y, Box::new(*l.clone()), Box::new(*r.clone()));
                    join_left(&j)
                } else {
                    let j = insert(z, l);
                    let j = Box::new(j);
                    Diet::Node(*x, *y, j, Box::new(*r.clone()))
                }
            } else if z > *y {
                if adjacent(*y, z) {
                    let j = Diet::Node(*x, z, Box::new(*l.clone()), Box::new(*r.clone()));
                    join_right(&j)
                } else {
                    let j = insert(z, r);
                    let j = Box::new(j);
                    Diet::Node(*x, *y, Box::new(*l.clone()), j)
                }
            } else {
                d.clone()
            }
        }
    }
}

fn no_more_than(d: &Diet, a: i16) -> (Diet, i16) {
    match d {
        Diet::Node(x, y, l, r) => {
            if a > y + 1 {
                let (r2, a2) = no_more_than(r, a);
                (Diet::Node(*x, *y, l.clone(), Box::new(r2)), min(a, a2))
            } else if a >= *x {
                (*l.clone(), *x)
            } else {
                no_more_than(l, a)
            }
        }
        x @ Diet::Empty => (x.clone(), a),
    }
}

fn no_less_than(d: &Diet, a: i16) -> (Diet, i16) {
    match d {
        Diet::Node(x, y, l, r) => {
            if a < x - 1 {
                let (l2, a2) = no_less_than(l, a);
                (Diet::Node(*x, *y, Box::new(l2), r.clone()), max(a, a2))
            } else if a <= *y {
                (*r.clone(), *y)
            } else {
                no_less_than(r, a)
            }
        }
        x @ Diet::Empty => (x.clone(), a),
    }
}

pub fn insert_range(px: i16, py: i16, d: &Diet) -> Diet {
    match d {
        Diet::Empty => Diet::Node(px, py, Box::new(Diet::Empty), Box::new(Diet::Empty)),
        Diet::Node(x, y, ln, rn) => {
            let (l, r) = if *x < px {
                ((*x, *y), (px, py))
            } else {
                ((px, py), (*x, *y))
            };

            let (r1, r2) = if l.1 >= r.0 || adjacent(l.1, r.0) {
                ((l.0, max(l.1, r.1)), None)
            } else {
                ((l.0, l.1), Some((r.0, r.1)))
            };

            match r2 {
                None => {
                    let (left, start) = no_more_than(ln, r1.0);
                    let (right, end) = no_less_than(rn, r1.1);
                    Diet::Node(start, end, Box::new(left), Box::new(right))
                }
                Some(r2) => {
                    if r1.0 == *x && r1.1 == *y {
                        Diet::Node(
                            r1.0,
                            r1.1,
                            Box::new(*ln.clone()),
                            Box::new(insert_range(r2.0, r2.1, rn)),
                        )
                    } else {
                        Diet::Node(
                            r2.0,
                            r2.1,
                            Box::new(insert_range(r1.0, r1.1, ln)),
                            Box::new(*rn.clone()),
                        )
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use crate::rand::Wyhash64RNG;

    #[test]
    fn test() {
        let mut d = Diet::Empty;

        d = insert(1, &d);
        d = insert(3, &d);
        d = insert(5, &d);
        d = insert(6, &d);
        d = insert(7, &d);

        d = insert_range(9, 12, &d);
        d = insert_range(14, 16, &d);
        d = insert_range(13, 18, &d);

        d = insert_range(2, 2, &d);

        d = insert(8, &d);
        d = insert(4, &d);

        assert_eq!(
            d,
            Diet::Node(1, 18, Box::new(Diet::Empty), Box::new(Diet::Empty))
        );
    }

    const RAND_MAX_VAL: usize = 512;
    const _MAX_VAL_VALID_CAST: () = assert!(RAND_MAX_VAL < i16::MAX as usize);

    const SIZE_RANGE: Range<u64> = 1..20;
    const _RANGE_VALID_CAST: () = assert!(SIZE_RANGE.end < i16::MAX as u64);

    #[test]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn random() {
        let num_insertions = 100;
        let seed = 123;

        let mut rng = Wyhash64RNG::from_seed(seed);
        let mut bitset = vec![false; RAND_MAX_VAL];
        let mut diet_once = Diet::Empty;
        let mut diet_bulk = Diet::Empty;

        for _ in 0..num_insertions {
            let start = rng.gen_in_range(0..(RAND_MAX_VAL as u64)) as i16;
            let size = rng.gen_in_range(SIZE_RANGE.clone()) as i16;
            let end = (start + size).min(RAND_MAX_VAL as i16);

            let range = (start as usize)..(end as usize);
            bitset[range].fill(true);

            for i in start..end {
                diet_once = insert(i, &diet_once);
            }

            diet_bulk = insert_range(start, end - 1, &diet_bulk);
        }

        let ranges_bitset = extract_ranges_from_bitset(&bitset);
        let ranges_diet_once = extract_ranges_from_diet(&diet_once);
        let ranges_diet_bulk = extract_ranges_from_diet(&diet_bulk);

        assert_eq!(ranges_bitset, ranges_diet_once);
        assert_eq!(ranges_bitset, ranges_diet_bulk);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn extract_ranges_from_bitset(bitset: &[bool]) -> Vec<(i16, i16)> {
        let mut ranges = vec![];
        let mut start = None;

        for (i, v) in bitset.iter().enumerate() {
            if *v {
                if start.is_none() {
                    start = Some(i as i16);
                }
            } else {
                if let Some(start_val) = start {
                    ranges.push((start_val, i as i16 - 1));
                    start = None;
                }
            }
        }

        if let Some(start_val) = start {
            ranges.push((start_val, bitset.len() as i16 - 1));
        }

        ranges
    }

    fn extract_ranges_from_diet(d: &Diet) -> Vec<(i16, i16)> {
        let mut ranges = vec![];

        match d {
            Diet::Node(start, end, left_node, right_node) => {
                ranges.push((*start, *end));

                let mut left = extract_ranges_from_diet(left_node);
                let mut right = extract_ranges_from_diet(right_node);

                ranges.append(&mut left);
                ranges.append(&mut right);
            }
            Diet::Empty => (),
        }

        ranges.sort_unstable();

        ranges
    }
}
