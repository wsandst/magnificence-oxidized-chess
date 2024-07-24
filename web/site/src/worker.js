
import init, { ChessEngine } from '../wasm/magnificence_oxidized_web.js';

async function initWorker() {
    await init();
    let engine = ChessEngine.new();

    // Set callback to handle messages passed to the worker.
    self.onmessage = async (e) => {
        // console.log('Message received from main thread: ', e.data);
        // Run function in chess engine
        let functionName = e.data[0];
        let args = e.data.slice(1);
        let result = null;
        if (functionName in engine) {
            result = await engine[functionName](...args);
        }
        else if (functionName in ChessEngine) {
            result = await ChessEngine[functionName](...args);
        }   

        // Send response back to be handled by callback in main thread.
        self.postMessage([functionName, result]);
    }
    self.postMessage("initiated");
}

initWorker();
