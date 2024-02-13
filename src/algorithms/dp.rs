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
    reconstruct_lcs(&mut w, &b, xs, xl, yl);
    w.reverse();
    w
}

fn reconstruct_lcs(w: &mut Vec<u8>, b: &Array2D<Direction>, xs: &[u8], x: usize, y: usize) {
    if x == 0 || y == 0 {
        return;
    }

    match b[y][x] {
        Direction::Unset => panic!("direction should be set"),
        Direction::Up => reconstruct_lcs(w, b, xs, x, y - 1),
        Direction::Left => reconstruct_lcs(w, b, xs, x - 1, y),
        Direction::UpLeft => {
            w.push(xs[x - 1]);
            reconstruct_lcs(w, b, xs, x - 1, y - 1);
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

pub fn longest_increasing_subsequence_2(xs: &[usize]) -> Vec<usize> {
    let mut m = vec![0; xs.len() + 1];
    let mut p = vec![0; xs.len()];

    m[0] = 0;

    let mut len = 0;

    for i in 0..xs.len() {
        let mut l = 1;
        let mut h = len + 1;

        while l < h {
            let mid = l + (h - l) / 2;

            if xs[m[mid]] >= xs[i] {
                h = mid;
            } else {
                l = mid + 1;
            }
        }

        p[i] = m[l - 1];
        m[l] = i;

        if l > len {
            len = l;
        }
    }

    let mut idx = m[len];
    let mut subs = vec![];

    for _ in 0..len {
        subs.push(xs[idx]);
        idx = p[idx];
    }

    subs.reverse();

    subs
}

#[derive(Clone, Copy, PartialEq)]
enum PalDirection {
    End,
    Char(u8),
    Left,
    Right,
}

pub fn longest_palindrome_subsequence(a: &[u8]) -> Vec<u8> {
    let n = a.len();
    let mut m = Array2D::new(0, n, n + 1);
    let mut b = Array2D::new(PalDirection::End, n, n + 1);

    for (s, a) in a.iter().enumerate() {
        m[1][s] = 1;
        b[1][s] = PalDirection::Char(*a);
    }

    for l in 2..=n {
        let mut s = 0;

        while s + l <= n {
            if a[s] == a[s + l - 1] {
                m[l][s] = 2 + m[l - 2][s + 1];
                b[l][s] = PalDirection::Char(a[s]);
            } else {
                let ll = m[l - 1][s];
                let lr = m[l - 1][s + 1];

                if ll > lr {
                    m[l][s] = ll;
                    b[l][s] = PalDirection::Left;
                } else {
                    m[l][s] = lr;
                    b[l][s] = PalDirection::Right;
                }
            }

            s += 1;
        }
    }

    reconstruct_lps(&m, &b, n)
}

fn reconstruct_lps(m: &Array2D<usize>, b: &Array2D<PalDirection>, n: usize) -> Vec<u8> {
    let len = m[n][0];

    let mut o = Vec::with_capacity(len);
    let mut s = 0;
    let mut l = n;

    loop {
        match b[l][s] {
            PalDirection::End => break,
            PalDirection::Char(c) => {
                o.push(c);
                s += 1;

                if l < 2 {
                    break;
                }

                l -= 2;
            }
            PalDirection::Left => {
                l -= 1;
            }
            PalDirection::Right => {
                s += 1;
                l -= 1;
            }
        }
    }

    let mut i = len / 2 - 1;

    while o.len() != len {
        o.push(o[i]);

        if i == 0 {
            break;
        }

        i -= 1;
    }

    o
}

#[allow(clippy::cast_possible_wrap)] // CBA
pub fn printing_neatly(words: &[&'static str], width: isize) -> Vec<String> {
    let n = words.len();
    let mut extras = Array2D::new(0, n + 1, n + 1);
    let mut linecost = Array2D::new(None, n + 1, n + 1);
    let mut startcost = vec![None; n + 1];
    let mut pred = vec![0; n + 1];

    for i in 1..=n {
        extras[i][i] = width - words[i - 1].len() as isize;

        for j in i + 1..=n {
            extras[i][j] = extras[i][j - 1] - words[j - 1].len() as isize - 1;
        }
    }

    for i in 1..=n {
        for j in i..=n {
            linecost[i][j] = if extras[i][j] < 0 {
                None
            } else if j == n && extras[i][j] >= 0 {
                Some(0)
            } else {
                Some(extras[i][j].pow(3))
            };
        }
    }

    startcost[0] = Some(0);

    for j in 1..=n {
        startcost[j] = None;

        for i in 1..=j {
            let Some(prev) = startcost[i - 1] else {
                continue;
            };
            let Some(line) = linecost[i][j] else { continue };

            let cost = prev + line;

            if startcost[j].is_none() || startcost[j].is_some_and(|s| cost < s) {
                startcost[j] = Some(cost);
                pred[j] = i;
            }
        }
    }

    let len = number_printed_lines(&pred, n);
    let mut lines = vec![&words[0..0]; len];

    reconstruct_neat_print(&mut lines, words, &pred, n);

    join_lines(&lines)
}

fn number_printed_lines(pred: &[usize], j: usize) -> usize {
    let i = pred[j];

    if i == 1 {
        1
    } else {
        number_printed_lines(pred, i - 1) + 1
    }
}

fn reconstruct_neat_print<'w>(
    lines: &mut [&'w [&str]],
    words: &'w [&'static str],
    pred: &[usize],
    j: usize,
) -> usize {
    let i = pred[j];

    let k = if i == 1 {
        1
    } else {
        reconstruct_neat_print(lines, words, pred, i - 1) + 1
    };

    lines[k - 1] = &words[i - 1..j];

    k
}

fn join_lines(lines: &[&[&str]]) -> Vec<String> {
    let mut paragraph = vec![String::new(); lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (head, tail) = line.split_at(1);

        paragraph[i].push_str(head[0]);

        for word in tail {
            paragraph[i].push(' ');
            paragraph[i].push_str(word);
        }
    }

    paragraph
}

#[derive(Clone, Copy)]
pub enum EditOperations {
    Unset,
    Copy(char),
    Replace(char, char),
    Delete(char),
    Insert(char),
    End,
}

pub fn edit_distance(x: &str, y: &str) -> Vec<EditOperations> {
    let m = x.len();
    let n = y.len();
    let mut d = Array2D::new(0, n + 1, m + 1);
    let mut p = Array2D::new(EditOperations::Unset, n + 1, m + 1);

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                d[i][j] = j;
                p[i][j] = EditOperations::End;
                continue;
            }

            if j == 0 {
                d[i][j] = i;
                p[i][j] = EditOperations::End;
                continue;
            }

            let xp = x.as_bytes()[i - 1] as char;
            let yp = y.as_bytes()[j - 1] as char;

            if xp == yp {
                d[i][j] = d[i - 1][j - 1];
                p[i][j] = EditOperations::Copy(xp);
            } else {
                let ins = d[i][j - 1];
                let del = d[i - 1][j];
                let rep = d[i - 1][j - 1];

                if ins < del && ins < rep {
                    d[i][j] = 1 + ins;
                    p[i][j] = EditOperations::Insert(yp);
                } else if del < ins && del < rep {
                    d[i][j] = 1 + del;
                    p[i][j] = EditOperations::Delete(xp);
                } else {
                    d[i][j] = 1 + rep;
                    p[i][j] = EditOperations::Replace(xp, yp);
                }
            }
        }
    }

    reconstruct_edit_distance(&p, m, n)
}

