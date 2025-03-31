#![allow(clippy::cast_possible_truncation)]

use crate::rand::Wyhash32RNG;

pub struct HashMapPerfectHashing<'v, V> {
    tables: ConstVec<SecondaryTable<'v, V>>,
    num_keys: u32,
    next_prime: u32,
    const_a: u32,
    const_b: u32,
}

struct SecondaryTable<'v, V> {
    const_a: u32,
    const_b: u32,
    slots: ConstVec<Option<&'v V>>,
}

struct ConstVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<'v, V> HashMapPerfectHashing<'v, V> {
    pub const fn from_u32s(key_values: &'v [(u32, V)]) -> Self {
        let mut seed = 1;

        loop {
            let inst = Self::try_from_u32s(key_values, seed);

            match inst {
                Some(s) => return s,
                None => seed += 1,
            }
        }
    }

    const fn try_from_u32s(key_values: &'v [(u32, V)], seed: u64) -> Option<Self> {
        if key_values.is_empty() {
            return None;
        }

        let num_keys = key_values.len() as u32;
        let max_key = get_max_key(key_values);
        let next_prime = get_next_prime(max_key);

        let mut rng = Wyhash32RNG::from_seed(seed);
        let const_a = rng.gen() as u32 % next_prime;
        let const_b = rng.gen() as u32 % next_prime;

        let mut i = 0;
        let mut tables = ConstVec::new(num_keys as usize);

        while i < num_keys {
            tables.push(SecondaryTable::new(&mut rng, num_keys, next_prime));
            i += 1;
        }

        let mut inst = Self {
            tables,
            num_keys,
            next_prime,
            const_a,
            const_b,
        };

        if inst.insert_all(key_values) {
            Some(inst)
        } else {
            None
        }
    }

    const fn insert_all(&mut self, key_values: &'v [(u32, V)]) -> bool {
        let mut i = 0;

        while i < key_values.len() {
            let (key, ref val) = key_values[i];

            if !self.insert_one(key, val) {
                return false;
            }

            i += 1;
        }

        true
    }

    const fn insert_one(&mut self, key: u32, val: &'v V) -> bool {
        let slot = self.get_mut(key);

        if slot.is_some() {
            return false;
        }

        *slot = Some(val);

        true
    }

    pub const fn get_mut(&mut self, key: u32) -> &mut Option<&'v V> {
        let (table_idx, secondary_idx) = self.get_indices(key);

        self.tables.as_mut(table_idx).slots.as_mut(secondary_idx)
    }

    const fn get_indices(&self, key: u32) -> (usize, usize) {
        let table_idx = universal_hash(
            self.const_a,
            self.const_b,
            self.next_prime,
            self.num_keys,
            key,
        );

        let table = self.tables.as_ref(table_idx as usize);

        let num_slots = table.slots.len as u32;

        let secondary_idx = universal_hash(
            table.const_a,
            table.const_b,
            self.next_prime,
            num_slots,
            key,
        );

        (table_idx as usize, secondary_idx as usize)
    }

    pub const fn get(&self, key: u32) -> &'v V {
        let (table_idx, secondary_idx) = self.get_indices(key);

        self.tables
            .as_ref(table_idx)
            .slots
            .as_ref(secondary_idx)
            .expect("key not found")
    }
}

impl<V> SecondaryTable<'_, V> {
    const fn new(rng: &mut Wyhash32RNG, num_keys: u32, next_prime: u32) -> Self {
        let max_keys = rng.gen() as u32 % (num_keys - 1) + 1;

        let (const_a, const_b) = if max_keys == 1 {
            (0, 0)
        } else {
            let a = rng.gen() as u32 % next_prime;
            let b = rng.gen() as u32 % next_prime;

            (a, b)
        };

        let num_slots = max_keys * max_keys;

        let slots = construct_vector_of_nones(num_slots as usize);

        Self {
            const_a,
            const_b,
            slots,
        }
    }
}

impl<T> ConstVec<T> {
    const fn new(cap: usize) -> Self {
        let size = size_of::<T>() * cap;
        let align = align_of::<T>();
        let ptr = unsafe { core::intrinsics::const_allocate(size, align) }.cast();
        let len = 0;

        Self { ptr, len, cap }
    }

    #[allow(unused)]
    fn new_non_const(cap: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(cap).expect("failed to construct layout");
        let ptr = unsafe { std::alloc::alloc(layout) }.cast();
        let len = 0;

        Self { ptr, len, cap }
    }

    const fn push(&mut self, item: T) {
        assert!(!self.ptr.is_null());
        assert!(self.len != self.cap);

        unsafe { self.ptr.add(self.len).write(item) }

        self.len += 1;
    }

    const fn as_mut(&mut self, idx: usize) -> &mut T {
        assert!(idx < self.len);

        unsafe { self.ptr.add(idx).as_mut() }.expect("elem is null")
    }

    const fn as_ref(&self, idx: usize) -> &T {
        assert!(idx < self.len);

        unsafe { self.ptr.add(idx).as_ref() }.expect("elem is null")
    }
}

const fn get_max_key<V>(key_values: &[(u32, V)]) -> u32 {
    let mut i = 0;
    let mut max = key_values[0].0;

    while i < key_values.len() {
        if key_values[i].0 > max {
            max = key_values[i].0;
        }

        i += 1;
    }

    max
}

const fn get_next_prime(mut x: u32) -> u32 {
    loop {
        x += 1;

        if is_prime(x) {
            break;
        }
    }

    x
}

const fn is_prime(x: u32) -> bool {
    let mut i = 2;

    while i * i <= x {
        if x % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

const fn construct_vector_of_nones<T>(len: usize) -> ConstVec<Option<T>> {
    let mut i = 0;
    let mut v = ConstVec::new(len);

    while i < len {
        v.push(None);
        i += 1;
    }

    v
}

const fn universal_hash(a: u32, b: u32, p: u32, m: u32, k: u32) -> u32 {
    ((a * k + b) % p) % m
}

#[test]
fn const_vec() {
    const {
        let mut vec = ConstVec::new(3);

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert!(1 == *vec.as_ref(0));
        assert!(2 == *vec.as_ref(1));
        assert!(3 == *vec.as_ref(2));
    }
}

#[test]
#[allow(long_running_const_eval)]
fn simple() {
    const {
        let key_values = [
            (10, 10 * 2),
            (22, 22 * 2),
            (37, 37 * 2),
            (40, 40 * 2),
            (52, 52 * 2),
            (60, 60 * 2),
            (70, 70 * 2),
            (72, 72 * 2),
            (75, 75 * 2),
        ];

        let hash_map = HashMapPerfectHashing::from_u32s(&key_values);

        assert!(*hash_map.get(10) == 10 * 2);
        assert!(*hash_map.get(22) == 22 * 2);
        assert!(*hash_map.get(37) == 37 * 2);
        assert!(*hash_map.get(40) == 40 * 2);
        assert!(*hash_map.get(52) == 52 * 2);
        assert!(*hash_map.get(60) == 60 * 2);
        assert!(*hash_map.get(70) == 70 * 2);
        assert!(*hash_map.get(72) == 72 * 2);
        assert!(*hash_map.get(75) == 75 * 2);
    }
}
