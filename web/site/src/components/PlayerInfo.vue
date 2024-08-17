<script setup lang="ts">

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faCaretDown } from '@fortawesome/free-solid-svg-icons/faCaretDown'
import { faSpinner } from '@fortawesome/free-solid-svg-icons/faSpinner'
import { ref, computed } from 'vue';
import { useChessEngineStore } from '../store/engine';

const chessEngine = useChessEngineStore();

const player1Placeholder = {"name": "Magnificence", "profile": "./images/robot-profile.png"}
const player2Placeholder = {"name": "Human", "profile": "./images/human-profile.png"}

const dropdownVisible = ref(false);

const props = defineProps({
    playerColor: {
        type: String,
        required: true
    },
})

function switchPlayer(player) {
    if (props.playerColor == "black") {
        chessEngine.setBlackPlayer(player);
    }
    else {
        chessEngine.setWhitePlayer(player);
    }
}

const playerInfo = computed(() => {
    const player = props.playerColor == "black" ? chessEngine.blackPlayer : chessEngine.whitePlayer;
    if (player == null) {
        return props.playerColor == "black" ? player1Placeholder : player2Placeholder;
    }
    return player;
});

function toggleDropdownVisibility() {
    dropdownVisible.value = !dropdownVisible.value;
    if (dropdownVisible.value) {
        window.setTimeout(() => {
            window.addEventListener("click", function hideDropdown() {
                dropdownVisible.value = false;
                window.removeEventListener('click', hideDropdown);
            });
        }, 300);
    }
}

const shouldShowCalculationSpinner = computed(() => {
    return chessEngine.engineSearching && props.playerColor == chessEngine.currentPlayerColor;
})

</script>

<template>
    <div class="flex flex-row gap-2 relative z-10">
        <img width="55" height="55" class="w-[50px] h-[50px] rounded-[3px] box-box" 
        :class="[chessEngine.currentPlayerColor == playerColor && 'border-[3px] border-solid border-green-600']" :src="playerInfo?.profile">
        <FontAwesomeIcon v-if="shouldShowCalculationSpinner" title="Calculating..." spin class="absolute left-[52px] top-[25px]  duration-300 ease-in-out ml-[6px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" :icon="faSpinner"/>
        <div @click="toggleDropdownVisibility" class="cursor-pointer flex flex-row hover:scale-105 transition-all duration-300 ease-in-out">
            <span class="text-sm font-bold">{{ playerInfo?.name }}</span>
            <FontAwesomeIcon class="cursor-pointer hover:scale-110 duration-300 ease-in-out ml-[6px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" :icon="faCaretDown"/>
        </div>
        <div v-if="dropdownVisible" class="absolute flex flex-col left-[60px] top-[22px] bg-container/95 w-[220px] p-2 rounded-[8px]">
            <div @click="switchPlayer(player)" v-for="player in chessEngine.availablePlayers" class="flex flex-row gap-2 items-center group cursor-pointer min-h-[30px]">
                <img width="20" height="20" class="w-[20px] h-[20px] group-hover:scale-125 rounded-[3px] transition-all duration-300 ease-in-out" :src="player.profile"/>
                <div class="group-hover:scale-110 group-hover:ml-1 text-sm transition-all duration-300 ease-in-out">{{player.name}}</div>
            </div>
        </div>
    </div>
</template>

<style scoped>

</style>