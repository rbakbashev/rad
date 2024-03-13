//! van Emde Boas tree

// universe size, `u`, is a field because `feature(generic_const_exprs)` is unstable
#[derive(Clone)]
pub struct VebTree {
    u: usize,
    u_lsqrt: usize,
    min: Option<usize>,
    max: Option<usize>,
    summary: Option<Box<VebTree>>,
    cluster: Vec<VebTree>,
}

impl VebTree {
    /// Construct a new van Emde Boas tree that can hold 2^`exp` items.
    pub fn new(exp: u32) -> Self {
        assert!(exp > 0);

        let u = 2_usize.pow(exp);
        let u_lsqrt = lsqrt(u);
        let min = None;
        let max = None;

        if u == 2 {
            return Self {
                u,
                u_lsqrt,
                min,
                max,
                summary: None,
                cluster: Vec::new(),
            };
        }

        let div_exp = exp.div_ceil(2);
        let summary = Some(Box::new(Self::new(div_exp)));
        let clu_len = usqrt(u);
        let cluster = vec![Self::new(div_exp); clu_len];

        Self {
            u,
            u_lsqrt,
            min,
            max,
            summary,
            cluster,
        }
    }

    pub fn insert(&mut self, mut key: usize) {
        if key >= self.u {
            return;
        }

        match self.min.as_mut() {
            None => {
                self.min = Some(key);
                self.max = Some(key);
            }
            Some(min) => {
                if key < *min {
                    std::mem::swap(&mut key, min);
                }

                if self.u > 2 {
                    let num = self.cluster_number(key);
                    let idx = self.idx_in_cluster(key);
                    let clu = &mut self.cluster[num];

                    if clu.min.is_none() {
                        let summary = self.summary.as_deref_mut().expect("summary should be set");

                        summary.insert(num);
                        clu.min = Some(idx);
                        clu.max = Some(idx);
                    } else {
                        clu.insert(idx);
                    }
                }

                let max = self.max.as_mut().expect("max should be set");

                if key > *max {
                    *max = key;
                }
            }
        }
    }

    pub fn min(&self) -> Option<&usize> {
        self.min.as_ref()
    }

    pub fn max(&self) -> Option<&usize> {
        self.max.as_ref()
    }

    pub fn has_key(&self, mut key: usize) -> bool {
        if key >= self.u {
            return false;
        }

        let mut subtree = self;

        loop {
            let min = subtree.min.as_ref();
            let max = subtree.max.as_ref();

            if min.is_some_and(|n| *n == key) || max.is_some_and(|x| *x == key) {
                return true;
            }

            if subtree.u == 2 {
                return false;
            }

            let num = subtree.cluster_number(key);

            key = subtree.idx_in_cluster(key);
            subtree = &subtree.cluster[num];
        }
    }

    // high(x)
    fn cluster_number(&self, x: usize) -> usize {
        x / self.u_lsqrt
    }

    // low(x)
    fn idx_in_cluster(&self, x: usize) -> usize {
        x % self.u_lsqrt
    }

    // index(x, y)
    fn idx_from_parts(&self, x: usize, y: usize) -> usize {
        x * self.u_lsqrt + y
    }

    pub fn successor(&self, key: usize) -> Option<usize> {
        if self.u == 2 {
            if key == 0 && self.max.is_some_and(|x| x == 1) {
                return Some(1);
            }

            return None;
        }

        if self.min.is_some_and(|n| key < n) {
            return self.min;
        }

        if self.max.is_some_and(|x| key > x) {
            return None;
        }

        let hi = self.cluster_number(key);
        let lo = self.idx_in_cluster(key);

        let key_clu = &self.cluster[hi];
        let max_low = key_clu.max;

        if max_low.is_some_and(|xl| lo < xl) {
            let offset = key_clu.successor(lo).expect("unexpected nil");
            let elem = self.idx_from_parts(hi, offset);
            return Some(elem);
        }

        let summary = self.summary.as_deref().expect("summary should be set");
        let succ_idx = summary.successor(hi)?;
        let succ_clu = &self.cluster[succ_idx];
        let offset = succ_clu.min.expect("unexpected nil");
        let elem = self.idx_from_parts(succ_idx, offset);

        Some(elem)
    }

