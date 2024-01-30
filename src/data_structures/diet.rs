//! Discrete Interval Encoding Tree (DIET).
//! A direct (and quite ugly) translation of diet.hs from "Diets for Fat Sets", JFP98.

#![allow(
    clippy::many_single_char_names,
    clippy::uninlined_format_args,
    clippy::option_if_let_else,
    clippy::missing_const_for_fn
)]

use std::cmp::{max, min};

#[derive(Clone, Debug, PartialEq)]
enum Diet {
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
    let Diet::Node(x, y, l, r) = &d else {
        unreachable!()
    };

    match **l {
        Diet::Empty => d.clone(),
        Diet::Node(_, _, _, _) => {
            let (nl, (nx, ny)) = split_max(*l.clone());
            let nl = Box::new(nl);

            if adjacent(ny, *x) {
                Diet::Node(nx, *y, nl, Box::new(*r.clone()))
            } else {
                d.clone()
            }
        }
    }
}

fn join_right(d: &Diet) -> Diet {
    let Diet::Node(x, y, l, r) = &d else {
        unreachable!()
    };

    match **r {
        Diet::Empty => d.clone(),
        Diet::Node(_, _, _, _) => {
            let (nr, (nx, ny)) = split_min(*r.clone());
            let nr = Box::new(nr);

            if adjacent(*y, nx) {
                Diet::Node(*x, ny, Box::new(*l.clone()), nr)
            } else {
                d.clone()
            }
        }
    }
}

fn insert(z: i16, d: &Diet) -> Diet {
    match d {
        Diet::Empty => Diet::Node(z, z, Box::new(Diet::Empty), Box::new(Diet::Empty)),
        Diet::Node(x, y, l, r) => {
            if z < *x {
                if adjacent(z, *x) {
                    let j = Box::new(Diet::Node(
                        z,
                        *y,
                        Box::new(*l.clone()),
                        Box::new(*r.clone()),
                    ));
                    join_left(&j)
                } else {
                    let j = insert(z, l);
                    let j = Box::new(j);
                    Diet::Node(*x, *y, j, Box::new(*r.clone()))
                }
            } else if z > *y {
                if adjacent(*y, z) {
                    let j = Box::new(Diet::Node(
                        *x,
                        z,
                        Box::new(*l.clone()),
                        Box::new(*r.clone()),
                    ));
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
                (
                    Diet::Node(*x, *y, Box::new(*l.clone()), Box::new(r2)),
                    min(a, a2),
                )
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
                (
                    Diet::Node(*x, *y, Box::new(l2), Box::new(*r.clone())),
                    max(a, a2),
                )
            } else if a <= *y {
                (*r.clone(), *y)
            } else {
                no_less_than(r, a)
            }
        }
        x @ Diet::Empty => (x.clone(), a),
    }
}

fn insert_range(px: i16, py: i16, d: &Diet) -> Diet {
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
