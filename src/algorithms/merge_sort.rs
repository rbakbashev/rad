pub fn merge_sort<T: Default + Copy + PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    merge_rec(a, 0, a.len() - 1);
}

fn merge_rec<T: Default + Copy + PartialOrd>(a: &mut [T], l: usize, h: usize) {
    let m = (l + h) / 2;

    if l == h {
        return;
    }

    merge_rec(a, l, m);
    merge_rec(a, m + 1, h);
    merge(a, l, m, h);
}

fn merge<T: Default + Copy + PartialOrd>(a: &mut [T], l: usize, m: usize, h: usize) {
    let llen = m - l + 1;
    let rlen = h - m;
    let mut left = Vec::new();
    let mut right = Vec::new();

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
}

#[cfg(test)]
#[test]
fn test() {
    crate::tests::test_sort(merge_sort);
}
