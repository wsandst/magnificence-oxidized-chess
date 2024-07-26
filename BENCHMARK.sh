
cd "$(dirname "$0")"
if [ $# -eq 0 ]; then
    echo "Please specify perft depth."
    echo "Usage: ./BENCHMARK <PERFT_DEPTH>"
    exit
fi
# Build
RUSTFLAGS="-C target-cpu=native" cargo build --release
# Benchmark
hyperfine --show-output --runs 10 "./target/release/magnificence-oxidized -c perft $1"
