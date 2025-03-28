pub struct HashMapDirectAddressing<V: Clone> {
    data: Vec<Option<V>>,
}

pub struct HashMapChaining<V: Clone> {
    lists: Vec<LinkedList<V>>,
}

struct LinkedList<V> {
    data: Vec<(V, usize)>,
}

pub struct HashMapChainingSingleList<V: Clone> {
    data: Vec<usize>,
    list: Vec<(V, usize)>,
    recycled: Vec<usize>,
}

impl<V: Clone> HashMapDirectAddressing<V> {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![None; size],
        }
    }

    pub fn insert<K: Into<usize>>(&mut self, key: K, value: V) {
        self.data[key.into()] = Some(value);
    }

    pub fn delete<K: Into<usize>>(&mut self, key: K) {
        self.data[key.into()] = None;
    }

    pub fn search<K: Into<usize>>(&self, key: K) -> Option<V> {
        self.data[key.into()].clone()
    }
}

impl<V: Clone> HashMapChaining<V> {
    const SLOTS_BASE: u32 = 14;
    const _BASE_ASSERT: () = assert!(Self::SLOTS_BASE < 32);
    const SLOTS: usize = 2_usize.pow(Self::SLOTS_BASE);

    #[allow(clippy::unreadable_literal)]
    fn hash_mult_shift(key: u32) -> usize {
        let hash = key.wrapping_mul(2654435769) >> (32 - Self::SLOTS_BASE);
        hash as usize
    }

    pub fn new() -> Self {
        let mut lists = Vec::with_capacity(Self::SLOTS);

        for _ in 0..Self::SLOTS {
            lists.push(LinkedList::empty());
        }

        Self { lists }
    }

    pub fn insert<K: Into<u32>>(&mut self, key: K, value: V) {
        let hash = Self::hash_mult_shift(key.into());
        self.lists[hash].insert(value);
    }

    pub fn delete<K: Into<u32>>(&mut self, key: K) {
        let hash = Self::hash_mult_shift(key.into());
        self.lists[hash].pop();
    }

    pub fn search<K: Into<u32>>(&self, key: K) -> Option<V> {
        let hash = Self::hash_mult_shift(key.into());
        self.lists[hash].last()
    }
}

impl<V: Clone> LinkedList<V> {
    const NIL: usize = usize::MAX;

    fn empty() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, value: V) {
        let cons = (value, Self::NIL);

        self.data.push(cons);

        if self.data.len() == 1 {
            return;
        }

        let inserted = self.data.len() - 1;
        let prevlast = self.data.len() - 2;

        self.data[prevlast].1 = inserted;
    }

    fn pop(&mut self) {
        if let [.., (_val, next), _last] = self.data.as_mut_slice() {
            *next = Self::NIL;
        }

        self.data.pop();
    }

    fn last(&self) -> Option<V> {
        match self.data.as_slice() {
            [] => None,
            [.., (val, _next)] => Some(val.clone()),
        }
    }
}

impl<V: Clone> HashMapChainingSingleList<V> {
    const NIL: usize = usize::MAX;
    const SLOTS_BASE: u32 = 14;
    const _BASE_ASSERT: () = assert!(Self::SLOTS_BASE < 32);
    const SLOTS: usize = 2_usize.pow(Self::SLOTS_BASE);

    #[allow(clippy::unreadable_literal)]
    fn hash_mult_shift(key: u32) -> usize {
        let hash = key.wrapping_mul(2654435769) >> (32 - Self::SLOTS_BASE);
        hash as usize
    }

    pub fn new() -> Self {
        Self {
            data: vec![Self::NIL; Self::SLOTS],
            list: vec![],
            recycled: vec![],
        }
    }

    pub fn insert<K: Into<u32>>(&mut self, key: K, value: V) {
        let hash = Self::hash_mult_shift(key.into());
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
        let hash = Self::hash_mult_shift(key.into());
        let curr = self.data[hash];

        if curr == Self::NIL {
            return;
        }

        let next = self.list[curr].1;

        self.data[hash] = next;
        self.recycled.push(curr);
    }

    pub fn search<K: Into<u32>>(&mut self, key: K) -> Option<V> {
        let hash = Self::hash_mult_shift(key.into());
        let curr = self.data[hash];

        if curr == Self::NIL {
            return None;
        }

        let node = self.data[hash];

        Some(self.list[node].0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rand::Wyhash64RNG;
    use std::collections::HashMap as HashMapStd;

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
    trait HashMapOps<K, V: Clone> {
        fn new(optional_size: usize) -> Self;
        fn insert(&mut self, key: K, value: V);
        fn delete(&mut self, key: K);
        fn search(&mut self, key: K) -> Option<V>;
    }

    impl<K, V: Clone> HashMapOps<K, V> for HashMapDirectAddressing<V>
    where
        K: Into<usize>,
    {
        fn new(optional_size: usize) -> Self {
            Self::new(optional_size)
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<V> {
            Self::search(self, key)
        }
    }

    impl<K, V: Clone> HashMapOps<K, V> for HashMapChaining<V>
    where
        K: Into<u32>,
    {
        fn new(_optional_size: usize) -> Self {
            Self::new()
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<V> {
            Self::search(self, key)
        }
    }

    impl<K, V: Clone> HashMapOps<K, V> for HashMapChainingSingleList<V>
    where
        K: Into<u32>,
    {
        fn new(_optional_size: usize) -> Self {
            Self::new()
        }

        fn insert(&mut self, key: K, value: V) {
            self.insert(key, value);
        }

        fn delete(&mut self, key: K) {
            self.delete(key);
        }

        fn search(&mut self, key: K) -> Option<V> {
            Self::search(self, key)
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn test_hash_map<HashMapTested: HashMapOps<u8, i32>>() {
        let mut nat = HashMapStd::new();
        let mut map = HashMapTested::new(SIZE);
        let mut rng = Wyhash64RNG::from_seed(SEED);
        let mut del = Vec::new();

        for _ in 0..ADDS / 2 {
            let key = rng.gen_in_range(0..SIZE as u64) as u8;
            let val = rng.gen_in_range_i64(-SPAN..SPAN) as i32;

            nat.insert(key, val);
            map.insert(key, val);
        }

        for _ in 0..DELS {
            let key = rng.gen_in_range(0..SIZE as u64) as u8;

            nat.remove(&key);
            map.delete(key);

            del.push(key);
        }

        for _ in 0..ADDS / 2 {
            let key = rng.gen_in_range(0..SIZE as u64) as u8;
            let val = rng.gen_in_range_i64(-SPAN..SPAN) as i32;

            nat.insert(key, val);
            map.insert(key, val);
        }

        for (key, val) in nat {
            assert_eq!(Some(val), map.search(key));
        }

        for del_key in del {
            if map.search(del_key).is_none() {
                assert_eq!(None, map.search(del_key));
            }
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
}
