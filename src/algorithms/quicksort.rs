use crate::rand::Wyhash64RNG;

pub fn quicksort<T: PartialOrd + Copy>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    quicksort_aux(a, 0, a.len() - 1);
}

fn quicksort_aux<T: PartialOrd + Copy>(a: &mut [T], l: usize, h: usize) {
    if l >= h {
        return;
    }

    let p = partition_hoare(a, l, h);

    quicksort_aux(a, l, p);
    quicksort_aux(a, p + 1, h);
}

fn partition_hoare<T: PartialOrd + Copy>(a: &mut [T], l: usize, h: usize) -> usize {
    let mut ineg = l == 0;
    let mut i = if ineg { l } else { l - 1 };
    let mut j = h + 1;
    let pivot = a[l];

    loop {
        loop {
            if ineg {
                ineg = false;
            } else {
                i += 1;
            }

            if a[i] >= pivot {
                break;
            }
        }

        loop {
            j -= 1;

            if a[j] <= pivot {
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

pub fn randomized_quicksort<T: PartialOrd + Copy>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let mut rng = Wyhash64RNG::from_seed(123);

    randomized_quicksort_aux(a, &mut rng, 0, a.len() - 1);
}

fn randomized_quicksort_aux<T: PartialOrd + Copy>(
    a: &mut [T],
    rng: &mut Wyhash64RNG,
    l: usize,
    h: usize,
) {
    if l >= h {
        return;
    }

    let p = randomized_partition(a, rng, l, h);

    randomized_quicksort_aux(a, rng, l, p);
    randomized_quicksort_aux(a, rng, p + 1, h);
}

#[allow(clippy::range_plus_one, clippy::cast_possible_truncation)]
fn randomized_partition<T: PartialOrd + Copy>(
    a: &mut [T],
    rng: &mut Wyhash64RNG,
    l: usize,
    h: usize,
) -> usize {
    let range = (l as u64)..(h as u64);
    let pivot = rng.gen_in_range(range) as usize;

    a.swap(l, pivot);

    partition_hoare(a, l, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        crate::tests::test_sort(quicksort);
    }

    #[test]
    fn randomized() {
        crate::tests::test_sort(randomized_quicksort);
    }
}
