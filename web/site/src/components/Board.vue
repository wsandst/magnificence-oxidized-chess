
<script setup lang="ts">

function boardPositionModulo(row : number, col: number) : number {
    return (row * 8 + col + row) % 2;
}

</script>
rgba(155, 199, 0, 0.41)
rgb(186, 186, 186)
<template>
    <div class="flex flex-col rounded-md overflow-hidden">
        <div class="flex flex-row" v-for="row in 8" :key="row">
            <div class="h-[50px] w-[50px]" 
                :class="{ 'bg-dark-square': boardPositionModulo(row, col) == 1, 'bg-light-square': boardPositionModulo(row, col) == 0}"
                v-for="col in 8" :key="row * col">
            </div>
        </div>
    </div>
</template>


<style>

</style>