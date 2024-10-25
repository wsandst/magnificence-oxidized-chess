RUSTFLAGS="-C target-cpu=native" cargo build --release --package magnificence-oxidized --target=x86_64-unknown-linux-gnu
REPO=/home/williamsandst/repos/rust/magnificence-oxidized-chess 
#ENGINE1=$REPO/target/x86_64-unknown-linux-gnu/release/magnificence-oxidized
ENGINE1=$1
ENGINE2=$2
OPENING_BOOK="-openings file=$REPO/benchmark/books/noob_4moves.pgn format=pgn start=1"
cutechess-cli -rounds 3 -games 2 -engine cmd=$ENGINE1 -engine cmd=$ENGINE2 -each restart=on proto=uci tc=inf/40+0.4 -pgnout result.pgn -recover $OPENING_BOOK