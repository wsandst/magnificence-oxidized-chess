<script setup lang="ts">
import { ref } from 'vue'
import { useChessEngineStore } from '../store/engine';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faTimes } from '@fortawesome/free-solid-svg-icons/faTimes';

const chessEngine = useChessEngineStore();
chessEngine.showCommandDialogCallback = showCommandDialog;
chessEngine.commandResponseCallback = onCommandResponse;

const isInvisible = ref(true);
const isHidden = ref(true);
const commandResponse = ref(null);
const currentCommand = ref(null);
const optionFields = ref([]);

function showCommandDialog(command: any) {
    isInvisible.value = false;
    isHidden.value = false;
    currentCommand.value = command;
    commandResponse.value = null;
}

function onCommandResponse(responseRows: any) {
    commandResponse.value = responseRows.join("\n");
    responseRows.forEach(response => {
        chessEngine.logHistory.push(response);
    });
}

function runCommand() {
    let args = [];
    optionFields.value.forEach((option, i) => {
        let value = option.children[0].children[1].value;
        if (!value) {
            return;
        }
        if (currentCommand.value.args[i].type == "number") {
            args.push(Number(value));
        }
        else {
            args.push(value);
        }
    });
    currentCommand.value.command(...args);
    if (!currentCommand.value.hasResponse) {
        hideCommandDialog();
    }
}

function hideCommandDialog() {
    isInvisible.value = true;
    isHidden.value = true;
}

</script>

<template>
   <div class="relative z-20 ease-in-out duration-300 transition-opacity mx-4 h-auto text-primary" :class="[
      isInvisible && 'opacity-0',
      !isInvisible && !isHidden && 'opacity-100',
      isHidden && '*:hidden']" role="dialog" aria-modal="true">
      <div className="fixed inset-0 bg-gray-500 bg-opacity-50">
     </div>
      <div className="flex justify-center h-screen items-center overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none">
        <div className="relative rounded w-[min(90%,400px)] text-theme-700  shadow-md shadow-theme-900/10 dark:shadow-theme-900/20 bg-container/95">
          <div className="flex-col w-fullrounded-md p-0 block font-medium ">
            <div className="flex items-center justify-between p-5 pr-7 pl-7 border-b border-solid rounded-t border-primary-darker">
              <h3 className="text-3xl font=semibold">{{ currentCommand?.name }}</h3>
              <FontAwesomeIcon
                    class="cursor-pointer hover:scale-110 duration-300 ease-in-out" 
                    :style="{ color: 'hsla(0, 0%, 96%, 1)' }" 
                    size="xl" 
                    @click="hideCommandDialog" 
                    :icon="faTimes"
                /> 
            </div>
              <form className="shadow-md rounded px-8 pt-6 pb-8 w-full">
                <div className="m-2" v-for="option in currentCommand?.args" ref="optionFields">
                  <label>
                    <div className="block text-sm font-bold mb-1">                    
                      {{ option.name }}
                    </div>
                    <input type="text" ref="optionField" className="shadow appearance-none rounded w-full py-2 px-2 bg-theme-50 dark:bg-white/10"
                    />
                  </label>
                </div>
              </form>
            <div className="shadow-md rounded px-8 pt-4 pb-6 w-full border-t border-solid rounded-b border-primary-darker" v-if="currentCommand?.hasResponse">
                <div className="m-2">
                    <div className="block text-sm font-bold mb-1">                    
                      Response
                    </div>
                    <div type="text" className="shadow appearance-none rounded w-full py-2 px-2 bg-theme-50 dark:bg-white/10 min-h-[40px] h-auto whitespace-pre-line">
                        {{ commandResponse }}
                    </div>
                </div>
            </div>
            <div className="flex items-center justify-between p-6 border-t border-solid rounded-b border-primary-darker">
              <button
                @click="hideCommandDialog"
                className="text-white bg-red-500 font-bold hover:bg-red-400 bfont-bold uppercase px-6 py-2 text-sm rounded outline-none focus:outline-none mr-1 mb-1"
                type="button"
              >
                Cancel
              </button>
              <button
                className="text-white bg-blue-500 hover:bg-blue-400 font-bold uppercase text-sm px-6 py-2 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1"
                type="button"
                @click="runCommand"
              >
                Run
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
</template>

<style scoped>

</style>