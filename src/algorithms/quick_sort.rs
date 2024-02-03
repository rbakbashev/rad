pub fn quicksort<T: PartialOrd>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    quicksort_aux(a, 0, a.len() - 1);
}

fn quicksort_aux<T: PartialOrd>(a: &mut [T], l: usize, h: usize) {
    if l >= h {
        return;
    }

    let p = partition_hoare(a, l, h);

    quicksort_aux(a, l, p);
    quicksort_aux(a, p + 1, h);
}

fn partition_hoare<T: PartialOrd>(a: &mut [T], l: usize, h: usize) -> usize {
    let mut ineg = l == 0;
    let mut i = if ineg { l } else { l - 1 };
    let mut j = h + 1;

    loop {
        loop {
            if !ineg {
                ineg = false;
                i += 1;
            }

            if a[i] >= a[l] {
                break;
            }
        }

        loop {
            j -= 1;

            if a[j] <= a[l] {
                break;
            }
        }

        if i < j {
            a.swap(i, j);
        } else {
            return j;
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    crate::tests::test_sort(quicksort);
}
