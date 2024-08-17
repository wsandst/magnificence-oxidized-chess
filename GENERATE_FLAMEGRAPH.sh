
cd "$(dirname "$0")"
if [ $# -eq 0 ]; then
    echo "Please specify perft depth."
    echo "Usage: ./GENERATE_FLAMEGRAPH.sh <PERFT_DEPTH>"
    exit
fi
 
RUSTFLAGS="-C target-cpu=native" cargo build --release --package magnificence-oxidized --target=x86_64-unknown-linux-gnu
flamegraph -o flamegraph.svg -- ./target/x86_64-unknown-linux-gnu/release/magnificence-oxidized -c perft $1