use std::fmt;

use crate::data_structures::array_2d::Array2D;

pub fn rod_cutting(prices: &[usize], n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut revenues = vec![0; n + 1];
    let mut cuts = vec![0; n + 1];

    for j in 1..=n {
        let mut max_rev = 0;

        for i in 1..=j {
            let cut_rev = prices[i] + revenues[j - i];

            if cut_rev > max_rev {
                max_rev = cut_rev;
                cuts[j] = i;
            }
        }

        revenues[j] = max_rev;
    }

    (revenues, cuts)
}

pub fn rod_cutting_extended(prices: &[usize], mut n: usize) -> (usize, Vec<usize>) {
    let (revenues, cuts) = rod_cutting(prices, n);
    let revenue = revenues[n];
    let mut reconstr = vec![];

    while n > 0 {
        reconstr.push(cuts[n]);
        n -= cuts[n];
    }

    (revenue, reconstr)
}

pub fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    let mut m = vec![0; n + 1];

    m[0] = 0;
    m[1] = 1;

    for i in 2..=n {
        m[i] = m[i - 1] + m[i - 2];
    }

    m[n]
}

pub fn fib2(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    let mut m0 = 0;
    let mut m1 = 1;

    for _ in 2..=n {
        let m = m1 + m0;

        m0 = m1;
        m1 = m;
    }

    m1
}

pub fn matrix_parenthesization(dimensions: &[u64]) -> String {
    let p = dimensions;
    let n = p.len() - 1;
    let mut m = Array2D::new(None, n + 1, n + 1);
    let mut s = Array2D::new(0, n + 1, n);

    for i in 1..=n {
        m[i][i] = Some(0);
    }

    for l in 2..=n {
        for i in 1..=(n - l + 1) {
            let j = i + l - 1;

            for k in i..j {
                let Some(prefix) = m[i][k] else {
                    continue;
                };
                let Some(postfix) = m[k + 1][j] else {
                    continue;
                };
                let this_cost = p[i - 1] * p[k] * p[j];
                let full_cost = prefix + postfix + this_cost;

                let curr = &mut m[i][j];

                if curr.is_none() || curr.is_some_and(|cost| full_cost < cost) {
                    *curr = Some(full_cost);
                    s[i][j] = k;
                }
            }
        }
    }

    let mut output = String::new();

    print_parenthesization(&mut output, &s, 1, n).expect("failed to print parenthesization");

    output
}

fn print_parenthesization<W: fmt::Write>(
    f: &mut W,
    s: &Array2D<usize>,
    i: usize,
    j: usize,
) -> fmt::Result {
    if i == j {
        return write!(f, "A{i}");
    }

    write!(f, "(")?;

    print_parenthesization(f, s, i, s[i][j])?;
    print_parenthesization(f, s, s[i][j] + 1, j)?;

    write!(f, ")")
}

#[derive(Clone, Copy)]
enum Direction {
    Unset,
    Up,
    Left,
    UpLeft,
}

pub fn longest_common_subsequence(xs: &[u8], ys: &[u8]) -> Vec<u8> {
    let xl = xs.len();
    let yl = ys.len();

    let mut c = Array2D::new(0, xl + 1, yl + 1);
    let mut b = Array2D::new(Direction::Unset, xl + 1, yl + 1);

    for x in 1..=xl {
        for y in 1..=yl {
            if xs[x - 1] == ys[y - 1] {
                c[y][x] = 1 + c[y - 1][x - 1];
                b[y][x] = Direction::UpLeft;
            } else if c[y - 1][x] >= c[y][x - 1] {
                c[y][x] = c[y - 1][x];
                b[y][x] = Direction::Up;
            } else {
                c[y][x] = c[y][x - 1];
                b[y][x] = Direction::Left;
            }
        }
    }

    let mut w = vec![];
    recontruct_lcs(&mut w, &b, xs, xl, yl);
    w.reverse();
    w
}

fn recontruct_lcs(w: &mut Vec<u8>, b: &Array2D<Direction>, xs: &[u8], x: usize, y: usize) {
    if x == 0 || y == 0 {
        return;
    }

    match b[y][x] {
        Direction::Unset => panic!("direction should be set"),
        Direction::Up => recontruct_lcs(w, b, xs, x, y - 1),
        Direction::Left => recontruct_lcs(w, b, xs, x - 1, y),
        Direction::UpLeft => {
            w.push(xs[x - 1]);
            recontruct_lcs(w, b, xs, x - 1, y - 1);
        }
    }
}

pub fn longest_increasing_subsequence(a: &[u64]) -> Vec<u64> {
    let n = a.len();
    let mut m = vec![0; n + 1];
    let mut p = vec![None; n + 1];

    for i in 0..n {
        m[i] = 1;

        for j in 0..i {
            let q = 1 + m[j];

            if a[j] < a[i] && q > m[i] {
                m[i] = q;
                p[i] = Some(j);
            }
        }
    }

    let mut max = m[0];
    let mut idx = 0;

    for (i, el) in m.iter().enumerate() {
        if *el > max {
            max = *el;
            idx = i;
        }
    }

    let mut idx = Some(idx);
    let mut subs = vec![];

    while let Some(i) = idx {
        subs.push(a[i]);
        idx = p[i];
    }

    subs.reverse();

    subs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rod_cutting_test() {
        let prices = [0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];

        assert_eq!((30, vec![10]), rod_cutting_extended(&prices, 10));
        assert_eq!((18, vec![1, 6]), rod_cutting_extended(&prices, 7));
    }

    #[test]
    fn fib_test() {
        for n in 0..=10 {
            assert_eq!(fib(n), fib_naive(n));
            assert_eq!(fib2(n), fib_naive(n));
        }
    }

    fn fib_naive(n: usize) -> usize {
        match n {
            0 => 0,
            1 => 1,
            x => fib_naive(x - 1) + fib_naive(x - 2),
        }
    }

    #[test]
    fn matrix_parenthesization_test() {
        let dimensions = [30, 35, 15, 5, 10, 20, 25];
        let p = matrix_parenthesization(&dimensions);

        assert_eq!(p, "((A1(A2A3))((A4A5)A6))");
    }

    #[test]
    fn longest_common_subsequence_test() {
        let s1 = b"springtime";
        let s2 = b"pioneer";
        let lcs = b"pine";
        assert_eq!(lcs.to_vec(), longest_common_subsequence(s1, s2));

        let s1 = b"abcbdab";
        let s2 = b"bdcaba";
        let lcs = b"bdab";
        assert_eq!(lcs.to_vec(), longest_common_subsequence(s1, s2));

        let s1 = b"ACCGGTCGAGTGCGCGGAAGCCGGCCGAA";
        let s2 = b"GTCGTTCGGAATGCCGTTGCTCTGTAAA";
        let lcs = b"GTCGTCGGAAGCCGGCCGAA";
        assert_eq!(lcs.to_vec(), longest_common_subsequence(s1, s2));
    }

    #[test]
    fn longest_increasing_subsequence_test() {
        let xs = [8, 3, 4, 6, 5, 2, 0, 7, 9, 1];
        let lis = [3, 4, 6, 7, 9];
        assert_eq!(lis.to_vec(), longest_increasing_subsequence(&xs));

        let xs = [8, 1, 2, 6, 5, 7, 3, 9, 4, 10];
        let lis = [1, 2, 6, 7, 9, 10];
        assert_eq!(lis.to_vec(), longest_increasing_subsequence(&xs));
    }
}
