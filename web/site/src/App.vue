<script setup lang="ts">
import { onMounted, ref, computed, onBeforeMount } from 'vue';
import Board from './components/Board.vue';
import PlayerInfo from './components/PlayerInfo.vue';
import BottomControls from './components/BottomControls.vue';
import TopControls from './components/TopControls.vue';
import EngineInfo from './components/EngineInfo.vue';
import { useUiStore } from './store/ui';
import { useChessEngineStore } from './store/engine';
import PopupDialog from './components/PopupDialog.vue';
import GameEndPopup from './components/GameEndPopup.vue';

const uiStore = useUiStore();
const chessEngine = useChessEngineStore();

const sidebarVisible = computed(() => uiStore.sidebarVisible);

onBeforeMount(async () => {
  chessEngine.initWasmWorker();
});

</script>

<template>
    <div class="layout h-screen text-primary flex flex-row items-center justify-center">
        <div class="absolute">
          <PopupDialog></PopupDialog>
          <GameEndPopup></GameEndPopup>
        </div>
        <div class="flex flex-col justify-center items-center gap-4 w-[min(500px,100vw)] 2xl:w-[62vh]">
            <div class="w-full px-6 flex flex-col gap-3">
              <div class="flex flex-row justify-between">
                <PlayerInfo player-color="black"/>
                <TopControls class="mt-auto invisible md:visible"/>
              </div>
              <Board class="rounded-[8px] overflow-hidden"/>
              <div class="flex flex-row justify-between">
                <PlayerInfo player-color="white"/>
                <BottomControls/>
              </div>
            </div>
        </div>  
        <transition name="slide" mode="in-out">
          <div v-if="sidebarVisible" class="invisible w-0 md:visible md:w-[300px] 3xl:w-[400px] h-[530px] max-h-[530px] 2xl:h-[calc(62vh+30px)] 2xl:max-h-[calc(62vh+30px)] md:px-0 rounded-[8px] overflow-hidden">
            <EngineInfo/>
          </div>
        </transition>
    </div>
</template>

<style>

</style>