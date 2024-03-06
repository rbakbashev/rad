pub fn merge_sort<T: Copy + PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    merge_rec(a, 0, a.len() - 1);
}

fn merge_rec<T: Copy + PartialOrd>(a: &mut [T], l: usize, h: usize) {
    let m = l + (h - l) / 2;

    if l == h {
        return;
    }

    merge_rec(a, l, m);
    merge_rec(a, m + 1, h);
    merge(a, l, m, h);
}

fn merge<T: Copy + PartialOrd>(a: &mut [T], l: usize, m: usize, h: usize) {
    let left = &a[l..=m].to_vec();
    let right = &a[m + 1..=h].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i != left.len() {
        a[k] = left[i];
        i += 1;
        k += 1;
    }

    while j != right.len() {
        a[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[test]
fn test() {
    crate::tests::test_sort(merge_sort);
}
