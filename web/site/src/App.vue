<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import Board from './components/Board.vue';
import Header from './components/Header.vue';
import init, { ChessEngine } from '../wasm';
import PlayerInfo from './components/PlayerInfo.vue';
import BottomControls from './components/BottomControls.vue';
import TopControls from './components/TopControls.vue';
import EngineInfo from './components/EngineInfo.vue';
import { useMainStore } from './store/store';

const store = useMainStore();
const sidebarVisible = computed(() => store.sidebarVisible);


const currentPieces : any = ref(null);
let engine : ChessEngine | null = null;

onMounted(async () => {
    // Init wasm
    await init();
    engine = ChessEngine.new();
    store.initAvailablePlayers(engine);
    currentPieces.value = engine.get_pieces();
});

function onPieceMove({from, to}) {
  engine?.make_move(from[0], from[1], to[0], to[1]);
  currentPieces.value = engine?.get_pieces();
}

</script>

<template>
    <div class="layout h-screen text-primary flex flex-row items-center justify-center">
        <div class="flex flex-col justify-center items-center gap-4 w-[min(500px,100vw)]">
            <div class="w-full px-6 flex flex-col gap-3">
              <div class="flex flex-row justify-between">
                <PlayerInfo player-number="1" image="src/assets/images/robot-profile.png" name="Magnificence"/>
                <TopControls class="mt-auto invisible md:visible"/>
              </div>
              <Board class="rounded-[8px] overflow-hidden" :pieces="currentPieces" @piece-moved="onPieceMove"/>
              <div class="flex flex-row justify-between">
                <PlayerInfo player-number="2" image="src/assets/images/human-profile.png" name="Human"/>
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