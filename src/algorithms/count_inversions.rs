pub fn count_inversions<T: Default + Copy + PartialOrd>(a: &mut [T]) -> usize {
    count_inv_rec(a, 0, a.len() - 1)
}

fn count_inv_rec<T: Default + Copy + PartialOrd>(a: &mut [T], l: usize, h: usize) -> usize {
    if l == h {
        return 0;
    }

    let m = l + (h - l) / 2;
    let left = count_inv_rec(a, l, m);
    let right = count_inv_rec(a, m + 1, h);

    left + right + count_inv_in_subranges(a, l, m, h)
}

fn count_inv_in_subranges<T: Default + Copy + PartialOrd>(
    a: &mut [T],
    l: usize,
    m: usize,
    h: usize,
) -> usize {
    let llen = m - l + 1;
    let rlen = h - m;
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut inv = 0;

    left.resize_with(llen, Default::default);
    right.resize_with(rlen, Default::default);

    left.copy_from_slice(&a[l..=m]);
    right.copy_from_slice(&a[m + 1..=h]);

    let mut i = 0;
    let mut j = 0;
    let mut k = l;
    while i < llen && j < rlen {
        if left[i] <= right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
            inv += llen - i;
        }
        k += 1;
    }

    while i != llen {
        a[k] = left[i];
        i += 1;
        k += 1;
    }

    while j != rlen {
        a[k] = right[j];
        j += 1;
        k += 1;
    }

    inv
}

#[cfg(test)]
#[test]
fn test() {
    let mut v = vec![2, 3, 8, 6, 1];

    assert_eq!(count_inversions(&mut v), 5);
}
