<script setup lang="ts">
import { onMounted } from 'vue';
import Board from './components/Board.vue';
import Header from './components/Header.vue';
import init, { ChessEngine } from '../wasm';
import PlayerInfo from './components/PlayerInfo.vue';
import GameControls from './components/GameControls.vue';

onMounted(async () => {
    // Init wasm
    await init();
    const engine = ChessEngine.new();
    console.log(engine.test());
    console.log(engine.get_counter());
    engine.increment_counter();
    console.log(engine.get_counter());
});
</script>

<template>
    <div class="layout bg-primary h-screen text-primary">
        <Header/>
        <div class="flex flex-col justify-center items-center gap-4 mx-auto w-[min(500px,100vw)] pt-32">
            <div class="w-full px-6 flex flex-col gap-3">
              <PlayerInfo image="src/assets/images/robot-profile.png" name="Magnificence"/>
              <Board/>
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