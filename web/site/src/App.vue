<script setup lang="ts">
import { onMounted } from 'vue';
import Board from './components/Board.vue';
import init, { ChessEngine } from '../wasm';

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
  <div class="flex flex-col items-center py-4 gap-20">
      <h1 class="text-3xl"> Magnificence Oxidized </h1>
      <Board/>
  </div>
</template>


<style>

</style>