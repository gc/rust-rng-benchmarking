mod bench;
mod chance_simulator;
use crate::chance_simulator::ChanceSimulator;
use bench::{benchmark_simulator, test_simulator};
use rand_core::RngCore;
use rand_pcg::Pcg64Mcg;

pub struct Pcg64McgSimulator;

impl ChanceSimulator for Pcg64McgSimulator {
    const NAME: &'static str = "rand_pcg";

    fn simulate_chances(rolls: u32, chance: u16) -> u32 {
        let mut success_count = 0;
        let mut rng = Pcg64Mcg::new(0);
        for _ in 0..rolls {
            if rng.next_u32() % (chance as u32) == 0 {
                success_count += 1;
            }
        }
        success_count
    }

    fn generate_x_ints(amount: u32, range: [u32; 2]) -> Vec<u32> {
        let mut rng = Pcg64Mcg::new(0);
        (0..amount)
            .map(|_| range[0] + (rng.next_u32() % (range[1] - range[0])))
            .collect()
    }

    fn generate_x_floats(amount: u32) -> Vec<f64> {
        let mut rng = Pcg64Mcg::new(0);
        (0..amount)
            .map(|_| rng.next_u64() as f64 / std::u64::MAX as f64)
            .collect()
    }
}

fn main() {
    test_simulator::<Pcg64McgSimulator>();
    benchmark_simulator::<Pcg64McgSimulator>();
}
