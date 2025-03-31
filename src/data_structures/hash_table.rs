pub struct HashMapDirectAddressing<V> {
    data: Vec<Option<V>>,
}

pub struct HashMapChaining<V> {
    lists: Vec<Vec<V>>,
}

pub struct HashMapChainingSingleList<V> {
    data: Vec<usize>,
    list: Vec<(V, usize)>,
    recycled: Vec<usize>,
}

pub struct HashMapLinearProbing<V: Clone> {
    data: Vec<Option<(u32, V)>>,
}

impl<V> HashMapDirectAddressing<V> {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(None);
        }

        Self { data }
    }

    pub fn insert<K: Into<usize>>(&mut self, key: K, value: V) {
        self.data[key.into()] = Some(value);
    }

    pub fn delete<K: Into<usize>>(&mut self, key: K) {
        self.data[key.into()] = None;
    }

    pub fn search<K: Into<usize>>(&self, key: K) -> Option<&V> {
        self.data[key.into()].as_ref()
    }
}

impl<V> HashMapChaining<V> {
    const SLOTS_BASE: u32 = 14;
    const _BASE_ASSERT: () = assert!(Self::SLOTS_BASE < 32);
    const SLOTS: usize = 2_usize.pow(Self::SLOTS_BASE);

    fn hash(key: u32) -> usize {
        hash_mult_shift(key, Self::SLOTS_BASE) as usize
    }

    pub fn new() -> Self {
        let mut lists = Vec::with_capacity(Self::SLOTS);

        for _ in 0..Self::SLOTS {
            lists.push(Vec::new());
        }

        Self { lists }
    }

    pub fn insert<K: Into<u32>>(&mut self, key: K, value: V) {
        let hash = Self::hash(key.into());
        self.lists[hash].push(value);
    }

    pub fn delete<K: Into<u32>>(&mut self, key: K) {
        let hash = Self::hash(key.into());
        self.lists[hash].pop();
    }

    pub fn search<K: Into<u32>>(&self, key: K) -> Option<&V> {
        let hash = Self::hash(key.into());
        self.lists[hash].last()
    }
}

impl<V> HashMapChainingSingleList<V> {
    const NIL: usize = usize::MAX;
    const SLOTS_BASE: u32 = 14;
    const _BASE_ASSERT: () = assert!(Self::SLOTS_BASE < 32);
    const SLOTS: usize = 2_usize.pow(Self::SLOTS_BASE);

    fn hash(key: u32) -> usize {
        hash_mult_shift(key, Self::SLOTS_BASE) as usize
    }

    pub fn new() -> Self {
        Self {
            data: vec![Self::NIL; Self::SLOTS],
            list: vec![],
            recycled: vec![],
        }
    }

    pub fn insert<K: Into<u32>>(&mut self, key: K, value: V) {
        let hash = Self::hash(key.into());
        let curr = self.data[hash];
        let node = self.allocate_node(value, curr);

        self.data[hash] = node;
    }

    fn allocate_node(&mut self, value: V, next: usize) -> usize {
        let cons = (value, next);

        if let Some(idx) = self.recycled.pop() {
            self.list[idx] = cons;
            return idx;
        }

        self.list.push(cons);

        self.list.len() - 1
    }

    pub fn delete<K: Into<u32>>(&mut self, key: K) {
        let hash = Self::hash(key.into());
        let curr = self.data[hash];

        if curr == Self::NIL {
            return;
        }

        let next = self.list[curr].1;

        self.data[hash] = next;
        self.recycled.push(curr);
    }

    pub fn search<K: Into<u32>>(&self, key: K) -> Option<&V> {
        let hash = Self::hash(key.into());
        let curr = self.data[hash];

        if curr == Self::NIL {
            return None;
        }

        let node = self.data[hash];

        Some(&self.list[node].0)
    }
}

impl<V: Clone> HashMapLinearProbing<V> {
    const SLOTS_BASE: u32 = 14;
    const _BASE_ASSERT: () = assert!(Self::SLOTS_BASE < 32);
    const SLOTS: usize = 2_usize.pow(Self::SLOTS_BASE);

    fn hash(key: u32) -> u32 {
        hash_mult_shift(key, Self::SLOTS_BASE)
    }

    fn hash_linear_probe(key: u32, i: u32) -> usize {
        (Self::hash(key) + i) as usize % Self::SLOTS
    }

    #[allow(clippy::cast_possible_truncation)]
    fn hash_inverse(key: u32, idx: usize) -> u32 {
        let i = (idx - Self::hash(key) as usize) % Self::SLOTS;
        i as u32
    }

    pub fn new() -> Self {
        let mut data = Vec::with_capacity(Self::SLOTS);

        for _ in 0..Self::SLOTS {
            data.push(None);
        }

        Self { data }
    }

    pub fn insert<K: Into<u32>>(&mut self, key: K, value: V) -> Option<usize> {
        let key = key.into();
        let mut i = 0;

        loop {
            let idx = Self::hash_linear_probe(key, i);

            if self.data[idx].is_none() {
                self.data[idx] = Some((key, value));
                return Some(idx);
            }

            i += 1;

            if i as usize == Self::SLOTS {
                return None;
            }
        }
    }

