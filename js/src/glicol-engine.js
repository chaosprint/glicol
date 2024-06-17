import { initSync, set_sr, set_seed, add_sample, set_bpm, set_track_amp, update, process } from "./glicol_wasm.js"

class GlicolEngine extends AudioWorkletProcessor {
    static get parameterDescriptors() {
        return []
    }
    constructor(options) {
        super(options)
        this._code = ""
        const { wasmBlob } = options.processorOptions;
        initSync(wasmBlob);
        this.port.onmessage = async e => {
            if (e.data.type === "load") {
                set_sr(sampleRate)
                set_seed(Math.random() * 4096)
                this.port.postMessage({type: 'ready'})
            } else if (e.data.type === "loadsample") {
              // console.log("data: ", e.data)
              let channels = e.data.channels;
              let sr = e.data.sr;
              let name = e.data.name

              add_sample(name, e.data.sample, channels, sr)

              // recall this to ensure
            } else if (e.data.type === "run") {
              this.update(e.data.value)
            } else if (e.data.type === "bpm") {
              set_bpm(e.data.value);
            } else if (e.data.type === "amp") {
              set_track_amp(e.data.value);
            // } else if (e.data.type === "sab") {

            } else {
                throw `Unexpected data type ${e.data.type}`;
            }
        }
    }
    update(code) {
        if (code !== this._code) {
            this._code = code
            let result = update(code)
            if (result[0] !== 0) {
                this.port.postMessage({type: 'e', info: result})
            }
        }
    }
    process(_, outputs, _parameters) {
      //   if (midiSize) {
      //     let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
      //     let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, size);
      //     codeUint8Array.set(this._codeArray.slice(0, size));

        let outBuf = process(256)

        outputs[0][0].set(outBuf.slice(0, outBuf.length / 2))
        outputs[0][1].set(outBuf.slice(outBuf.length / 2, outBuf.length))
        return true
    }
}

registerProcessor('glicol-engine', GlicolEngine)
