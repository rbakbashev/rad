use std::collections::HashMap;

use crate::rand::Wyhash64RNG;

const TEST_ARRAY_LEN: usize = 1000;
const RAND_SEED: u64 = 123;

pub fn test_sort(f: fn(&mut [u64])) {
    let len = TEST_ARRAY_LEN;
    let id = generate_array_identical(len, 1);
    let asc = generate_array_ascending(len);
    let desc = generate_array_descending(len);
    let rand = generate_array_random(len, 1, len as u64);
    let perm = generate_array_permuation(len);

    test_sort_single(f, id);
    test_sort_single(f, asc);
    test_sort_single(f, desc);
    test_sort_single(f, rand);
    test_sort_single(f, perm);
}

fn generate_array_identical<T: Copy>(n: usize, x: T) -> Vec<T> {
    vec![x; n]
}

fn generate_array_ascending<T: From<u64>>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((i as u64).into());
    }
    v
}

fn generate_array_descending<T: From<u64>>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(((n - i) as u64).into());
    }
    v
}

fn generate_array_random(n: usize, lower: u64, upper: u64) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    let mut r = Wyhash64RNG::from_seed(RAND_SEED);

    for _ in 0..n {
        v.push(r.gen_in_range(lower..upper));
    }

    v
}

fn generate_array_permuation<T: From<u64>>(n: usize) -> Vec<T> {
    let mut v = generate_array_ascending(n);
    permute(&mut v);
    v
}

fn permute<T>(v: &mut [T]) {
    let mut r = Wyhash64RNG::from_seed(RAND_SEED);

    for i in 0..v.len() {
        let j = r.gen_in_range(0..v.len() as u64) as usize;

        v.swap(i, j);
    }
}

fn test_sort_single(f: fn(&mut [u64]), mut values: Vec<u64>) {
    let orig = values.clone();

    f(&mut values);

    assert_sorted(&values);

    let old_counts = count_elements(&orig);
    let new_counts = count_elements(&values);

    assert_eq!(old_counts, new_counts);
}

fn assert_sorted<T: PartialOrd + std::fmt::Display>(a: &[T]) {
    for i in 1..a.len() {
        for j in 0..i {
            assert!(a[j] <= a[i], "a[{}] = {} > a[{}] = {}", j, a[j], i, a[i]);
        }
    }
}

fn count_elements(values: &[u64]) -> HashMap<u64, usize> {
    let mut map = HashMap::new();

    for v in values {
        map.entry(*v).and_modify(|count| *count += 1).or_insert(1);
    }

    map
}
