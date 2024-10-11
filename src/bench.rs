use crate::chance_simulator::ChanceSimulator;
use csv::{Writer, WriterBuilder};
use std::fs::{File, OpenOptions};
use std::time::Instant;

fn measure_time<F>(mut func: F) -> u128
where
    F: FnMut(),
{
    let mut times = Vec::new();
    for _ in 0..20 {
        let start = Instant::now();
        func();
        times.push(start.elapsed().as_millis());
    }

    return times.iter().sum::<u128>() / times.len() as u128;
}

pub fn make_writer(name: &str) -> Writer<File> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}.csv", name))
        .expect("Unable to open file");

    let writer = WriterBuilder::new().delimiter(b'\t').from_writer(file);
    return writer;
}

pub fn benchmark_simulator<S: ChanceSimulator>() {
    let chances_rolls = 10_000_000;
    let chances = [1, 20, 40];

    let mut chance_writer = make_writer("simulate_chances");
    for chance in &chances {
        let duration = measure_time(|| {
            S::simulate_chances(chances_rolls, *chance);
        });
        chance_writer
            .write_record(&[
                S::NAME,
                &chances_rolls.to_string(),
                &chance.to_string(),
                &duration.to_string(),
            ])
            .expect("Unable to write to CSV");
    }
    chance_writer.flush().expect("");

    let mut int_writer = make_writer("generate_ints");
    let gen_int_rolls = [5_000_000, 100_000_000];
    let ranges = [[1, 10_000_000]];
    for range in &ranges {
        for rolls in &gen_int_rolls {
            let duration = measure_time(|| {
                S::generate_x_ints(*rolls, *range);
            });

            int_writer
                .write_record(&[
                    S::NAME,
                    &rolls.to_string(),
                    &format!("[{}, {}]", range[0], range[1]),
                    &duration.to_string(),
                ])
                .expect("Unable to write to CSV");
        }
    }
    int_writer.flush().expect("");

    let mut float_writer = make_writer("generate_float");
    let amount_floats = 5_000_000;
    let duration = measure_time(|| {
        S::generate_x_floats(amount_floats);
    });

    float_writer
        .write_record(&[
            S::NAME,
            &amount_floats.to_string(),
            "-",
            &duration.to_string(),
        ])
        .expect("Unable to write to CSV");
    float_writer.flush().expect("");
}

pub fn test_simulator<S: ChanceSimulator>() {
    let qty = 10_000_000;
    let chance_simulator = 1000;

    let result = S::simulate_chances(qty, chance_simulator);
    let expected = qty / chance_simulator as u32;
    let actual = (expected as i32 - result as i32).abs() < expected as i32 / 2;
    assert!(actual, "Expected {} but got {}", expected, result);
    let ratio = (result as f64 / (qty as f64 / chance_simulator as f64)) as f64;
    println!(
        "{} had {result} success from {qty} rolls: {:.3}",
        S::NAME,
        ratio,
    );
}
