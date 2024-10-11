pub trait ChanceSimulator {
    const NAME: &'static str;

    fn simulate_chances(rolls: u32, chance: u16) -> u32
    where
        Self: Sized;

    fn generate_x_ints(amount: u32, range: [u32; 2]) -> Vec<u32>
    where
        Self: Sized;

    fn generate_x_floats(amount: u32) -> Vec<f64>
    where
        Self: Sized;
}
