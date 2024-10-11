cargo build --release && \
./target/release/xorshift.exe && \
./target/release/fastrand.exe && \
./target/release/smallrng.exe && \
./target/release/pcg.exe &&
cat generate_float.csv &&
cat generate_ints.csv &&
cat simulate_chances.csv