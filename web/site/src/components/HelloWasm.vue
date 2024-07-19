<template>
    <button>hello wasm</button>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import init, { ChessEngine } from '../../wasm'

onMounted(async () => {
    // Init wasm
    await init();
    console.log(ChessEngine);
    const engine = ChessEngine.new();
    console.log(engine.test());
    console.log(engine.get_counter());
    engine.increment_counter();
    console.log(engine.get_counter());
});


</script>
