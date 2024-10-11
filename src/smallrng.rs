mod bench;
mod chance_simulator;

use bench::{benchmark_simulator, test_simulator};
use chance_simulator::ChanceSimulator;
use rand::{distr::Uniform, rngs::SmallRng, Rng};

pub struct SmallRngSimulator;

impl ChanceSimulator for SmallRngSimulator {
    const NAME: &'static str = "smallrng";

    fn simulate_chances(rolls: u32, chance: u16) -> u32 {
        let mut success_count = 0;
        let mut rng = SmallRng::from_thread_rng();
        for _ in 0..rolls {
            if rng.gen_range(0..chance) == 0 {
                success_count += 1;
            }
        }
        success_count
    }

    fn generate_x_ints(amount: u32, range: [u32; 2]) -> Vec<u32> {
        let mut rng = SmallRng::from_thread_rng();
        let range = Range::new(1, 20);

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(range.ind_sampsle(&mut rng));
        }

        vec
    }

    fn generate_x_floats(amount: u32) -> Vec<f64> {
        let mut rng = SmallRng::from_thread_rng();
        (0..amount).map(|_| rng.random()).collect()
    }
}

fn main() {
    test_simulator::<SmallRngSimulator>();
    benchmark_simulator::<SmallRngSimulator>();
}
