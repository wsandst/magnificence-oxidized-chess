<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Board from './components/Board.vue';
import Header from './components/Header.vue';
import init, { ChessEngine } from '../wasm';
import PlayerInfo from './components/PlayerInfo.vue';
import GameControls from './components/GameControls.vue';

const currentPieces : any = ref(null);
let engine : ChessEngine | null = null;

onMounted(async () => {
    // Init wasm
    await init();
    engine = ChessEngine.new();
    currentPieces.value = engine.get_pieces();
});

function onPieceMove({from, to}) {
  engine?.make_move(from[0], from[1], to[0], to[1]);
  currentPieces.value = engine?.get_pieces();
}

</script>

<template>
    <div class="layout bg-primary h-screen text-primary">
        <Header/>
        <div class="flex flex-col justify-center items-center gap-4 mx-auto w-[min(500px,100vw)] pt-32">
            <div class="w-full px-6 flex flex-col gap-3">
              <PlayerInfo image="src/assets/images/robot-profile.png" name="Magnificence"/>
              <Board :pieces="currentPieces" @piece-moved="onPieceMove"/>
              <div class="flex flex-row justify-between">
                <PlayerInfo image="src/assets/images/human-profile.png" name="Human"/>
                <GameControls/>
              </div>
            </div>
        </div>
    </div>
</template>

<style>

</style>