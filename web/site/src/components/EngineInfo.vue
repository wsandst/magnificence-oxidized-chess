
<script setup lang="ts">

import { nextTick, onMounted, ref, watch } from 'vue';
import { useChessEngineStore } from '../store/engine';

const chessEngine = useChessEngineStore();
const logContainer = ref<HTMLElement | null>(null);

// Scroll to the bottom of the log container
const scrollToBottom = async () => {
    await nextTick();
    if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight;
    }
};

watch(() => chessEngine.logHistory.length, () => {
    scrollToBottom();
});

onMounted(() => {
    scrollToBottom();
});

</script>

<template>
    <div class="flex flex-col gap-3 bg-container h-full md:w-[300px] 3xl:w-[400px]  p-3 font-mono text-[13px] break-words break-all">
        <div class="flex flex-row gap-4">
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                Eval: {{ chessEngine.searchMetadata?.eval }}
            </div>
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                Depth: {{ chessEngine.searchMetadata?.depth }}
            </div>
        </div>
        <div class="flex flex-row gap-4">
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                PV: {{ chessEngine.searchMetadata?.pv }}
            </div>
        </div>
        <div class="flex flex-row gap-4">
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                <span>{{chessEngine.currentBoardFenString}} </span>
            </div>
        </div>
        <div class="flex-1 flex flex-col h-0 bg-container-lighter rounded-[8px] py-2">
            <div class="h-[30px] font-bold border-b px-2 mb-2 border-primary-darker">
                Log output
            </div>
            <div class="overflow-auto flex-1 px-2 custom-scrollbar" ref="logContainer">
                <div v-for="entry in chessEngine.logHistory" :key="entry">
                    {{entry}}
                </div> 
                <div v-for="entry in chessEngine.logHistory" :key="entry">
                    {{entry}}
                </div> 
                <div v-for="entry in chessEngine.logHistory" :key="entry">
                    {{entry}}
                </div> 
                <div v-for="entry in chessEngine.logHistory" :key="entry">
                    {{entry}}
                </div> 
                <div v-for="entry in chessEngine.logHistory" :key="entry">
                    {{entry}}
                </div> 
            </div>
        </div>
    </div>
</template>


<style scoped>

</style>