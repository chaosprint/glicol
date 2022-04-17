import text from './glicol-engine'
import wasm from "./glicol_wasm.wasm"
// import {sin, } from './nodechain'
import {TextParameterReader, TextParameterWriter, RingBuffer} from './ringbuf'
// import { sequence, State, TimeSpan } from '@strudel.cycles/core';

class Engine {
    constructor(isLiveCoding) {
        (async () => {
            // this.sr = 44100;
            this.encoder = new TextEncoder('utf-8');
            this.audioContext = new AudioContext()
            this.audioContext.suspend()

            const blob = new Blob([text], { type: "application/javascript" });
            const module = URL.createObjectURL(blob);
            await this.audioContext.audioWorklet.addModule(module)

            let sab = RingBuffer.getStorageForCapacity(2048, Uint8Array);
            let rb = new RingBuffer(sab, Uint8Array);
            this.codeWriter = new TextParameterWriter(rb);

            let sab2 = RingBuffer.getStorageForCapacity(2048, Uint8Array);
            let rb2 = new RingBuffer(sab2, Uint8Array);
            this.paramWriter = new TextParameterWriter(rb2);

            this.node = new AudioWorkletNode(this.audioContext, 'glicol-engine', {
                outputChannelCount: [2],
                processorOptions: {
                  codeQueue: sab,
                  paramQueue: sab2
                },
                isLiveCoding: isLiveCoding === true ? true: false
            })
            this.node.connect(this.audioContext.destination)
            // wasm({env:{now:Date.now}}).then(res=>window._wasm=res);
            // console.log("wasm func; we don't call it, just want the url",wasm)
            let url = String(wasm).replaceAll(' ', '')
            // console.log("wasm url remove all spaces:",url)
            url = url.split(",\"/")[1];
            // console.log("wasm url trim prefix:",url)
            url = url.split("\")")[0]
            // console.log("wasm url trim end:",url)
            // console.log("url",url)
            fetch(url)
            .then(response => response.arrayBuffer())
            .then(arrayBuffer => {
                this.node.port.postMessage({
                    type: "load", obj: arrayBuffer
                })
            })
        })();
    }
    run(code) {
      this.audioContext.resume()
      if (this.codeWriter.available_write()) {
        this.codeWriter.enqueue(this.encoder.encode(code))
      }
    }

    send_msg(msg) {
      let str;
      str = msg.slice(-1) === ";"? msg : msg+";"
      if (this.paramWriter.available_write()) {
        this.paramWriter.enqueue(this.encoder.encode(str))
      }
    }

    set_bpm(bpm) {
      this.node.port.postMessage({
        type: "bpm", value: bpm
    })
    }

    live_coding_mode(yes_or_no) {
      this.node.port.postMessage({
        type: "livecoding", value: yes_or_no
      })
    }

    reset() {
      // this.node
    }

    play(obj) {
      let code = ``;
      for (let key in obj) {
        code += key + ": ";
        code += obj[key].code + ";\n\n"
      }
      console.log(code)
      this.run(code)
    }
}

export default Engine
export * from './nodechain'