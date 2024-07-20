<script setup>

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGear } from '@fortawesome/free-solid-svg-icons/faGear'


</script>

<template>
    <header class="fixed h-[40px] w-full z-10 invisible">
        <div class="w-[min(500px,100vw)] mx-auto flex justify-between px-6">
            <img class="w-[50px]" src="../assets/icons/magnificence-icon.svg">
            <div class="flex justify-end py-4 gap-3">
                <FontAwesomeIcon class="cursor-pointer hover:scale-110 duration-300 ease-in-out" :style="{ color: 'hsla(0, 0%, 96%, 1)' }" size="2xl" :icon="faGear"/>
            </div>
        </div>
    </header>
</template>

<style scoped>

</style>