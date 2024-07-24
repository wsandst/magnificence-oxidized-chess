
import init, { ChessEngine } from '../wasm/magnificence_oxidized_web.js';

class AsyncLock {
    constructor () {
        this.disable = () => {}
        this.promise = Promise.resolve()
    }
  
    enable () {
        this.promise = new Promise(resolve => this.disable = resolve)
    }
}

const lock = new AsyncLock();

async function initWorker() {
    await init();
    let engine = ChessEngine.new();

    // Set callback to handle messages passed to the worker.
    self.onmessage = async (e) => {
        // console.log('Message received from main thread: ', e.data);
        
        // Ensure only one message is processed at a time.
        await lock.promise;
        lock.enable();

        // Run function in chess engine
        let functionName = e.data[0];
        let args = e.data.slice(1);
        let result = null;
        var startTime = performance.now();
        if (functionName in engine) {
            result = await engine[functionName](...args);
        }
        else if (functionName in ChessEngine) {
            result = await ChessEngine[functionName](...args);
        }   
        var endTime = performance.now()

        // Send response back to be handled by callback in main thread.
        self.postMessage([functionName, result, endTime - startTime]);
        lock.disable();
    }
    self.postMessage("initiated");
}

initWorker();
