#![allow(clippy::needless_range_loop)]

pub fn max_subarray_sum(a: &[i64]) -> (usize, usize, i64) {
    max_subarray_aux(a, 0, a.len() - 1)
}

fn max_subarray_aux(a: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if low == high {
        return (low, high, a[low]);
    }

    let mid = low + (high - low) / 2;

    let (left_low, left_high, left_sum) = max_subarray_aux(a, low, mid);
    let (right_low, right_high, right_sum) = max_subarray_aux(a, mid + 1, high);
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
    let mut sum = 0;
    let mut beg = 0;
    let mut end = 0;
    let mut max_sum = 0;

    for i in 1..a.len() {
        if sum < 0 {
            sum = a[i];
            beg = i;
            end = i;
        } else {
            sum += a[i];
        }

        if sum > max_sum {
            max_sum = sum;
            end = i;
        }
    }

    (beg, end, max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE1: &[i64] = &[
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    const CASE2: &[i64] = &[1, -4, 3, -4];

    #[test]
    fn dq_1() {
        assert_eq!((7, 10, 43), max_subarray_sum(CASE1));
    }

    #[test]
    fn dq_2() {
        assert_eq!((2, 2, 3), max_subarray_sum(CASE2));
    }

    #[test]
    fn linear_1() {
        assert_eq!((7, 10, 43), max_subarray_sum_linear(CASE1));
    }

    #[test]
    fn linear_2() {
        assert_eq!((2, 2, 3), max_subarray_sum_linear(CASE2));
    }
}
