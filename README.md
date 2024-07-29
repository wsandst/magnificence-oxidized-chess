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

### Web  
- [ ] Improve sidebar
- [ ] Implement game win condition (win popup)
- [ ] Implement display of valid moves. Mark previous moved squares. Only allow valid moves. 
- [ ] Implement command dropdown (perft, set fen)
- [ ] Implement new game dialog

### UCI
- [ ] Implement go/search properly
- [ ] Test in a UCI GUI interface

## Ideas
* Using Stockfish NNUE for eval
* Compile using PGO (https://en.wikipedia.org/wiki/Profile-guided_optimization, https://github.com/dede1751/carp?tab=readme-ov-file)


### Possible future web functionality
- [ ] Time controls
- [ ] New game dialog