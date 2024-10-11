mod bench;
mod chance_simulator;
use bench::{benchmark_simulator, test_simulator};
use chance_simulator::ChanceSimulator;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

pub struct XorShiftSimulator;

impl ChanceSimulator for XorShiftSimulator {
    const NAME: &'static str = "rand_xorshift";

    fn simulate_chances(rolls: u32, chance: u16) -> u32 {
        let mut success_count = 0;
        let mut rng = XorShiftRng::from_seed([0; 16]);
        for _ in 0..rolls {
            if (rng.next_u32() % (chance as u32)) == 0 {
                success_count += 1;
            }
        }
        success_count
    }

    fn generate_x_ints(amount: u32, range: [u32; 2]) -> Vec<u32> {
        let mut rng = XorShiftRng::from_seed([0; 16]);
        (0..amount)
            .map(|_| range[0] + (rng.next_u32() % (range[1] - range[0])))
            .collect()
    }

    fn generate_x_floats(amount: u32) -> Vec<f64> {
        let mut rng = XorShiftRng::from_seed([0; 16]);
        (0..amount)
            .map(|_| rng.next_u32() as f64 / std::u32::MAX as f64)
            .collect()
    }
}

fn main() {
    test_simulator::<XorShiftSimulator>();
    benchmark_simulator::<XorShiftSimulator>();
}
