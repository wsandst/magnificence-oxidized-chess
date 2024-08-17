
cd "$(dirname "$0")"
if [ $# -eq 0 ]; then
    echo "Please specify perft depth."
    echo "Usage: ./BENCHMARK <PERFT_DEPTH>"
    exit
fi
# Build
RUSTFLAGS="-C target-cpu=native" 
cargo build --release --package magnificence-oxidized --target=x86_64-unknown-linux-gnu
# Benchmark
hyperfine --show-output --warmup 3 --runs 15 "./target/x86_64-unknown-linux-gnu/release/magnificence-oxidized -c perft $1"
