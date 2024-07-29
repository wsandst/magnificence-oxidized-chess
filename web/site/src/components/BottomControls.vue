<script setup lang="ts">
import { ref } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faPlay } from '@fortawesome/free-solid-svg-icons/faPlay'
import { faPause } from '@fortawesome/free-solid-svg-icons/faPause'
import { faRotateLeft } from '@fortawesome/free-solid-svg-icons/faRotateLeft'
import { faArrowsRotate } from '@fortawesome/free-solid-svg-icons/faArrowsRotate'
import { faWrench } from '@fortawesome/free-solid-svg-icons/faWrench'

import { useChessEngineStore } from '../store/engine';

const toolsDropdownVisible = ref(false);

const chessEngine = useChessEngineStore();

function toggletoolsDropdownVisibility() {
    toolsDropdownVisible.value = !toolsDropdownVisible.value;
    if (toolsDropdownVisible.value) {
        window.setTimeout(() => {
            window.addEventListener("click", function hideDropdown() {
                toolsDropdownVisible.value = false;
                window.removeEventListener('click', hideDropdown);
            });
        }, 300);
    }
}

const toolCommands = [
    {
        name: "Set FEN", 
        hasResponse: false, 
        args: [{name: "FEN String", type: "string"}], 
        command: (fen: string) => chessEngine.setBoardFen(fen)
    },
    {
        name: "Perft", 
        hasResponse: true, 
        args: [{name: "Depth", type: "number"}], 
        command: (depth: number) => chessEngine.perft(depth)
    }
]

</script>

<template>
    <div class="flex justify-end gap-3 relative">
        <FontAwesomeIcon @click="toggletoolsDropdownVisibility()" title="Tools" class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faWrench"/>
        <div v-if="toolsDropdownVisible" class="absolute flex flex-col left-0 top-[35px] bg-container/95 w-[130px] p-2 rounded-[8px]">
            <div @click="chessEngine.showCommandDialogCallback(command)" v-for="command in toolCommands" class="flex flex-row gap-2 items-center group cursor-pointer min-h-[30px]">
                <div class="group-hover:scale-110 group-hover:ml-1 text-sm transition-all duration-300 ease-in-out">{{command.name}}</div>
            </div>
        </div>
        
        <FontAwesomeIcon @click="chessEngine.resetGame()" title="Reset game to starting position" class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faArrowsRotate"/>
        <FontAwesomeIcon @click="chessEngine.undoMove()" title="Undo last move" class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faRotateLeft"/>
        <FontAwesomeIcon @click="chessEngine.resumeGame()" v-if="chessEngine.gamePaused" title="Resume engine calculation" class="cursor-pointer hover:scale-110 duration-300 ease-in-out w-[18px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faPlay"/>
        <FontAwesomeIcon @click="chessEngine.pauseGame()" v-else title="Pause engine calculation" class="cursor-pointer hover:scale-110 duration-300 ease-in-out w-[18px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faPause"/>
    </div>
</template>

<style scoped>

</style>