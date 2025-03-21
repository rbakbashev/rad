pub fn count_inversions<T: Copy + PartialOrd>(a: &mut [T]) -> usize {
    count_inv_rec(a, 0, a.len() - 1)
}

fn count_inv_rec<T: Copy + PartialOrd>(a: &mut [T], l: usize, h: usize) -> usize {
    if l == h {
        return 0;
    }

    let m = l + (h - l) / 2;
    let left = count_inv_rec(a, l, m);
    let right = count_inv_rec(a, m + 1, h);

    left + right + count_inv_in_subrange(a, l, m, h)
}

fn count_inv_in_subrange<T: Copy + PartialOrd>(a: &mut [T], l: usize, m: usize, h: usize) -> usize {
    let left = a[l..=m].to_vec();
    let right = a[m + 1..=h].to_vec();

    let mut inv = 0;

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
            inv += left.len() - i;
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

    inv
}

#[test]
fn test() {
    let mut v = vec![2, 3, 8, 6, 1];

    assert_eq!(count_inversions(&mut v), 5);
}
