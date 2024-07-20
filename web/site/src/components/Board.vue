
<script setup lang="ts">

import { ref, onMounted } from 'vue';

const boardElement = ref(null);
const refreshPieces = ref(0);
const movingPiece = ref(null);

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

const props = defineProps({
    pieces: {
        type: Array,
        required: true
    },
})

const pieceToIconMap = {
    1: "src/assets/icons/pieces/white-pawn.svg",
    2: "src/assets/icons/pieces/white-bishop.svg",
    3: "src/assets/icons/pieces/white-knight.svg",
    4: "src/assets/icons/pieces/white-rook.svg",
    5: "src/assets/icons/pieces/white-queen.svg",
    6: "src/assets/icons/pieces/white-king.svg",
    7: "src/assets/icons/pieces/black-pawn.svg",
    8: "src/assets/icons/pieces/black-bishop.svg",
    9: "src/assets/icons/pieces/black-knight.svg",
    10: "src/assets/icons/pieces/black-rook.svg",
    11: "src/assets/icons/pieces/black-queen.svg",
    12: "src/assets/icons/pieces/black-king.svg"
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
    console.log(xPos, yPos);
    return `transform: translate(${xPos}px, ${yPos}px);`
}


function pieceDragStartMouse(e: MouseEvent, x: number, y: number) {
    console.log(e, x, y);
    movingPiece.value = e.target;
}

function pieceDragStopMouse(e: MouseEvent, x: number, y: number) {
    movingPiece.value = null;
}

function pieceDragStartTouch(e: TouchEvent, x: number, y: number) {
    console.log(e, x, y);
    movingPiece.value = e.target;
}

function pieceDragStopTouch(e: TouchEvent, x: number, y: number) {
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
                v-for="col in 8" :key="row * col">
            </div>
        </div>
        <div class="absolute w-full" :key="refreshPieces" v-if="refreshPieces != 0">
            <img 
                v-for="{x, y, piece} in pieces" :key="x * y + 'b'"
                class="w-[12.5%] object-cover absolute select-none cursor-pointer" 
                :src="pieceToIconMap[piece]"
                :style="calculateTranslationBasedOnPosition(x, y)"
                draggable="false"
                @mousedown="(e) => pieceDragStartMouse(e, x, y)"
                @touchstart="(e) => pieceDragStartTouch(e, x, y)"
                @mouseup="(e) => pieceDragStopMouse(e, x, y)"
                @touchend="(e) => pieceDragStartTouch(e, x, y)"
            />
        </div>
    </div>
</template>


<style>

</style>