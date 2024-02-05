pub fn radix_sort(xs: &mut [u64]) {
    if xs.len() <= 1 {
        return;
    }

    let Some(max) = xs.iter().max() else {
        return;
    };
    let max = *max;

    let mut b = vec![0; xs.len()];
    let mut p = 1;

    while max / p > 0 {
        let mut c = [0; 10];

        for x in xs.iter() {
            let d = ((x / p) % 10) as usize;
            c[d] += 1;
        }

        for i in 1..10 {
            c[i] += c[i - 1];
        }

        for x in xs.iter().rev() {
            let d = ((x / p) % 10) as usize;
            b[c[d] - 1] = *x;
            c[d] -= 1;
        }

        xs.copy_from_slice(b.as_slice());

        p *= 10;
    }
}

#[test]
fn test() {
    crate::tests::test_sort(radix_sort);
}
