
<script setup lang="ts">

import { ref, onMounted, computed, watch } from 'vue';
import { useChessEngineStore } from '../store/engine';

const boardElement : any = ref(null);
const movingPiece : any = ref(null);
const selectedPiecePos: any = ref(null);
const previousMoveFromPos: any = ref(null);
const previousMoveToPos: any = ref(null);
const allPieces : any = ref([]);
const legalMovesHighlightedSquares: any = ref([]);

const chessEngine = useChessEngineStore();
chessEngine.makeBoardEngineMoveCallback = makeEngineMove;
chessEngine.clearBoardSelectionsCallback = clearSelections;

var moveSoundEffect = new Audio('./sounds/move.mp3');



const boardPieces = computed(() => {
    return chessEngine.currentBoardPieces;
})

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

const pieceToIconMap : any = {
    0: "./icons/pieces/white-pawn.svg",
    1: "./icons/pieces/white-bishop.svg",
    2: "./icons/pieces/white-knight.svg",
    3: "./icons/pieces/white-rook.svg",
    4: "./icons/pieces/white-queen.svg",
    5: "./icons/pieces/white-king.svg",
    6: "./icons/pieces/black-pawn.svg",
    7: "./icons/pieces/black-bishop.svg",
    8: "./icons/pieces/black-knight.svg",
    9: "./icons/pieces/black-rook.svg",
    10: "./icons/pieces/black-queen.svg",
    11: "./icons/pieces/black-king.svg"
}

function getMousePosAsBoardPos(mouseX: number, mouseY: number) {
    let squareSizePx = 0.125 * boardElement.value?.clientWidth;
    var rect = boardElement.value?.getBoundingClientRect();
    let x = Math.floor((mouseX - rect.left) / squareSizePx);
    let y = Math.floor((mouseY - rect.top) / squareSizePx);
    return [x, y];
}

function calculateTranslationBasedOnPosition(x : number, y: number) {
    let xPos = 0.125 * x * boardElement.value?.clientWidth;
    let yPos = 0.125 * y * boardElement.value?.clientHeight; 
    return `translate(${xPos}px, ${yPos}px)`
}

function calculateTranslationBasedMousePosition(x : number, y: number) {
    // It needs to be relative to the board element position
    var rect = boardElement.value.getBoundingClientRect();
    let xPos = x - rect.left - (0.125 / 2) * boardElement.value.clientWidth;
    let yPos = y - rect.top - (0.125 / 2) * boardElement.value.clientHeight;
    return `translate(${xPos}px, ${yPos}px)`
}

function pieceDragStart(e: any, x: number, y: number) {
    let piece = chessEngine.getPiece(x, y);
    legalMovesHighlightedSquares.value = piece.legal_moves.map(move => [move.to_x, move.to_y])
    movingPiece.value = e.target;
    movingPiece.value.style.transform = calculateTranslationBasedMousePosition(e.x, e.y);
    movingPiece.value.style.zIndex = 100;
    selectedPiecePos.value = [x, y];
}

function animatePieceToPosition(piece: any, to_x: number, to_y: number, from_x: any, from_y: any) {
    if (piece) {
        piece.style.transition = `all 300ms ease`;
        piece.style.zIndex = `100`;
        piece.style.transform = calculateTranslationBasedOnPosition(to_x, to_y);
    }
}

function pieceDragStop(e: any, x: number, y: number) {
    let dragStopX = null;
    let dragStopY = null;
    if (e.type == "touchend") {
        dragStopX = e.changedTouches[0].pageX;
        dragStopY = e.changedTouches[0].pageY;
    }
    else if (e.type == "mouseup") {
        dragStopX = e.x;
        dragStopY = e.y;
    }
    if (movingPiece.value != null && selectedPiecePos.value != null) {
        movingPiece.value.style.zIndex = 1;
        [x, y] = selectedPiecePos.value;
        let [to_x, to_y] = getMousePosAsBoardPos(dragStopX, dragStopY);
        if (to_x >= 0 && to_x < 8 && to_y >= 0 && to_y < 8 && (to_x != x || to_y != y)) {
            legalMovesHighlightedSquares.value = null;
            makeHumanMove(x, y, to_x, to_y);
        }
        else {
            movingPiece.value.style.transform = calculateTranslationBasedOnPosition(x, y);
        }
    }
    movingPiece.value = null;
}

function makeHumanMove(from_x: number, from_y: number, to_x: number, to_y: number, promotion : number|null = null) {
    // Validate the legality of the move
    if (!chessEngine.isMoveLegal([from_x, from_y], [to_x, to_y], promotion)) {
        movingPiece.value.style.transform = calculateTranslationBasedOnPosition(from_x, from_y);
        return;
    }
    makeMove(from_x, from_y, to_x, to_y, promotion);
    selectedPiecePos.value = null;
    movingPiece.value.style.transform = calculateTranslationBasedOnPosition(to_x, to_y);
}

