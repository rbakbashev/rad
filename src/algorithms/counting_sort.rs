pub fn counting_sort(xs: &mut [usize]) {
    if xs.len() <= 1 {
        return;
    }

    let Some(max) = xs.iter().max() else {
        return;
    };
    let max = *max;

    let mut c = vec![0; max + 1];
    let mut out = vec![0; xs.len() + 1];

    for x in xs.iter() {
        c[*x] += 1;
    }

    for i in 1..=max {
        c[i] += c[i - 1];
    }

    for x in xs.iter().rev() {
        out[c[*x]] = *x;
        c[*x] -= 1;
    }

    xs.copy_from_slice(&out[1..=xs.len()]);
}

pub fn counting_sort_2(xs: &mut [usize]) {
    if xs.len() <= 1 {
        return;
    }

    let Some((min, max)) = find_min_max(xs) else {
        return;
    };

    let range = max - min + 1;
    let mut c = vec![0; range];

    for x in xs.iter() {
        c[*x - min] += 1;
    }

    let mut idx = 0;
    for i in min..=max {
        for _ in 0..c[i - min] {
            xs[idx] = i;
            idx += 1;
        }
    }
}

fn find_min_max<T: PartialOrd + Copy>(xs: &[T]) -> Option<(T, T)> {
    let (mut min, mut max, offset) = match xs.len() {
        0 => return None,
        1 => return Some((xs[0], xs[0])),
        n if n % 2 == 0 => {
            let x0 = xs[0];
            let x1 = xs[1];
            let off = 2;
            if x0 < x1 {
                (x0, x1, off)
            } else {
                (x1, x0, off)
            }
        }
        _ => (xs[0], xs[0], 1),
    };

    let skip = &xs[offset..xs.len()];

    for ch in skip.chunks(2) {
        match ch {
            [a, b] => {
                let (nmin, nmax) = if *a < *b { (*a, *b) } else { (*b, *a) };

                if nmin < min {
                    min = nmin;
                }
                if nmax > max {
                    max = nmax;
                }
            }
            [a] => {
                let a = *a;
                if a < min {
                    min = a;
                }
                if a > max {
                    max = a;
                }
            }
            _ => unreachable!(),
        }
    }

    Some((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    fn single_test(f: fn(&mut [usize])) {
        let wrap = move |a: &mut [u64]| {
            let mut copy = copy_to_usize(a);
            f(&mut copy);
            write_to_u64(&copy, a);
        };

        crate::tests::test_sort(wrap);
    }

    fn copy_to_usize(a: &[u64]) -> Vec<usize> {
        a.iter().map(|x| usize::try_from(*x).unwrap()).collect()
    }

    fn write_to_u64(src: &[usize], dst: &mut [u64]) {
        for (i, x) in src.iter().enumerate() {
            dst[i] = *x as u64;
        }
    }

    #[test]
    fn normal() {
        single_test(counting_sort);
    }

    #[test]
    fn alternate() {
        single_test(counting_sort_2);
    }

    #[test]
    fn min_max() {
        let len = 1000;
        let arrays = tests::generate_test_arrays(len);

        for (desc, arr) in arrays {
            println!("Array: {}", desc);
            min_max_single_test(arr);
        }
    }

    fn min_max_single_test(a: Vec<u64>) {
        assert_eq!(find_min_max(&a).map(|t| t.0), a.iter().min().copied());
        assert_eq!(find_min_max(&a).map(|t| t.1), a.iter().max().copied());
    }
}
