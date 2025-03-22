#![allow(clippy::needless_range_loop)]

pub fn max_subarray_sum(a: &[i64]) -> (usize, usize, i64) {
    max_subarray_rec(a, 0, a.len() - 1)
}

fn max_subarray_rec(a: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if low == high {
        return (low, high, a[low]);
    }

    let mid = low + (high - low) / 2;

    let (left_low, left_high, left_sum) = max_subarray_rec(a, low, mid);
    let (right_low, right_high, right_sum) = max_subarray_rec(a, mid + 1, high);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(a, low, mid, high);

    if left_sum >= right_sum && left_sum >= cross_sum {
        (left_low, left_high, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_low, right_high, right_sum)
    } else {
        (cross_low, cross_high, cross_sum)
    }
}

fn find_max_crossing_subarray(
    a: &[i64],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i64) {
    let mut sum = 0;
    let mut left_sum = None;
    let mut left_idx = 0;

    for i in (low..=mid).rev() {
        sum += a[i];

        if left_sum.is_none() || left_sum.is_some_and(|ls| sum > ls) {
            left_sum = Some(sum);
            left_idx = i;
        }
    }

    let mut sum = 0;
    let mut right_sum = None;
    let mut right_idx = 0;

    for j in mid + 1..high {
        sum += a[j];

        if right_sum.is_none() || right_sum.is_some_and(|rs| sum > rs) {
            right_sum = Some(sum);
            right_idx = j;
        }
    }

    let sum = left_sum.unwrap_or(0) + right_sum.unwrap_or(0);

    (left_idx, right_idx, sum)
}

pub fn max_subarray_sum_linear(a: &[i64]) -> (usize, usize, i64) {
    let mut sum = a[0];
    let mut beg = 0;
    let mut end = 0;
    let mut max = 0;

    for i in 1..a.len() {
        if sum < 0 {
            sum = a[i];
            beg = i;
            end = i;
        } else {
            sum += a[i];
        }

        if sum > max {
            max = sum;
            end = i;
        }
    }

    (beg, end, max)
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use crate::rand::Wyhash64RNG;

    const CASE1: &[i64] = &[
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    const CASE1_ANS: (usize, usize, i64) = (7, 10, 43);

    const CASE2: &[i64] = &[1, -4, 3, -4];
    const CASE2_ANS: (usize, usize, i64) = (2, 2, 3);

    #[test]
    fn dq_simple() {
        assert_eq!(CASE1_ANS, max_subarray_sum(CASE1));
        assert_eq!(CASE2_ANS, max_subarray_sum(CASE2));
    }

    #[test]
    fn linear_simple() {
        assert_eq!(CASE1_ANS, max_subarray_sum_linear(CASE1));
        assert_eq!(CASE2_ANS, max_subarray_sum_linear(CASE2));
    }

    #[test]
    fn dq_random() {
        test_subarray_func(max_subarray_sum);
    }

    #[test]
    fn linear_random() {
        test_subarray_func(max_subarray_sum_linear);
    }

    fn test_subarray_func(func: impl Fn(&[i64]) -> (usize, usize, i64)) {
        let cases = 20;
        let sizes = if cfg!(miri) { 50 } else { 200 };
        let range = -100..100;

        for _ in 0..cases {
            let array = generate_array(sizes, &range);
            let (_, _, ans_naive) = max_subarray_naive(&array);
            let (_, _, ans_tested) = func(&array);

            assert_eq!(ans_naive, ans_tested);
        }
    }

    fn generate_array(n: usize, range: &Range<i64>) -> Vec<i64> {
        let seed = 123;
        let mut vec = Vec::with_capacity(n);
        let mut rng = Wyhash64RNG::from_seed(seed);

        for _ in 0..n {
            vec.push(rng.gen_in_range_i64(range.clone()));
        }

        vec
    }

    fn max_subarray_naive(a: &[i64]) -> (usize, usize, i64) {
        let mut max_sum = a[0];
        let mut max_beg = 0;
        let mut max_end = 0;

        for end in 0..a.len() {
            for beg in 0..end {
                let sum = a[beg..end].iter().sum::<i64>();

                if sum > max_sum {
                    max_sum = sum;
                    max_beg = beg;
                    max_end = end;
                }
            }
        }

        (max_beg, max_end, max_sum)
    }
}