fn reconstruct_edit_distance(
    p: &Array2D<EditOperations>,
    mut i: usize,
    mut j: usize,
) -> Vec<EditOperations> {
    let mut ops = vec![];

    loop {
        let op = p[i][j];

        match op {
            EditOperations::Unset => panic!("edit operation should be set"),
            EditOperations::Copy(_x) => {
                i -= 1;
                j -= 1;
            }
            EditOperations::Replace(_x, _y) => {
                i -= 1;
                j -= 1;
            }
            EditOperations::Delete(_x) => {
                i -= 1;
            }
            EditOperations::Insert(_x) => {
                j -= 1;
            }
            EditOperations::End => break,
        }

        ops.push(op);
    }

    ops.reverse();

    ops
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
        assert_eq!(lcs, longest_common_subsequence(s1, s2).as_slice());

        let s1 = b"abcbdab";
        let s2 = b"bdcaba";
        let lcs = b"bdab";
        assert_eq!(lcs, longest_common_subsequence(s1, s2).as_slice());

        let s1 = b"ACCGGTCGAGTGCGCGGAAGCCGGCCGAA";
        let s2 = b"GTCGTTCGGAATGCCGTTGCTCTGTAAA";
        let lcs = b"GTCGTCGGAAGCCGGCCGAA";
        assert_eq!(lcs, longest_common_subsequence(s1, s2).as_slice());
    }

    #[test]
    fn longest_increasing_subsequence_test() {
        let xs = [8, 3, 4, 6, 5, 2, 0, 7, 9, 1];
        let lis = [3, 4, 6, 7, 9];
        assert_eq!(lis, longest_increasing_subsequence(&xs).as_slice());

        let xs = [8, 1, 2, 6, 5, 7, 3, 9, 4, 10];
        let lis = [1, 2, 6, 7, 9, 10];
        assert_eq!(lis, longest_increasing_subsequence(&xs).as_slice());
    }

    #[test]
    fn longest_increasing_subsequence_2_test() {
        let xs = [8, 3, 4, 6, 5, 2, 0, 7, 9, 1];
        let lis = [3, 4, 5, 7, 9];
        assert_eq!(lis, longest_increasing_subsequence_2(&xs).as_slice());

        let xs = [8, 1, 2, 6, 5, 7, 3, 9, 4, 10];
        let lis = [1, 2, 5, 7, 9, 10];
        assert_eq!(lis, longest_increasing_subsequence_2(&xs).as_slice());
    }

    #[test]
    fn longest_palindrome_subsequence_test() {
        let tst = b"character";
        let ans = b"carac";
        assert_eq!(ans, longest_palindrome_subsequence(tst).as_slice());

        let tst = b"xdRAfdfCECA123R_";
        let ans = b"RACECAR";
        assert_eq!(ans, longest_palindrome_subsequence(tst).as_slice());
    }

    #[test]
    #[rustfmt::skip]
    fn printing_neatly_test() {
        let words = [
            "priesthood", "piccolos", "cuisine", "veneers", "enrichment", "bids", "rightest",
            "endue", "facsimiled", "bareback", "spams", "slice", "atelier", "cuddles", "weekdays",
            "shibboleth", "introvert", "stooges", "rosebush", "acolytes", "armistice", "bishop",
            "sect", "plundering", "obstinate", "yearling", "slinks", "megaliths", "handsomer",
            "chigger", "valve", "outsources", "thralls", "satirical", "aplomb", "cursed",
            "backpedal", "dunce", "classicist", "butternut", "spurns", "trend", "ensconcing",
            "torrents", "revive", "mosquitos", "stragglier", "ghost", "nailing", "appending",
        ];

        let fmt = [
            "priesthood piccolos cuisine veneers",
            "enrichment bids rightest endue",
            "facsimiled bareback spams slice atelier",
            "cuddles weekdays shibboleth introvert",
            "stooges rosebush acolytes armistice",
            "bishop sect plundering obstinate",
            "yearling slinks megaliths handsomer",
            "chigger valve outsources thralls",
            "satirical aplomb cursed backpedal",
            "dunce classicist butternut spurns trend",
            "ensconcing torrents revive mosquitos",
            "stragglier ghost nailing appending",
        ];

        let ans = printing_neatly(&words, 40);

        assert_eq!(&fmt, ans.as_slice());
    }

    #[test]
    fn edit_distance_test() {
        let x = "algorithm";
        let y = "altruistic";

        assert_eq!(10, edit_distance(x, y).len());
    }
}