    pub fn predecessor(&self, key: usize) -> Option<usize> {
        if self.u == 2 {
            if key == 1 && self.min.is_some_and(|n| n == 0) {
                return Some(0);
            }

            return None;
        }

        if self.max.is_some_and(|x| key > x) {
            return self.max;
        }

        if self.min.is_some_and(|n| key < n) {
            return None;
        }

        let hi = self.cluster_number(key);
        let lo = self.idx_in_cluster(key);

        let key_clu = &self.cluster[hi];
        let min_low = key_clu.min;

        if min_low.is_some_and(|nl| lo > nl) {
            let offset = key_clu.predecessor(lo).expect("unexpected nil");
            let elem = self.idx_from_parts(hi, offset);
            return Some(elem);
        }

        let summary = self.summary.as_deref().expect("summary should be set");
        let pred_idx = summary.predecessor(hi);

        match pred_idx {
            Some(pred_idx) => {
                let pred_clu = &self.cluster[pred_idx];
                let offset = pred_clu.max.expect("unexpected nil");
                let elem = self.idx_from_parts(pred_idx, offset);
                Some(elem)
            }
            None => {
                if self.min.is_some_and(|n| key > n) {
                    self.min
                } else {
                    None
                }
            }
        }
    }

    /// Delete `key` from the tree. Assumes that `key` is present in the collection.
    pub fn delete(&mut self, mut key: usize) {
        // Only one element is present. Based on the assumption, this must be `key`.
        if self.min.is_some() && self.min == self.max {
            self.min = None;
            self.max = None;
            return;
        }

        if self.u == 2 {
            if key == 0 {
                self.min = Some(1);
            } else {
                self.min = Some(0);
            }

            self.max = self.min;
            return;
        }

        if self.min.is_some_and(|n| n == key) {
            let summary = self.summary.as_deref().expect("summary should be set");
            let fst_idx = summary.min.expect("unexpected nil");
            let fst_clu = &self.cluster[fst_idx];
            let fst_min = fst_clu.min.expect("unexpected nil");

            key = self.idx_from_parts(fst_idx, fst_min);
            self.min = Some(key);
        }

        let num = self.cluster_number(key);
        let idx = self.idx_in_cluster(key);
        let clu = &mut self.cluster[num];

        clu.delete(idx);

        if clu.min.is_none() {
            let summary = self.summary.as_deref_mut().expect("summary should be set");

            summary.delete(num);

            if self.max.is_some_and(|x| x == key) {
                match summary.max {
                    Some(sum_max) => {
                        let sum_clu = &self.cluster[sum_max];
                        let sum_idx = sum_clu.max.expect("unexpected nil");
                        self.max = Some(self.idx_from_parts(sum_max, sum_idx));
                    }
                    None => self.max = self.min,
                }
            }

            return;
        }

        if self.max.is_some_and(|x| x == key) {
            let max = clu.max.expect("unexpected nil");
            self.max = Some(self.idx_from_parts(num, max));
        }
    }
}

const fn usqrt(x: usize) -> usize {
    let exp = x.ilog2().div_ceil(2);
    2_usize.pow(exp)
}

