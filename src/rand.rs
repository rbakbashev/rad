use std::time;

pub struct Wyhash64RNG {
    state: u64,
}

impl Wyhash64RNG {
    pub fn new() -> Wyhash64RNG {
        Self::new_fixed(time_seed())
    }

    pub fn new_fixed(seed: u64) -> Wyhash64RNG {
        Wyhash64RNG { state: seed }
    }

    pub fn gen(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x60be_e2be_e120_fc15);
        let t = u128::from(self.state).wrapping_mul(0xa3b1_9535_4a39_b70d);
        let m = (((t >> 64) ^ t) & 0xffff_ffff_ffff_ffff) as u64;
        let y = u128::from(m).wrapping_mul(0x1b03_7387_12fa_d5c9);

        (((y >> 64) ^ y) & 0xffff_ffff_ffff_ffff) as u64
    }

    pub fn gen_in_range(&mut self, min: u64, max: u64) -> u64 {
        min + Self::gen(self) % (max - min + 1)
    }

    pub fn gen_in_size(&mut self, max: usize) -> usize {
        Self::gen_in_range(self, 0, (max - 1) as u64) as usize
    }

    pub fn gen_percent(&mut self) -> u64 {
        Self::gen_in_range(self, 0, 100)
    }
}

fn time_seed() -> u64 {
    let now = time::SystemTime::now();
    let full = now.duration_since(time::UNIX_EPOCH).unwrap();

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
            let mut rng = Wyhash64RNG::new_fixed(seed);
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
            let mut rng = Wyhash64RNG::new_fixed(seed);
            let mut sum = 0;
            let min = 500;
            let max = 1500;
            let err = 1.;

            for _ in 1..ITERATIONS {
                sum += rng.gen_in_range(min, max)
            }

            let avg = (sum as f64) / (ITERATIONS as f64);
            let mexp = ((max as f64) + (min as f64)) / 2.;

            assert!((avg - mexp).abs() < err);
        }
    }
}
