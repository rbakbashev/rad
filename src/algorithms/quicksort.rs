use crate::rand::Wyhash64RNG;

pub fn quicksort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    quicksort_rec(arr, 0, arr.len() - 1);
}

fn quicksort_rec<T: PartialOrd + Copy>(arr: &mut [T], low: usize, high: usize) {
    if high <= low {
        return;
    }

    let p = partition_hoare(arr, low, high);

    quicksort_rec(arr, low, p);
    quicksort_rec(arr, p + 1, high);
}

fn partition_hoare<T: PartialOrd + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut l = low.wrapping_sub(1);
    let mut r = high.wrapping_add(1);

    let pivot = arr[low];

    loop {
        loop {
            l = l.wrapping_add(1);

            if arr[l] >= pivot {
                break;
            }
        }

        loop {
            r = r.wrapping_sub(1);

            if arr[r] <= pivot {
                break;
            }
        }

        if r <= l {
            return r;
        }

        arr.swap(l, r);
    }
}

pub fn randomized_quicksort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut rng = Wyhash64RNG::from_seed(123);

    randomized_quicksort_rec(arr, &mut rng, 0, arr.len() - 1);
}

fn randomized_quicksort_rec<T: PartialOrd + Copy>(
    arr: &mut [T],
    rng: &mut Wyhash64RNG,
    low: usize,
    high: usize,
) {
    if low >= high {
        return;
    }

    let p = randomized_partition(arr, rng, low, high);

    randomized_quicksort_rec(arr, rng, low, p);
    randomized_quicksort_rec(arr, rng, p + 1, high);
}

#[allow(clippy::cast_possible_truncation)]
fn randomized_partition<T: PartialOrd + Copy>(
    arr: &mut [T],
    rng: &mut Wyhash64RNG,
    low: usize,
    high: usize,
) -> usize {
    let range = (low as u64)..(high as u64);
    let pivot = rng.gen_in_range(range) as usize;

    arr.swap(low, pivot);

    partition_hoare(arr, low, high)
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
