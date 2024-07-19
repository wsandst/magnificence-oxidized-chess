<template>
  <h1 class="text-3xl font-bold underline"> Magnificence Oxidized </h1>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import init, { ChessEngine } from '../wasm'

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

<style>

</style>