function makeMove(from_x: number, from_y: number, to_x: number, to_y: number, promotion : number|null = null) {
    // Validate that it is this players turn and that the player is human
    chessEngine.makeMove([from_x, from_y], [to_x, to_y], promotion);
    moveSoundEffect.play();
    previousMoveFromPos.value = [from_x, from_y];
    previousMoveToPos.value = [to_x, to_y];
}

// Callbacks from Chess Engine store

function makeEngineMove(from_x: number, from_y: number, to_x: number, to_y: number, promotion: number) {
    let piece = allPieces.value.find((piece: any) => piece.getAttribute("data-x") == from_x && piece.getAttribute("data-y") == from_y);
    //selectedPiecePos.value = [from_x, from_y];
    animatePieceToPosition(piece, to_x, to_y, from_x, from_y);
    setTimeout(() => {
        if (piece) {
            piece.style.transition = "";
            piece.style.zIndex = 100;
        }
        if (from_x != null) {
            makeMove(from_x, from_y, to_x, to_y, promotion);
        }
    }, 300);
}

function clearSelections() {
    selectedPiecePos.value = null;
    previousMoveFromPos.value = null;
    previousMoveToPos.value = null;
    legalMovesHighlightedSquares.value = null;
}

function boardMouseMove(e: MouseEvent) {
    if (movingPiece.value) {
        movingPiece.value.style.transform = calculateTranslationBasedMousePosition(e.x, e.y);
    }
}

function boardTouchMove(e: TouchEvent) {
    if (movingPiece.value) {
        movingPiece.value.style.transform = calculateTranslationBasedMousePosition(e.touches[0].pageX, e.touches[0].pageY);
    }
}

function boardResized() {
    chessEngine.boardStateCounter += 1;
}

function shouldSquareBeHighlighted(x: number, y: number): boolean {
    let positions = [selectedPiecePos.value, previousMoveFromPos.value, previousMoveToPos.value];
    for (let position of positions) {
        if (position != null && position[0] == x && position[1] == y) {
            return true;
        }
    }
    return false;
}

function shouldSquareBeHighlightedAsLegalMove(x: number, y: number): boolean {
    if (legalMovesHighlightedSquares.value == null) {
        return false;
    }
    for (let position of legalMovesHighlightedSquares.value) {
        if (position != null && position[0] == x && position[1] == y) {
            return true;
        }
    }
    return false;
}

function getSquareColor(col: number, row: number) : string {
    if (boardPositionModulo(row, col) == 1) {
        // Dark square
        if (shouldSquareBeHighlightedAsLegalMove(col, row)) {
            return 'bg-dark-square-legal-highlight';
        }
        else if (shouldSquareBeHighlighted(col, row)) {
            return 'bg-dark-square-highlight';
        }
        return 'bg-dark-square';
    }
    else {
        // Light square
        if (shouldSquareBeHighlightedAsLegalMove(col, row)) {
            return 'bg-light-square-legal-highlight';
        }
        else if (shouldSquareBeHighlighted(col, row)) {
            return 'bg-light-square-highlight';
        }
        return 'bg-light-square';
    }
}

function posToAlgebraicPos(rows: number, cols: number) {
    return String.fromCharCode(cols - 1 + 'a'.charCodeAt(0)) + (9 - rows).toString();
}

onMounted(() => {
    new ResizeObserver(boardResized).observe(boardElement.value);
    chessEngine.currentBoardPieces = chessEngine.convertFenToBoardPieces(localStorage.getItem("current_board_fen"));
})

</script>

<template>
    <div @mousemove="boardMouseMove" @touchmove.prevent="boardTouchMove" class="flex select-none flex-col w-full aspect-square relative" ref="boardElement">
        <div class="flex flex-row w-full h-[12.5%]" v-for="row in 8" :key="row">
            <div class="w-[12.5%] h-full text-sm"
                :class="[getSquareColor(col - 1, row - 1)]"
                v-for="col in 8" :key="row * 8 + col">
                {{ (row - 1) * 8 + col - 1 }} {{ posToAlgebraicPos(row, col) }}
            </div>
        </div>
        <div class="absolute w-full" v-if="chessEngine.boardStateCounter != 0">
            <img 
                v-for="{x, y, piece} in boardPieces" :key="y * 8 + x + 'p' + piece"
                ref="allPieces"
                :data-x="x"
                :data-y="y"
                class="w-[12.5%] object-cover absolute select-none cursor-pointer" 
                :class="{'cursor-grabbing': movingPiece != null}"
                :src="pieceToIconMap[piece]"
                :style="'transform:'+calculateTranslationBasedOnPosition(x, y)"
                draggable="false"
                @mousedown="(e) => pieceDragStart(e, x, y)"
                @touchstart.prevent="(e) => pieceDragStart(e, x, y)"
                @mouseup="(e) => pieceDragStop(e, x, y)"
                @touchend="(e) => pieceDragStop(e, x, y)"
            />
        </div>
    </div>
</template>


<style>

</style>