const fn lsqrt(x: usize) -> usize {
    let exp = x.ilog2() / 2;
    2_usize.pow(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_roots() {
        for exp in 1..16 {
            let po2 = 2_usize.pow(exp);

            if exp % 2 == 0 {
                assert_eq!(lsqrt(po2), usqrt(po2));
            } else {
                assert_eq!(lsqrt(po2) * usqrt(po2), po2);
            }
        }
    }

    #[test]
    fn simple() {
        let mut t = VebTree::new(5);

        assert_eq!(false, t.has_key(0));
        assert_eq!(false, t.has_key(3));
        assert_eq!(false, t.has_key(1));
        assert_eq!(false, t.has_key(40));

        t.insert(3);
        assert_eq!(true, t.has_key(3));
        assert_eq!(Some(&3), t.min());
        assert_eq!(Some(&3), t.max());

        t.insert(1);
        assert_eq!(true, t.has_key(3));
        assert_eq!(true, t.has_key(1));
        assert_eq!(Some(&1), t.min());
        assert_eq!(Some(&3), t.max());

        t.insert(16);
        t.insert(28);
        t.insert(31);
        assert_eq!(true, t.has_key(3));
        assert_eq!(true, t.has_key(1));
        assert_eq!(true, t.has_key(16));
        assert_eq!(true, t.has_key(28));
        assert_eq!(true, t.has_key(31));
        assert_eq!(Some(&1), t.min());
        assert_eq!(Some(&31), t.max());
    }

    #[test]
    fn succ_pred() {
        let mut t = VebTree::new(5);

        t.insert(5);
        t.insert(10);
        t.insert(15);

        assert_eq!(Some(5), t.successor(0));
        assert_eq!(Some(5), t.successor(1));
        assert_eq!(Some(10), t.successor(5));
        assert_eq!(Some(10), t.successor(6));
        assert_eq!(Some(15), t.successor(10));
        assert_eq!(None, t.successor(15));
        assert_eq!(None, t.successor(16));
        assert_eq!(None, t.successor(40));

        assert_eq!(None, t.predecessor(5));
        assert_eq!(None, t.predecessor(1));
        assert_eq!(None, t.predecessor(0));
        assert_eq!(Some(5), t.predecessor(10));
        assert_eq!(Some(5), t.predecessor(9));
        assert_eq!(Some(10), t.predecessor(15));
        assert_eq!(Some(10), t.predecessor(14));
        assert_eq!(Some(15), t.predecessor(16));
        assert_eq!(Some(15), t.predecessor(31));
        assert_eq!(Some(15), t.predecessor(40));
    }

    #[test]
    fn small() {
        let mut t = VebTree::new(1);

        t.insert(1);
        t.insert(0);

        assert_eq!(true, t.has_key(0));
        assert_eq!(true, t.has_key(1));
        assert_eq!(Some(&0), t.min());
        assert_eq!(Some(&1), t.max());
        assert_eq!(Some(1), t.successor(0));
        assert_eq!(Some(0), t.predecessor(1));
    }

    #[test]
    fn delete() {
        let mut t = VebTree::new(5);

        t.insert(5);
        t.insert(10);
        t.insert(15);
        t.insert(20);

        assert_eq!(true, t.has_key(5));
        assert_eq!(true, t.has_key(10));
        assert_eq!(true, t.has_key(15));
        assert_eq!(true, t.has_key(20));

        t.delete(15);
        assert_eq!(true, t.has_key(5));
        assert_eq!(true, t.has_key(10));
        assert_eq!(false, t.has_key(15));
        assert_eq!(true, t.has_key(20));

        t.delete(5);
        assert_eq!(false, t.has_key(5));
        assert_eq!(true, t.has_key(10));
        assert_eq!(false, t.has_key(15));
        assert_eq!(true, t.has_key(20));

        t.delete(10);
        assert_eq!(false, t.has_key(5));
        assert_eq!(false, t.has_key(10));
        assert_eq!(false, t.has_key(15));
        assert_eq!(true, t.has_key(20));

        t.delete(20);
        assert_eq!(false, t.has_key(5));
        assert_eq!(false, t.has_key(10));
        assert_eq!(false, t.has_key(15));
        assert_eq!(false, t.has_key(20));
    }

    #[test]
    fn full() {
        let exp = 10;
        let mut t = VebTree::new(exp);

        for x in 0..2_usize.pow(exp) {
            assert_eq!(false, t.has_key(x));
            t.insert(x);
            assert_eq!(true, t.has_key(x));
        }

        for x in 0..2_usize.pow(exp) {
            assert_eq!(true, t.has_key(x));
            t.delete(x);
            assert_eq!(false, t.has_key(x));
        }
    }

    #[test]
    fn drop() {
        let exp = if cfg!(miri) { 8 } else { 16 };
        let mut t = VebTree::new(exp);

        for x in 0..2_usize.pow(exp) {
            t.insert(x);
        }
    }
}
