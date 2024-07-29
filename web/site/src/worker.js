
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

    self.onerror = async function(event) {
        await resetWasm();
    };

    async function resetWasm() {
        // WASM function panicked - we need to reinit the web assembly
        await init();
        engine = ChessEngine.new();
        self.postMessage(["engine_crash"]);
        self.postMessage("initiated");
        lock.disable();
    }

    // Set callback to handle messages passed to the worker.
    self.onmessage = async (e) => {
        //console.log('Message received from main thread: ', e.data);
        let functionName = e.data[0];
        let args = e.data.slice(1);
        let result = null;

        // Special handling for abort command, signals to currently running function to abort
        if (functionName == "abort") {
            self.shouldAbort = true;
            return
        }

        // Ensure only one message is processed at a time.
        await lock.promise;
        lock.enable();

        // Run function in chess engine
        var startTime = performance.now();
        try {
            if (functionName in engine) {
                result = await engine[functionName](...args);
            }
            else if (functionName in ChessEngine) {
                result = await ChessEngine[functionName](...args);
            }   
            else {
                console.log(`Worker received WASM function '${functionName}', which does not appear to exist.`)
            }
        }
        catch (e) {
            console.error("Web assembly error: ", e);
            await resetWasm();
        }

        self.shouldAbort = false;
        var endTime = performance.now()

        // Send response back to be handled by callback in main thread.
        self.postMessage([functionName, result, endTime - startTime]);
        lock.disable();
    }
    self.postMessage("initiated");
}

initWorker();
