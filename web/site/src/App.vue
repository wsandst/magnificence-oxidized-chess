<script setup lang="ts">
import { onMounted, ref, computed, onBeforeMount } from 'vue';
import Board from './components/Board.vue';
import Header from './components/Header.vue';
import init, { ChessEngine, PiecePosition } from '../wasm';
import PlayerInfo from './components/PlayerInfo.vue';
import BottomControls from './components/BottomControls.vue';
import TopControls from './components/TopControls.vue';
import EngineInfo from './components/EngineInfo.vue';
import { useMainStore } from './store/store';

const store = useMainStore();
const sidebarVisible = computed(() => store.sidebarVisible);

let engine : ChessEngine | null = null;

onBeforeMount(async () => {
  store.initEngineWorker();
});

function onPieceMove({from, to}) {
  store.makeMove(from, to);
}

</script>

<template>
    <div class="layout h-screen text-primary flex flex-row items-center justify-center">
        <div class="flex flex-col justify-center items-center gap-4 w-[min(500px,100vw)]">
            <div class="w-full px-6 flex flex-col gap-3">
              <div class="flex flex-row justify-between">
                <PlayerInfo player-number="1"/>
                <TopControls class="mt-auto invisible md:visible"/>
              </div>
              <Board class="rounded-[8px] overflow-hidden" :pieces="store.currentBoardPieces" @piece-moved="onPieceMove"/>
              <div class="flex flex-row justify-between">
                <PlayerInfo player-number="2"/>
                <BottomControls/>
              </div>
            </div>
        </div>  
        <transition name="slide" mode="in-out">
          <div v-if="sidebarVisible" class="invisible w-0 md:visible md:w-[300px] h-[530px] max-h-[530px] md:px-0 rounded-[8px] overflow-hidden">
            <EngineInfo/>
          </div>
        </transition>
    </div>
</template>

<style>

</style>