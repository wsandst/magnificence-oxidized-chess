# magnificence-oxidized-chess
Chess engine written in Rust, built on the previous Magnificence revisions

## Build instructions
**Build native**:  
`RUSTFLAGS="-C target-cpu=native" cargo build --release` 



## Todo web
- [ ] Implement player/engine select dropdown
- [ ] Implement web worker for engine. It must run on a separate thread.
Web worker which runs engine -> use javascript callbacks for info, which sends a web worker post message to our main thread which handles ui. UI -> engine is done by posting a message, then calling a javasccript function which returns whatever we should do engine side (cancel, pause etc).
- [ ] Improve sidebar
- [ ] Implement undo move, board state switching and such. Pause.
- [ ] Engine settings perhaps?
- [ ] Implement display of valid moves. Mark previous moved squares. Only allow valid moves. 
- [ ] Show lost material under PlayerInfo.