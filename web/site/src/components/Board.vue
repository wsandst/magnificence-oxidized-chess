
<script setup lang="ts">

import { ref, onMounted, computed } from 'vue';
import { useChessEngineStore } from '../store/engine';

const boardElement : any = ref(null);
const movingPiece : any = ref(null);

const chessEngine = useChessEngineStore();

const startingPosition = [
    {"x": 0, "y": 0, "piece": 9},
    {"x": 1, "y": 0, "piece": 8},
    {"x": 2, "y": 0, "piece": 7},
    {"x": 3, "y": 0, "piece": 10},
    {"x": 4, "y": 0, "piece": 11},
    {"x": 5, "y": 0, "piece": 7},
    {"x": 6, "y": 0, "piece": 8},
    {"x": 7, "y": 0, "piece": 9},
    {"x": 0, "y": 1, "piece": 6},
    {"x": 1, "y": 1, "piece": 6},
    {"x": 2, "y": 1, "piece": 6},
    {"x": 3, "y": 1, "piece": 6},
    {"x": 4, "y": 1, "piece": 6},
    {"x": 5, "y": 1, "piece": 6},
    {"x": 6, "y": 1, "piece": 6},
    {"x": 7, "y": 1, "piece": 6},
    {"x": 0, "y": 6, "piece": 0},
    {"x": 1, "y": 6, "piece": 0},
    {"x": 2, "y": 6, "piece": 0},
    {"x": 3, "y": 6, "piece": 0},
    {"x": 4, "y": 6, "piece": 0},
    {"x": 5, "y": 6, "piece": 0},
    {"x": 6, "y": 6, "piece": 0},
    {"x": 7, "y": 6, "piece": 0},
    {"x": 0, "y": 7, "piece": 3},
    {"x": 1, "y": 7, "piece": 2},
    {"x": 2, "y": 7, "piece": 1},
    {"x": 3, "y": 7, "piece": 4},
    {"x": 4, "y": 7, "piece": 5},
    {"x": 5, "y": 7, "piece": 1},
    {"x": 6, "y": 7, "piece": 2},
    {"x": 7, "y": 7, "piece": 3}
]

const boardPieces = computed(() => {
    return chessEngine.currentBoardPieces ?? startingPosition;
})

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

const pieceToIconMap : any = {
    0: "src/assets/icons/pieces/white-pawn.svg",
    1: "src/assets/icons/pieces/white-bishop.svg",
    2: "src/assets/icons/pieces/white-knight.svg",
    3: "src/assets/icons/pieces/white-rook.svg",
    4: "src/assets/icons/pieces/white-queen.svg",
    5: "src/assets/icons/pieces/white-king.svg",
    6: "src/assets/icons/pieces/black-pawn.svg",
    7: "src/assets/icons/pieces/black-bishop.svg",
    8: "src/assets/icons/pieces/black-knight.svg",
    9: "src/assets/icons/pieces/black-rook.svg",
    10: "src/assets/icons/pieces/black-queen.svg",
    11: "src/assets/icons/pieces/black-king.svg"
}

function calculateTranslationBasedOnPosition(x : number, y: number) {
    let xPos = 0.125 * x * boardElement.value.clientWidth;
    let yPos = 0.125 * y * boardElement.value.clientHeight; 
    return `translate(${xPos}px, ${yPos}px)`
}

function calculateTranslationBasedMousePosition(x : number, y: number) {
    // It needs to be relative to the board element position
    var rect = boardElement.value.getBoundingClientRect();
    let xPos = x - rect.left - (0.125 / 2) * boardElement.value.clientWidth;
    let yPos = y - rect.top - (0.125 / 2) * boardElement.value.clientHeight;
    return `translate(${xPos}px, ${yPos}px)`
}

function pieceDragStart(e: any) {
    movingPiece.value = e.target;
    movingPiece.value.style.transform = calculateTranslationBasedMousePosition(e.x, e.y);
}

function animatePieceToPosition(piece: any, to_x: number, to_y: number, from_x: any, from_y: any) {
    piece.style.transition = `all 300ms ease`;
    piece.style.transform = calculateTranslationBasedOnPosition(to_x, to_y);
    console.log("Animate transform: ", piece.style.transform);
    setTimeout(() => {
        piece.style.transition = "";
        if (from_x != null) {
            chessEngine.makeMove([from_x, from_y], [to_x, to_y]);
        }
    }, 300);
}

function pieceDragStop(e: any, x: number, y: number) {
    var rect = boardElement.value.getBoundingClientRect();
    let dragStopX = null;
    let dragStopY = null;
    let squareSizePx = 0.125 * boardElement.value.clientWidth;
    if (e.type == "touchend") {
        dragStopX = e.changedTouches[0].pageX;
        dragStopY = e.changedTouches[0].pageY;
    }
    else if (e.type == "mouseup") {
        dragStopX = e.x;
        dragStopY = e.y;
    }
    if (movingPiece.value != null) {
        let to_x = Math.floor((dragStopX - rect.left) / squareSizePx);
        let to_y = Math.floor((dragStopY - rect.top) / squareSizePx);
        if (to_x >= 0 && to_x < 8 && to_y >= 0 && to_y < 8 && (to_x != x || to_y != y)) {
            chessEngine.makeMove([x, y], [to_x, to_y]);
            movingPiece.value.style.transform = calculateTranslationBasedOnPosition(to_x, to_y);
        }
        else {
            movingPiece.value.style.transform = calculateTranslationBasedOnPosition(x, y);
        }
    }
    movingPiece.value = null;

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

onMounted(() => {
    new ResizeObserver(boardResized).observe(boardElement.value);
})

</script>

<template>
    <div @mousemove="boardMouseMove" @touchmove.passive="boardTouchMove" class="flex select-none flex-col w-full aspect-square relative" ref="boardElement">
        <div class="flex flex-row w-full h-[12.5%]" v-for="row in 8" :key="row">
            <div class="w-[12.5%] h-full"
                :class="{ 'bg-dark-square': boardPositionModulo(row, col) == 1, 'bg-light-square': boardPositionModulo(row, col) == 0}"
                v-for="col in 8" :key="row * 8 + col">
            </div>
        </div>
        <div class="absolute w-full" :key="chessEngine.boardStateCounter" v-if="chessEngine.boardStateCounter != 0">
            <img 
                v-for="{x, y, piece} in boardPieces" :key="y * 8 + x + 'b'"
                class="w-[12.5%] object-cover absolute select-none cursor-pointer" 
                :class="{'cursor-grabbing': movingPiece != null}"
                :src="pieceToIconMap[piece]"
                :style="'transform:'+calculateTranslationBasedOnPosition(x, y)"
                draggable="false"
                @mousedown="pieceDragStart"
                @touchstart.passive="pieceDragStart"
                @mouseup="(e) => pieceDragStop(e, x, y)"
                @touchend="(e) => pieceDragStop(e, x, y)"
            />
        </div>
    </div>
</template>


<style>

</style>