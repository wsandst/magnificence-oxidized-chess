rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data -Ctarget-cpu=native" \
    cargo build --release --package magnificence-oxidized --target=x86_64-unknown-linux-gnu

./target/x86_64-unknown-linux-gnu/release/magnificence-oxidized -c perft 8
~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata \
 merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Ctarget-cpu=native" \
    cargo build --release --package magnificence-oxidized --target=x86_64-unknown-linux-gnu