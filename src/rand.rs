use std::ops::Range;
use std::time;

pub struct Wyhash64RNG {
    state: u64,
}

impl Wyhash64RNG {
    #[allow(clippy::new_without_default)]
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
}

fn current_time_ns() -> u64 {
    let now = time::SystemTime::now();
    let full = now
        .duration_since(time::UNIX_EPOCH)
        .expect("Current time before Unix epoch");

    u64::from(full.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITER: u64 = 10;
    const SUM_ITER: u64 = 1_000_000;
    const ERR_EPSILON: f64 = 1.;

    type GenCb = fn(&mut Wyhash64RNG) -> u64;

    fn test(f: GenCb, mexp: f64, fixed_seed: bool) {
        for seed in 0..TEST_ITER {
            let mut sum = 0;
            let mut rng = if fixed_seed {
                Wyhash64RNG::from_seed(seed)
            } else {
                Wyhash64RNG::new()
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
        test(|r| 1 + r.gen() % 100, 100. / 2., false);
    }

    #[test]
    fn range() {
        test(|r| r.gen_in_range(50..150), (50. + 150.) / 2., false);
    }

    #[test]
    fn from_time() {
        test(|r| 1 + r.gen() % 100, 100. / 2., true);
    }
}
