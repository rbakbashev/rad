use std::ops::Range;
use std::time;

pub struct Wyhash64RNG {
    state: u64,
}

pub struct Wyhash32RNG {
    seed: u64,
}

impl Wyhash64RNG {
    pub fn new() -> Self {
        Self::from_seed(current_time_ns())
    }

    pub fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn gen(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x60be_e2be_e120_fc15);

        let t = u128::from(self.state).wrapping_mul(0xa3b1_9535_4a39_b70d);
        let m = (((t >> 64) ^ t) & 0xffff_ffff_ffff_ffff) as u64;
        let y = u128::from(m).wrapping_mul(0x1b03_7387_12fa_d5c9);

        (((y >> 64) ^ y) & 0xffff_ffff_ffff_ffff) as u64
    }

    pub fn gen_in_range(&mut self, range: Range<u64>) -> u64 {
        let min = range.start;
        let max = range.end;

        min + self.gen() % (max - min)
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn gen_in_range_i64(&mut self, range: Range<i64>) -> i64 {
        let min = range.start;
        let max = range.end;
        let gen = self.gen() as i64;

        min + gen.rem_euclid(max - min)
    }
}

impl Wyhash32RNG {
    pub const fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    pub const fn gen(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0xa076_1d64_78bd_642f);

        let mut see1 = self.seed ^ 0xe703_7ed1_a0b4_28db;

        see1 = see1.wrapping_mul(rot(see1));

        self.seed.wrapping_mul(rot(self.seed)) ^ rot(see1)
    }

    pub const fn gen_in_range(&mut self, range: Range<u64>) -> u64 {
        let min = range.start;
        let max = range.end;

        min + self.gen() % (max - min)
    }
}

fn current_time_ns() -> u64 {
    let now = time::SystemTime::now();
    let full = now
        .duration_since(time::UNIX_EPOCH)
        .expect("Current time before Unix epoch");

    u64::from(full.subsec_nanos())
}

const fn rot(x: u64) -> u64 {
    x.rotate_left(32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITER: u64 = 10;
    const SUM_ITER: u64 = if cfg!(miri) { 10_000 } else { 1_000_000 };
    const ERR_EPSILON: f64 = 1.;

    trait Generator {
        fn new() -> Self;
        fn from_seed(seed: u64) -> Self;
    }

    impl Generator for Wyhash64RNG {
        fn new() -> Self {
            Self::new()
        }

        fn from_seed(seed: u64) -> Self {
            Self::from_seed(seed)
        }
    }

    impl Generator for Wyhash32RNG {
        fn new() -> Self {
            Self::from_seed(1)
        }

        fn from_seed(seed: u64) -> Self {
            Self::from_seed(seed)
        }
    }

    fn test<R: Generator>(mut f: impl FnMut(&mut R) -> u64, mexp: f64, fixed_seed: bool) {
        for seed in 0..TEST_ITER {
            let mut sum = 0;
            let mut rng = if fixed_seed {
                R::from_seed(seed)
            } else {
                R::new()
            };

            for _ in 0..SUM_ITER {
                sum += f(&mut rng);
            }

            let max_int_f64 = 2_u64.pow(f64::MANTISSA_DIGITS) - 1;
            assert!(sum < max_int_f64);
            assert!(SUM_ITER < max_int_f64);

            #[allow(clippy::cast_precision_loss)]
            let avg = (sum as f64) / (SUM_ITER as f64);

            assert!((avg - mexp).abs() < ERR_EPSILON);
        }
    }

    #[test]
    fn simple() {
        test::<Wyhash64RNG>(|r| r.gen() % 101, 100. / 2., true);
    }

    #[test]
    fn range() {
        test::<Wyhash64RNG>(|r| r.gen_in_range(50..151), f64::midpoint(50., 150.), true);
    }

    #[test]
    fn range_i64() {
        for seed in 0..TEST_ITER {
            let mut sum = 0;
            let mut rng = Wyhash64RNG::from_seed(seed);

            for _ in 0..SUM_ITER {
                let val = rng.gen_in_range_i64(-100..101);
                sum += val;
            }

            #[allow(clippy::cast_precision_loss)]
            let avg = (sum as f64) / (SUM_ITER as f64);

            assert!(avg.abs() < ERR_EPSILON);
        }
    }

    #[test]
    #[cfg(not(miri))]
    fn from_time() {
        test::<Wyhash64RNG>(|r| r.gen() % 101, 100. / 2., false);
    }

    #[test]
    fn simple32() {
        test::<Wyhash32RNG>(|r| r.gen() % 101, 100. / 2., true);
    }

    #[test]
    fn range32() {
        test::<Wyhash32RNG>(|r| r.gen_in_range(50..151), f64::midpoint(50., 150.), true);
    }
}
