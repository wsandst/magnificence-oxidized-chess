
<script setup lang="ts">

import { ref, onMounted } from 'vue';

const emit = defineEmits(['pieceMoved']);

const boardElement : any = ref(null);
const refreshPieces : any = ref(0);
const movingPiece : any = ref(null);

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

const props = defineProps({
    pieces: {
        type: Array,
        required: true
    },
})

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
    return `transform: translate(${xPos}px, ${yPos}px);`
}

function calculateTranslationBasedMousePosition(x : number, y: number) {
    // It needs to be relative to the board element position
    var rect = boardElement.value.getBoundingClientRect();

    let xPos = x - rect.left - (0.125 / 2) * boardElement.value.clientWidth;
    let yPos = y - rect.top - (0.125 / 2) * boardElement.value.clientHeight;
    return `transform: translate(${xPos}px, ${yPos}px);`
}


function pieceDragStart(e: any) {
    movingPiece.value = e.target;
}

function pieceDragStop(e: any, x: number, y: number) {
    var rect = boardElement.value.getBoundingClientRect();
    let dragStopX = null;
    let dragStopY = null;
    let squareSizePx = 0.125 * boardElement.value.clientWidth;
    if (e.type == "touchend") {
        dragStopX = e.touches[0].pageX;
        dragStopY = e.touches[0].pageY;
    }
    else if (e.type == "mouseup") {
        dragStopX = e.x;
        dragStopY = e.y;
    }
    if (movingPiece.value != null) {
        let to_x = Math.floor((dragStopX - rect.left) / squareSizePx);
        let to_y = Math.floor((dragStopY - rect.top) / squareSizePx);
        if (to_x >= 0 && to_x < 8 && to_y >= 0 && to_y < 8 && (to_x != x || to_y != y)) {
            emit("pieceMoved", {from: [x, y], to: [to_x, to_y]});
            refreshPieces.value += 1;

        }
        else {
            movingPiece.value.style = calculateTranslationBasedOnPosition(x, y);
        }
    }
    movingPiece.value = null;
}

function boardMouseMove(e: MouseEvent) {
    if (movingPiece.value) {
        movingPiece.value.style = calculateTranslationBasedMousePosition(e.x, e.y);
    }
}

function boardTouchMove(e: TouchEvent) {
    if (movingPiece.value) {
        movingPiece.value.style = calculateTranslationBasedMousePosition(e.touches[0].pageX, e.touches[0].pageY);
    }
}

function boardResized() {
    refreshPieces.value += 1;
}

onMounted(() => {
    new ResizeObserver(boardResized).observe(boardElement.value);
})

</script>

<template>
    <div @mousemove="boardMouseMove" @touchmove="boardTouchMove" class="flex flex-col w-full aspect-square rounded-[12px] relative" ref="boardElement">
        <div class="flex flex-row w-full h-[12.5%]" v-for="row in 8" :key="row">
            <div class="w-[12.5%] h-full" 
                :class="{ 'bg-dark-square': boardPositionModulo(row, col) == 1, 'bg-light-square': boardPositionModulo(row, col) == 0}"
                v-for="col in 8" :key="row * 8 + col">
            </div>
        </div>
        <div class="absolute w-full" :key="refreshPieces" v-if="refreshPieces != 0">
            <img 
                v-for="{x, y, piece} in pieces" :key="y * 8 + x + 'b'"
                class="w-[12.5%] object-cover absolute select-none cursor-pointer" 
                :src="pieceToIconMap[piece]"
                :style="calculateTranslationBasedOnPosition(x, y)"
                draggable="false"
                @mousedown="pieceDragStart"
                @touchstart="pieceDragStart"
                @mouseup="(e) => pieceDragStop(e, x, y)"
                @touchend="(e) => pieceDragStop(e, x, y)"
            />
        </div>
    </div>
</template>


<style>

</style>