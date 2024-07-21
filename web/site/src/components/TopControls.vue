<script setup>
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faCaretRight } from '@fortawesome/free-solid-svg-icons/faCaretRight';
import { faCaretLeft } from '@fortawesome/free-solid-svg-icons/faCaretLeft';
import { useMainStore } from '../store/store';

const store = useMainStore();

</script>

<template>
    <div class="flex justify-end gap-3">
        <FontAwesomeIcon
            @click="store.sidebarVisible = false" 
            v-if="store.sidebarVisible" 
            class="cursor-pointer hover:scale-110 duration-300 ease-in-out" 
            :style="{ color: 'hsla(0, 0%, 96%, 1)' }" 
            size="xl" 
            :icon="faCaretLeft"
        />
        <FontAwesomeIcon 
            @click="store.sidebarVisible = true" 
            v-else 
            class="cursor-pointer 
            hover:scale-110 duration-300 ease-in-out" 
            :style="{ color: 'hsla(0, 0%, 96%, 1)' }" 
            size="xl" 
            :icon="faCaretRight"
        />
    </div>
</template>

<style scoped>

</style>