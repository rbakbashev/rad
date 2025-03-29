pub struct HashMapDirectAddressing<V: Clone> {
    data: Vec<Option<V>>,
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
}
