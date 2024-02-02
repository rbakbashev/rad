use std::mem::swap;

#[allow(clippy::needless_range_loop)]
fn cycle_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let n = arr.len();

    for lo in 0..n - 1 {
        let mut x = arr[lo];
        let mut idx = lo;

        for i in lo + 1..n {
            if arr[i] < x {
                idx += 1;
            }
        }

        if idx == lo {
            continue;
        }

        while x == arr[idx] {
            idx += 1;
        }

        if idx != lo {
            swap(&mut arr[idx], &mut x);
        }

        while idx != lo {
            idx = lo;

            for i in lo + 1..n {
                if arr[i] < x {
                    idx += 1;
                }
            }

            while x == arr[idx] {
                idx += 1;
            }

            if x != arr[idx] {
                swap(&mut arr[idx], &mut x);
            }
        }
    }
}

#[test]
fn test() {
    crate::tests::test_sort(cycle_sort);
}
