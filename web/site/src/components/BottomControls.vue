<script setup>
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faPlay } from '@fortawesome/free-solid-svg-icons/faPlay'
import { faPause } from '@fortawesome/free-solid-svg-icons/faPause'
import { faRotateLeft } from '@fortawesome/free-solid-svg-icons/faRotateLeft'
import { faArrowsRotate } from '@fortawesome/free-solid-svg-icons/faArrowsRotate'
import { useChessEngineStore } from '../store/engine';

const chessEngine = useChessEngineStore();

</script>

<template>
    <div class="flex justify-end gap-3">
        <FontAwesomeIcon class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faArrowsRotate"/>
        <FontAwesomeIcon class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faRotateLeft"/>
        <FontAwesomeIcon @click="chessEngine.gamePaused = false" v-if="chessEngine.gamePaused" class="cursor-pointer hover:scale-110 duration-300 ease-in-out w-[18px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faPlay"/>
        <FontAwesomeIcon @click="chessEngine.gamePaused = true" v-else class="cursor-pointer hover:scale-110 duration-300 ease-in-out w-[18px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="xl" :icon="faPause"/>
    </div>
</template>

<style scoped>

</style>