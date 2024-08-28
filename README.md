# magnificence-oxidized-chess
Magnificence Oxidized is a Chess Engine written in Rust, built on the previous Magnificence revisions. It uses state of the art bitboards. The engine has two interfaces - a command-line version implementing the Universal Chess Interface (UCI) protocol and a web interface. The web interface is made using Vue and interacts with the Chess Engine through a WebAssembly worker.

## Dependencies
* **Rust**   
* **NPM**  
* **wasm-pack**  
Install with: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`  
* **rsw wasm build tool**  
Install with: `cargo install rsw`  

## Native build instructions (UCI)
**Build**:  
`RUSTFLAGS="-C target-cpu=native" cargo build --release`  
**Run:**  
`./target/release/magnificence-chess` 

## Web build instructions
**First time setup:**  
`cd ./web/site && npm install`  
**Development:**  
`cd ./web/site && npm run dev`  
**Building a production version**:  
`cd ./web/site && ./BUILD.sh`. The generated site will be available under `./web/site/dist`.

## Todo
- [ ] Implement standardized testing with cutechess-cli tournaments
- [ ] Validate Go parameter parsing and pass to engine

### Web interface
- [ ] Improve win condition popup

## Ideas
* Using Stockfish NNUE for eval
* Compile using PGO (https://en.wikipedia.org/wiki/Profile-guided_optimization, https://github.com/dede1751/carp?tab=readme-ov-file)

### Possible future web functionality
- [ ] Time controls
- [ ] New game dialog

## Install CuteChess
`git clone https://github.com/cutechess/cutechess`
`cd cutechess`
`pip3 install aqtinstall`
`aqt install-qt linux desktop 5.15.2` 
Setup path to point to 5.15.2
`cd cutechess && mkdir build && cd build && cmake .. && make`

## Tournament commands
`cutechess-cli -engine cmd=/home/williamsandst/repos/rust/magnificence-oxidized-chess/target/release/magnificence-oxidized -engine cmd=/home/williamsandst/repos/rust/magnificence-oxidized-chess/target/release/magnificence-oxidized -each proto=uci tc=40/60 -debug` 
`cutechess-cli -engine cmd=/home/williamsandst/repos/rust/magnificence-oxidized-chess/target/release/magnificence-oxidized -engine cmd=/home/williamsandst/repos/cpp/magnificence-chess/build/magnificence-chess -each proto=uci tc=40/60+2 -debug -pgnout result.pgn`

`REPO=/home/williamsandst/repos/rust/magnificence-oxidized-chess cutechess-cli -engine cmd=$REPO/target/release/magnificence-oxidized -engine cmd=$REPO/benchmark/engines/stash-9.0-linux-x86_64-bmi2 -each proto=uci tc=40/60 -debug` 