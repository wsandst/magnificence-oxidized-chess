<script setup lang="ts">
import { ref } from 'vue'
import { useChessEngineStore } from '../store/engine';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faTimes } from '@fortawesome/free-solid-svg-icons/faTimes';

const chessEngine = useChessEngineStore();
chessEngine.showGameEndPopupCallback = showGameEndPopup;

const isInvisible = ref(true);
const isHidden = ref(true);
const gameStatus = ref(null);

function showGameEndPopup(status: "white_won" | "black_won" | "stalemate") {
    isInvisible.value = false;
    isHidden.value = false;
    gameStatus.value = status;
}

function newGame() {
    isInvisible.value = true;
    isHidden.value = true;
    gameStatus.value = null;
    chessEngine.newGame();
}

</script>

<template>
   <div class="relative z-40 ease-in-out duration-300 transition-opacity mx-4 h-auto text-primary" :class="[
      isInvisible && 'opacity-0',
      !isInvisible && !isHidden && 'opacity-100',
      isHidden && '*:hidden']" role="dialog" aria-modal="true">
      <div className="fixed inset-0 bg-gray-500 bg-opacity-50">
     </div>
      <div className="flex justify-center h-screen items-center overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none">
        <div className="relative rounded w-[min(90%,400px)] text-theme-700  shadow-md shadow-theme-900/10 dark:shadow-theme-900/20 bg-container/95">
          <div className="flex-col w-fullrounded-md p-0 block font-medium ">
            <div class="flex items-center justify-center p-6 border-t border-solid rounded-b border-primary-darker">
              <div v-if="gameStatus == 'white_won'">
                White has won
              </div> 
              <div v-else-if="gameStatus == 'black_won'">
                Black has won
              </div> 
              <div v-else-if="gameStatus == 'stalemate'">
                Stalemate
              </div> 
            </div>
            <div className="flex items-center justify-center p-6 border-t border-solid rounded-b border-primary-darker">
              <button
                className="text-white bg-blue-500 hover:bg-blue-400 font-bold uppercase text-sm px-6 py-2 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1"
                type="button"
                @click="newGame"
              >
                New Game
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
</template>

<style scoped>

</style>