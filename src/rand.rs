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

    const ITERATIONS: usize = 1000000;

    #[test]
    fn rand_test() {
        let attempts = 10;

        for seed in 1..attempts {
            let mut rng = Wyhash64RNG::from_seed(seed);
            let mut sum = 0;
            let max = 1000;
            let err = 1.;

            for _ in 1..ITERATIONS {
                sum += 1 + rng.gen() % max;
            }

            let avg = (sum as f64) / (ITERATIONS as f64);
            let mexp = (max as f64) / 2.;

            assert!((avg - mexp).abs() < err);
        }
    }

    #[test]
    fn rand_test_range() {
        let attempts = 10;

        for seed in 1..attempts {
            let mut rng = Wyhash64RNG::from_seed(seed);
            let mut sum = 0;
            let min = 500;
            let max = 1500;
            let err = 1.;

            for _ in 1..ITERATIONS {
                sum += rng.gen_in_range(min..max)
            }

            let avg = (sum as f64) / (ITERATIONS as f64);
            let mexp = ((max as f64) + (min as f64)) / 2.;

            assert!((avg - mexp).abs() < err);
        }
    }
}
