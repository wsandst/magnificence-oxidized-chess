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

## Todo web
- [ ] Improve sidebar
- [ ] Implement time controls
- [ ] Implement game win condition
- [ ] Implement local storage of current board
- [ ] Implement undo move.
- [ ] Implement display of valid moves. Mark previous moved squares. Only allow valid moves. 
- [ ] Show lost material under PlayerInfo.