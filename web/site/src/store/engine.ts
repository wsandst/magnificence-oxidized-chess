
import { defineStore } from 'pinia';
import { ChessEngine } from '../../wasm/magnificence_oxidized_web';

// Load web worker
let worker = new Worker(new URL('../worker.js', import.meta.url), {
  type: 'module', name: "chess_worker"
});

let lastCrashFenString = null;
let currentFenCrashCount = 0;

const charToPieceMap = {
  'P': 0,
  'B': 1,
  'N': 2,
  'R': 3,
  'Q': 4,
  'K': 5,
  'p': 6,
  'b': 7,
  'n': 8,
  'r': 9,
  'q': 10,
  'k': 11,
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
    currentBoardFenString: null,
    searchMetadata: null,
    logHistory: [],
    engineSearching: false,
    gameStatus: null,
    currentPly: 0,

    // Callbacks
    makeBoardEngineMoveCallback: null,
    clearBoardSelectionsCallback: null,
    showCommandDialogCallback: null,
    commandResponseCallback: null,
    showGameEndPopupCallback: null
  }),
  actions: {
    setAvailableEngines(engines : any) {
      let human = {"name": "Human", "profile": "./images/human-profile.png", "type": "human"}
      this.availablePlayers = [];
      this.availablePlayers.push(human);
      for (const engineName of engines) {
        this.availablePlayers.push({"name": engineName, "profile": "./images/robot-profile.png", "type": "engine"})
      }
      if (!this.whitePlayer) {
        this.setWhitePlayer(human);
      }
      else {
        worker.postMessage(["set_white_player", this.whitePlayer.name]);
      }
      if (!this.blackPlayer) {
        this.setBlackPlayer(this.availablePlayers[1]);
      }
      else {
        worker.postMessage(["set_black_player", this.blackPlayer.name]);
      }
    },
    setWhitePlayer(player: any) {
      this.whitePlayer = player;
      worker.postMessage(["set_white_player", player.name]);
      if (player.type == "engine" && this.currentPlayerColor == "white") {
        if (this.engineSearching) {
          worker.postMessage(["abort"]);
        }
        worker.postMessage(["search"]);
        this.engineSearching = true;
      }
      localStorage.setItem("white_player", JSON.stringify(this.whitePlayer));
    },
    setBlackPlayer(player: any) {
      this.blackPlayer = player;
      worker.postMessage(["set_black_player", player.name]);
      if (player.type == "engine" && this.currentPlayerColor == "black") {
        if (this.engineSearching) {
          worker.postMessage(["abort"]);
        }
        worker.postMessage(["search"]);
        this.engineSearching = true;
      }
      localStorage.setItem("black_player", JSON.stringify(this.blackPlayer));
    },
    progressTurn() {
      this.currentPlayerColor = this.currentPlayerColor == "white" ? "black" : "white";
    },
    getCurrentPlayer() : any {
      return this.currentPlayerColor == "white" ? this.whitePlayer : this.blackPlayer;
    },
    getPiece(x: number, y: number) {
      return this.currentBoardPieces.find((piece: any) => piece.x == x && piece.y == y) ?? 12;
    },
    initWasmWorker() {
      worker.onmessage = function (e) {
          //console.log('Message received from worker: ', e.data);
          const messageType = e.data[0];
          const data = e.data[1];
          const duration = e.data[2];
          if (data == "aborted") {
            this.engineSearching = false;
            return;
          }
          if (e.data == "initiated") {
            const savedBoardFen = localStorage.getItem("current_board_fen");
            if (savedBoardFen) {
              worker.postMessage(["set_board_fen", savedBoardFen]);
            } 
            worker.postMessage(["get_allowed_engines"]);
            this.syncBoardState();
          }
          else if (messageType == "get_pieces") {
            this.currentBoardPieces = data;
            this.boardStateCounter += 1;
          }
          else if (messageType == "log") {
            this.logHistory.push(data);
          }
          else if (messageType == "get_allowed_engines") {
            this.setAvailableEngines(data);
          }
          else if (messageType == "get_board_fen") {
            this.currentBoardFenString = data;
            localStorage.setItem("current_board_fen", data);
          }
          else if (messageType == "get_game_status") {
            this.setGameStatus(data);
          }
          else if (messageType == "search") {
            console.log("Search complete: ", data);
            this.engineSearching = false;
            this.searchOnUnpause = false;
            const move = data[0];
            if (move == undefined) {
              return;
            }
            this.makeBoardEngineMoveCallback(move.from_x, move.from_y, move.to_x, move.to_y, move.promotion)
          }
          else if (messageType == "search_metadata_update") {
            console.log("Search metadata update: ", data);
            this.searchMetadata = data;
          }
          else if (messageType == "perft") {
            const perft_count = data;
            const million_moves_per_second = (perft_count / 1000000) / (duration / 1000);
            this.commandResponseCallback([
              `Perft completed in ${(duration/1000).toFixed(2)} seconds (${million_moves_per_second.toFixed(2)}M moves per second)`,
              `Perft result: ${perft_count}`
            ]);
          }
          else if (messageType == "get_current_player_color") {
            this.currentPlayerColor = data;
            this.startSearchIfNecessary();
          }
          else if (messageType == "engine_crash") {
            console.log("Web Assembly seems to have crashed, reinitiating...");
            // Reset the board state if we are stuck in a never-ending crash loop
            if (this.currentBoardFenString == lastCrashFenString) {
                currentFenCrashCount += 1;
            }
            else {
                currentFenCrashCount = 1;
                lastCrashFenString = this.currentBoardFenString;
            }
            if (currentFenCrashCount >= 3) {
              console.log("Engine crashed 3 times in a row with this position, resetting to starting position...");
              this.currentBoardFenString = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
              this.clearBoardSelectionsCallback();
              localStorage.setItem("current_board_fen", this.currentBoardFenString);
            }
          }
      }.bind(this);
    },
    newGame() {
      this.currentBoardFenString = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
      this.clearBoardSelectionsCallback();
      localStorage.setItem("current_board_fen", this.currentBoardFenString);
      worker.postMessage(["set_board_fen", this.currentBoardFenString]);
      this.syncBoardState();
    },
    syncBoardState() {
      worker.postMessage(["get_game_status"]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
      worker.postMessage(["get_current_player_color"]);
    },
    makeMove(from: [number, number], to: [number, number], promotion : any|null = null) {
      const from_piece = this.getPiece(from[0], from[1]).piece;
      const to_piece = this.getPiece(to[0], to[1]).piece;
      if (promotion == null) {
        // Always promote pawns to queen for now
        promotion = 12;
        if (to[1] == 7 && from_piece.piece == 6) {
          promotion = 10;
        }
        else if (to[1] == 0 && from_piece.piece  == 0) {
          promotion = 4;
        }
      }

      let moveType = "regular";
      if (to_piece && to_piece != 12) {
        moveType = "capture";
      }
    

      worker.postMessage(["make_move", from[0], from[1], to[0], to[1], promotion]);
      worker.postMessage(["get_game_status"]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
      this.progressTurn();
      if (this.getCurrentPlayer().type == "engine") {
        this.engineSearching = true;
        worker.postMessage(["search"]);
      }
      this.currentPly += 1;
      return moveType;
    },
    
    undoMove() {
      if (this.currentPly <= 0) {
        return;
      }
      if (this.engineSearching) {
        worker.postMessage(["abort"]);
      }
      worker.postMessage(["undo_move"]);
      worker.postMessage(["get_pieces"]);
      this.progressTurn();
      this.startSearchIfNecessary();
      this.clearBoardSelectionsCallback();
      this.currentPly -= 1;
    },
    isMoveLegal(from: [number, number], to: [number, number], promotion : any|null = null, moving_piece = null) {
      // Make sure it is this players turn
      if (this.currentPlayerColor == "white" && this.whitePlayer.type != "human") {
        return false;
      }
      else if (this.currentPlayerColor == "black" && this.blackPlayer.type != "human") {
        return false;
      }
      // The current player can only move its own pieces
      const piece = this.getPiece(from[0], from[1]);
      if (this.currentPlayerColor == "white" && piece.piece >= 6) {
        return false;
      }
      else if (this.currentPlayerColor == "black" && piece.piece < 6) {
        return false;
      }

      // Check that the move is among the pieces legal moves
      return piece.legal_moves.find((move: any) => move.to_x == to[0] && move.to_y == to[1]);
    },  
    perft(depth: number) {
      console.log("Perft: ", depth);
      worker.postMessage(["perft", depth]);
    },
    setBoardFen(fen: string) {
      this.currentBoardFenString = fen;
      this.gameStatus = "running";
      this.clearBoardSelectionsCallback();
      localStorage.setItem("current_board_fen", this.currentBoardFenString);
      worker.postMessage(["set_board_fen", this.currentBoardFenString]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
      worker.postMessage(["get_current_player_color"]);
      this.currentPly = 0;
    },
    setPlayersFromLocalStorage() {
      const whitePlayer = localStorage.getItem("white_player");
      const blackPlayer = localStorage.getItem("black_player");
      console.log(whitePlayer, blackPlayer);
      if (whitePlayer) {
        this.whitePlayer = JSON.parse(whitePlayer);
      }
      if (blackPlayer) {
        this.blackPlayer = JSON.parse(blackPlayer);
      }
    },
    resetGame() {
      worker.postMessage(["abort"]);
      worker.postMessage(["reset_board"]);
      worker.postMessage(["get_pieces"]);
      worker.postMessage(["get_board_fen"]);
      this.currentPlayerColor = "white";
      this.clearBoardSelectionsCallback();
      this.startSearchIfNecessary();
      this.currentPly = 0;
    },
    pauseGame() {
      if (this.engineSearching) {
        console.log("Aborting!");
        worker.postMessage(["abort"]);
      }
      this.gamePaused = true;
    },
    resumeGame() {
      this.gamePaused = false;
      this.startSearchIfNecessary();
    },
    setGameStatus(status: "white_won" | "black_won" | "stalemate" | "running") {
      this.gameStatus = status;
      if (status != "running") {
        worker.postMessage(["abort"]);
        this.engineSearching = false;
        this.showGameEndPopupCallback(status);
      }
    },
    startSearchIfNecessary() {
      if (this.gameStatus == "running" &&
          (this.currentPlayerColor == "white" && this.whitePlayer.type == "engine" ||
            this.currentPlayerColor == "black" && this.blackPlayer.type == "engine")
      ) {
        this.engineSearching = true;
        worker.postMessage(["search"]);
      }
    },
    convertFenToBoardPieces(fen: string | null) {
      if (fen == null) {
        fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
      }
      let pieces = []
      console.log(fen);
      const fenPieces = fen.split(" ")[0];
      let rows = fenPieces.split("/");
      let y = 0;
      for (const row of rows) {
        let x = 0;
        for (const c of row) {
          if (c >= '0' && c <= '9') {
              // Digit means empty spaces
              x += parseInt(c);
          }
          else {
              // Map the character to the correct piece
              pieces.push({"x": x, "y": y, "piece": charToPieceMap[c], "legal_moves": []});
              x += 1;
          }
          if (x >= 8) {
              break;
          }
        }
        y = y + 1;
      }
      return pieces;
    }
  }
});