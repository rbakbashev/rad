use crate::rand::Wyhash64RNG;

pub fn assert_sorted<T: PartialOrd + std::fmt::Display>(a: &[T]) {
    for i in 1..a.len() {
        for j in 0..i {
            assert!(a[j] <= a[i], "a[{}] = {} > a[{}] = {}", j, a[j], i, a[i]);
        }
    }
}

pub fn generate_array_identical<T: Clone>(n: usize, x: T) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    v.resize(n, x);
    v
}

pub fn generate_array_ascending<T: From<u64>>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((i as u64).into());
    }
    v
}

pub fn generate_array_descending<T: From<u64>>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(((n - i) as u64).into());
    }
    v
}

pub fn generate_array_random(n: usize, lower: u64, upper: u64) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    let mut r = Wyhash64RNG::new();

    for _ in 0..n {
        v.push(r.gen_in_range(lower..upper));
    }

    v
}

pub fn generate_array_permuation<T: From<u64> + Copy>(n: usize) -> Vec<T> {
    let mut v = generate_array_ascending(n);
    permute(&mut v);
    v
}

pub fn permute<T: Copy>(v: &mut [T]) {
    let mut r = Wyhash64RNG::new();

    for i in 0..v.len() {
        let j = r.gen_in_range(0..v.len() as u64) as usize;

        v.swap(i, j);
    }
}

pub fn test_sort(f: fn(&mut [u64])) {
    let len = 100;
    let mut id = generate_array_identical(len, 1);
    let mut asc = generate_array_ascending(len);
    let mut desc = generate_array_descending(len);
    let mut rand = generate_array_random(len, 1, len as u64);
    let mut perm = generate_array_permuation(len);

    f(&mut id);
    assert_sorted(&id);

    f(&mut asc);
    assert_sorted(&asc);

    f(&mut desc);
    assert_sorted(&desc);

    f(&mut rand);
    assert_sorted(&rand);

    f(&mut perm);
    assert_sorted(&perm);
}
