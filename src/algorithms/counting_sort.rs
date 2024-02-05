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

    let (min, max) = find_min_max(xs);

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

fn find_min_max<T: PartialOrd + Copy>(xs: &[T]) -> (T, T) {
    let mut min = xs[0];
    let mut max = xs[0];

    for &x in xs {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
