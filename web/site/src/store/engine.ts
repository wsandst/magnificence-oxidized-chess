
import { defineStore } from 'pinia';
import { ChessEngine } from '../../wasm/magnificence_oxidized_web';

// Load web worker
let worker = new Worker(new URL('../worker.js', import.meta.url), {
  type: 'module', name: "chess_worker"
});

function getPiece(currentBoardPieces: any, x: number, y: number) {
  return currentBoardPieces.find((piece: any) => piece.x == x && piece.y == y) ?? 12;
}

export const useChessEngineStore = defineStore('chess_engine', {
  state: () => ({
    gamePaused: false,
    whitePlayer: null,
    blackPlayer: null,
    availablePlayers: [],

    currentPlayerColor: "white",
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
      this.setBlackPlayer(this.availablePlayers[1]);
      this.setWhitePlayer(human);
    },
    setWhitePlayer(player: any) {
      this.whitePlayer = player;
      worker.postMessage(["set_white_player", player.name]);
    },
    setBlackPlayer(player: any) {
      this.blackPlayer = player;
      worker.postMessage(["set_black_player", player.name]);
    },
    progressTurn() {
      this.currentPlayerColor = this.currentPlayerColor == "white" ? "black" : "white";
    },
    getCurrentPlayer() : any {
      return this.currentPlayerColor == "white" ? this.whitePlayer : this.blackPlayer;
    },
    initWasmWorker() {
      worker.onmessage = function (e) {
          //console.log('Message received from worker: ', e.data);
          const messageType = e.data[0];
          const data = e.data[1];
          if (e.data == "initiated") {
            worker.postMessage(["get_pieces"]);
            worker.postMessage(["get_allowed_engines"]);
            worker.postMessage(["get_board_fen"]);
          }
          else if (messageType == "get_pieces") {
            this.currentBoardPieces = data;
            this.boardStateCounter += 1;
          }
          else if (messageType == "get_allowed_engines") {
            this.setAvailableEngines(data);
          }
          else if (messageType == "get_board_fen") {
            this.currentBoardFenString = data;
          }
          else if (messageType == "search") {
            console.log("Search complete: ", data);
            const move = data[0];
            this.makeMove([move.from_x, move.from_y], [move.to_x, move.to_y], move.promotion)
          }
          else if (messageType == "search_metadata_update") {
            console.log("Search metadata update: ", data);
          }
      }.bind(this);
    },
    makeMove(from: [number, number], to: [number, number], promotion = null) {
      if (promotion == null) {
        // Always promote pawns to queen for now
        promotion = 12;
        if (to[1] == 7 && getPiece(this.currentBoardPieces, from[0], from[1]).piece == 6) {
          promotion = 10;
        }
        else if (to[1] == 0 && getPiece(this.currentBoardPieces, from[0], from[1]).piece  == 0) {
          promotion = 4;
        }
      }

      worker.postMessage(["make_move", from[0], from[1], to[0], to[1], promotion]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
      this.progressTurn();
      if (this.getCurrentPlayer().type == "engine") {
        worker.postMessage(["search"]);

      }
    }
  }
});