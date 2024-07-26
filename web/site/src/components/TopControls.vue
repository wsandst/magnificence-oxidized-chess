<script setup lang="ts">
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faCaretRight } from '@fortawesome/free-solid-svg-icons/faCaretRight';
import { faCaretLeft } from '@fortawesome/free-solid-svg-icons/faCaretLeft';
import { useUiStore } from '../store/ui';

const uiStore = useUiStore();

</script>

<template>
    <div class="flex justify-end gap-3">
        <FontAwesomeIcon
            @click="uiStore.sidebarVisible = false" 
            v-if="uiStore.sidebarVisible" 
            class="cursor-pointer hover:scale-110 duration-300 ease-in-out" 
            :style="{ color: 'hsla(0, 0%, 96%, 1)' }" 
            size="xl" 
            :icon="faCaretLeft"
        />
        <FontAwesomeIcon 
            @click="uiStore.sidebarVisible = true" 
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