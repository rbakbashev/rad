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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rod_cutting() {
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
}
