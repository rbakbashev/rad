use crate::algorithms::insertion_sort::insertion_sort;

pub fn select<T: Ord + Copy + Default>(a: &mut [T], k: usize) -> Option<T> {
    if a.is_empty() {
        return None;
    }

    select_aux(a, 0, a.len() - 1, k)
}

fn select_aux<T: Ord + Copy + Default>(a: &mut [T], l: usize, h: usize, k: usize) -> Option<T> {
    let n = h - l + 1;

    if k == 0 || k > n {
        return None;
    }

    let mut medians = Vec::with_capacity(n / 5 + 1);

    for group in a[l..=h].chunks(5) {
        let mut buf = group.to_vec();

        insertion_sort(&mut buf);

        let n = buf.len();
        let median = buf[(n + 1) / 2 - 1];

        medians.push(median);
    }

    let ml = medians.len();
    let mk = (ml + 1) / 2;
    let mm = if ml == 1 {
        medians[0]
    } else {
        select_aux(&mut medians, 0, ml - 1, mk).expect("median should be available")
    };

    let p = partition_lomuto(a, l, h, mm);
    let pl = p - l + 1;

    if pl == k {
        return Some(mm);
    }

    if pl > k {
        select_aux(a, l, p - 1, k)
    } else {
        select_aux(a, p + 1, h, k - pl)
    }
}

fn partition_lomuto<T: PartialOrd + Copy>(a: &mut [T], l: usize, h: usize, pivot: T) -> usize {
    for p in l..=h {
        if a[p] == pivot {
            a.swap(p, h);
            break;
        }
    }

    let mut i = l;

    for j in l..h {
        if a[j] <= a[h] {
            a.swap(i, j);
            i += 1;
        }
    }

    a.swap(i, h);

    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    #[test]
    fn test() {
        let len = 10;
        let ks = [1, 2, len / 3, len / 2, len * 2 / 3, len - 1, len];
        let arrays = tests::generate_test_arrays(len);

        for k in ks {
            for (desc, arr) in &arrays {
                println!("Array: {}", desc);
                single_test(&arr, k);
            }
        }
    }

    fn single_test(a: &[u64], k: usize) {
        let mut copy = a.to_vec();
        let naive = select_naive(&a, k);
        let check = select(&mut copy, k);

        assert_eq!(naive, check);
    }

    fn select_naive<T: Ord + Copy + Default>(a: &[T], k: usize) -> Option<T> {
        let mut copy = a.to_vec();
        copy.sort_unstable();
        copy.get(k - 1).map(|x| *x)
    }
}