    pub fn search<K: Into<u32>>(&self, key: K) -> Option<&V> {
        let (_idx, (_key, value)) = self.search_linear(key)?;

        Some(value)
    }

    fn search_linear<K: Into<u32>>(&self, key: K) -> Option<(usize, &(u32, V))> {
        let key = key.into();
        let mut i = 0;

        loop {
            let idx = Self::hash_linear_probe(key, i);
            let slot = self.data[idx].as_ref();

            if let Some(pair @ (slot_key, _value)) = slot {
                if *slot_key == key {
                    return Some((idx, pair));
                }
            }

            i += 1;

            if slot.is_none() || i as usize == Self::SLOTS {
                return None;
            }
        }
    }

    pub fn delete<K: Into<u32>>(&mut self, key: K) {
        let Some((idx, ..)) = self.search_linear(key) else {
            return;
        };

        self.delete_linear(idx);
    }

    fn delete_linear(&mut self, mut idx: usize) {
        let mut next;
        let mut slot;

        loop {
            self.data[idx] = None;

            next = idx;

            loop {
                next = (next + 1) % Self::SLOTS;
                slot = &self.data[next];

                match slot {
                    None => return,
                    Some((k, _v)) => {
                        if Self::hash_inverse(*k, idx) < Self::hash_inverse(*k, next) {
                            break;
                        }
                    }
                }
            }

            self.data[idx] = slot.clone();
            idx = next;
        }
    }
}

#[allow(clippy::unreadable_literal)]
fn hash_mult_shift(key: u32, slots_base: u32) -> u32 {
    key.wrapping_mul(2654435769) >> (32 - slots_base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rand::Wyhash64RNG;
    use std::collections::{HashMap as HashMapStd, HashSet};

    const SIZE: usize = 123;
    const SEED: u64 = 321;
    const SPAN: i64 = 10_000;
    const ADDS: usize = 100;
    const DELS: usize = 50;

    const _SIZE_ASSERTS: () = {
        assert!(SIZE < u8::MAX as usize);
        assert!(SPAN < i32::MAX as i64);
    };

    // This wrapper trait exists to 1) keep implementations clean, 2) create a generic testing func
    trait HashMapOps<K, V> {
        fn new(optional_size: usize) -> Self;
        fn insert(&mut self, key: K, value: V);
        fn delete(&mut self, key: K);
        fn search(&mut self, key: K) -> Option<&V>;
    }

    impl<K: Into<usize>, V> HashMapOps<K, V> for HashMapDirectAddressing<V> {
        fn new(optional_size: usize) -> Self {
            Self::new(optional_size)
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<&V> {
            Self::search(self, key)
        }
    }

    impl<K: Into<u32>, V> HashMapOps<K, V> for HashMapChaining<V> {
        fn new(_optional_size: usize) -> Self {
            Self::new()
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<&V> {
            Self::search(self, key)
        }
    }

    impl<K: Into<u32>, V> HashMapOps<K, V> for HashMapChainingSingleList<V> {
        fn new(_optional_size: usize) -> Self {
            Self::new()
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<&V> {
            Self::search(self, key)
        }
    }

    impl<K: Into<u32>, V: Clone> HashMapOps<K, V> for HashMapLinearProbing<V> {
        fn new(_optional_size: usize) -> Self {
            Self::new()
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<&V> {
            Self::search(self, key)
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn test_hash_map<HashMapTested: HashMapOps<u8, i32>>() {
        let mut std = HashMapStd::new();
        let mut map = HashMapTested::new(SIZE);
        let mut rng = Wyhash64RNG::from_seed(SEED);
        let mut ins = HashSet::new();
        let mut del = Vec::new();

        for _ in 0..ADDS / 2 {
            let (key, val) = generate_unique_key(&mut rng, &mut ins);

            std.insert(key, val);
            map.insert(key, val);
        }

        for _ in 0..DELS {
            let key = rng.gen_in_range(0..SIZE as u64) as u8;

            std.remove(&key);
            map.delete(key);

            del.push(key);
        }

        for _ in 0..ADDS / 2 {
            let (key, val) = generate_unique_key(&mut rng, &mut ins);

            std.insert(key, val);
            map.insert(key, val);
        }

        let mut key_values = std.iter().collect::<Vec<_>>();
        key_values.sort();

        for (key, val) in key_values {
            assert_eq!(Some(val), map.search(*key));
        }

        for del_key in del {
            if !std.contains_key(&del_key) {
                assert_eq!(None, map.search(del_key));
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn generate_unique_key(rng: &mut Wyhash64RNG, inserted: &mut HashSet<u8>) -> (u8, i32) {
        loop {
            let key = rng.gen_in_range(0..SIZE as u64) as u8;
            let val = rng.gen_in_range_i64(-SPAN..SPAN) as i32;

            if inserted.contains(&key) {
                continue;
            }

            inserted.insert(key);

            return (key, val);
        }
    }

    #[test]
    fn direct_addressing() {
        test_hash_map::<HashMapDirectAddressing<i32>>();
    }

    #[test]
    fn chaining() {
        test_hash_map::<HashMapChaining<i32>>();
    }

    #[test]
    fn single_list() {
        test_hash_map::<HashMapChainingSingleList<i32>>();
    }

    #[test]
    fn open_addressing() {
        test_hash_map::<HashMapLinearProbing<i32>>();
    }
}
