<script setup>

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faCaretDown } from '@fortawesome/free-solid-svg-icons/faCaretDown'

defineProps({
    name: {
        type: String,
        required: true
    },
    image: {
        type: String,
        required: true,
    },
})

</script>

<template>
    <div class="flex flex-row gap-2">
        <img width="50" height="50" class="w-[50px] h-[50px] rounded-[3px]" :src="image">
        <div class="cursor-pointer flex flex-row">
            <span class="text-sm font-bold">{{ name }}</span>
            <FontAwesomeIcon class="cursor-pointer hover:scale-110 duration-300 ease-in-out ml-[6px]" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="md" :icon="faCaretDown"/>
        </div>
    </div>
</template>

<style scoped>

</style>