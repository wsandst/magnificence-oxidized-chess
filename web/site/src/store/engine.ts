
import { defineStore } from 'pinia';
import { ChessEngine } from '../../wasm/magnificence_oxidized_web';

// Load web worker
let worker = new Worker(new URL('../worker.js', import.meta.url), {
  type: 'module'
});

function getPiece(currentBoardPieces: any, x: number, y: number) {
  return currentBoardPieces.find((piece: any) => piece.x == x && piece.y == y) ?? 12;
}

export const useChessEngineStore = defineStore('chess_engine', {
  state: () => ({
    gamePaused: false,
    player1: null,
    player2: null,
    availablePlayers: [],

    currentBoardPieces: null,
    boardStateCounter: 0,
    currentBoardFenString: null
  }),
  actions: {
    setAvailableEngines(engines : any) {
      let human = {"name": "Human", "profile": "src/assets/images/human-profile.png", "type": "human"}
      this.availablePlayers.push(human);
      for (const engineName of engines) {
        this.availablePlayers.push({"name": engineName, "profile": "src/assets/images/robot-profile.png", "type": "engine"})
      }
      this.player1 = this.availablePlayers[1];
      this.player2 = human;
    },
    toggleSidebar() {
      this.sidebarVisible = !this.sidebarVisible;
    },
    initWasmWorker() {
      worker.onmessage = function (e) {
          //console.log('Message received from worker: ', e.data);
          if (e.data == "initiated") {
            worker.postMessage(["get_pieces"]);
            worker.postMessage(["get_allowed_engines"]);
            worker.postMessage(["get_board_fen"]);
          }
          else if (e.data[0] == "get_pieces") {
            this.currentBoardPieces = e.data[1];
            this.boardStateCounter += 1;
          }
          else if (e.data[0] == "get_allowed_engines") {
            this.setAvailableEngines(e.data[1]);
          }
          else if (e.data[0] == "get_board_fen") {
            this.currentBoardFenString  = e.data[1];
          }
      }.bind(this);
    },
    makeMove(from, to) {
      // Always promote pawns to queen for now
      let promotion = 12;
      if (to[1] == 7 && getPiece(this.currentBoardPieces, from[0], from[1]).piece == 6) {
        promotion = 10;
      }
      else if (to[1] == 0 && getPiece(this.currentBoardPieces, from[0], from[1]).piece  == 0) {
        promotion = 4;
      }
      worker.postMessage(["make_move", from[0], from[1], to[0], to[1], promotion]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
    }
  }
});