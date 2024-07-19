
<script setup lang="ts">

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

</script>
rgba(155, 199, 0, 0.41)
rgb(186, 186, 186)
<template>
    <div class="flex flex-col w-full aspect-square rounded-[12px] overflow-hidden">
        <div class="flex flex-row w-full h-[12.5%]" v-for="row in 8" :key="row">
            <div class="w-[12.5%] h-full" 
                :class="{ 'bg-dark-square': boardPositionModulo(row, col) == 1, 'bg-light-square': boardPositionModulo(row, col) == 0}"
                v-for="col in 8" :key="row * col">
            </div>
        </div>
    </div>
</template>


<style>

</style>