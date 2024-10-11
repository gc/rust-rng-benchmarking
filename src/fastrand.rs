mod bench;
mod chance_simulator;
use bench::{benchmark_simulator, test_simulator};
use chance_simulator::ChanceSimulator;

pub struct FastrandSimulator;

impl ChanceSimulator for FastrandSimulator {
    const NAME: &'static str = "fastrand";
    fn simulate_chances(rolls: u32, chance: u16) -> u32 {
        let mut success_count = 0;
        let mut rng = fastrand::Rng::new();
        for _ in 0..rolls {
            if rng.u16(0..chance) == 0 {
                success_count += 1;
            }
        }
        success_count
    }

    fn generate_x_ints(amount: u32, range: [u32; 2]) -> Vec<u32> {
        let mut rng = fastrand::Rng::new();
        (0..amount).map(|_| rng.u32(range[0]..range[1])).collect()
    }

    fn generate_x_floats(amount: u32) -> Vec<f64> {
        let mut rng = fastrand::Rng::new();
        (0..amount).map(|_| rng.f64()).collect()
    }
}

fn main() {
    test_simulator::<FastrandSimulator>();
    benchmark_simulator::<FastrandSimulator>();
}
