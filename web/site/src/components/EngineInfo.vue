
<script setup lang="ts">

import { useChessEngineStore } from '../store/engine';

const chessEngine = useChessEngineStore();

</script>

<template>
    <div class="flex flex-col gap-3 bg-container h-full md:w-[300px] 3xl:w-[400px]  p-3 font-mono text-[13px] break-words break-all">
        <div class="flex flex-row gap-4">
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                Eval: +30
            </div>
            <div class="w-auto flex justify-center items-center bg-container-lighter rounded-[8px] px-[12px] py-[6px] font-bold">
                Depth: +30
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
        <div class="flex-1 flex-col h-full bg-container-lighter rounded-[8px] pt-2">
            <div class="h-[30px] font-bold border-b px-2 mb-2 border-primary-darker">
                Log output
            </div>
            <div class="overflow-auto flex flex-col px-2">
                <span v-for="entry in chessEngine.logHistory">
                    {{entry}}
                </span>
            </div>
        </div>
    </div>
</template>


<style>

</style>