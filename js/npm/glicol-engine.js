export default (t, r) => {
const TextParameterReader = t;
const RingBuffer = r;

class GlicolEngine extends AudioWorkletProcessor {
    static get parameterDescriptors() {
        return []
    }
    constructor(options) {
        super(options)
        this._codeArray = new Uint8Array(2048);
        this._paramArray = new Uint8Array(2048);
        const isLiveCoding = options.processorOptions.isLiveCoding;
        // console.log("options.isLiveCoding", options.processorOptions.isLiveCoding);
        this.useSAB = options.processorOptions.useSAB;
        if (this.useSAB)  {
          // console.log(this.useSAB)
          this._code_reader = new TextParameterReader(
            new RingBuffer(options.processorOptions.codeQueue, Uint8Array));
          this._param_reader = new TextParameterReader(
            new RingBuffer(options.processorOptions.paramQueue, Uint8Array));
        }

        this.port.onmessage = async e => {
            if (e.data.type === "load") {
                await WebAssembly.instantiate(e.data.obj, {
                  env: {
                    now: Date.now
                  }
                }).then(obj => {
                  // console.log(obj)
                    this._wasm = obj.instance
                    this._size = 256
                    this._wasm.exports.live_coding_mode(isLiveCoding);
                    this._resultPtr = this._wasm.exports.alloc_uint8array(256);
                    this._result = new Uint8Array(
                      this._wasm.exports.memory.buffer,
                      this._resultPtr,
                      256
                    )
                    this._resultPtr = this._wasm.exports.alloc_uint8array(256);
                    this._result = new Uint8Array(
                      this._wasm.exports.memory.buffer,
                      this._resultPtr,
                      256
                    )
                    this._outPtr = this._wasm.exports.alloc(this._size)
                    this._outBuf = new Float32Array(
                      this._wasm.exports.memory.buffer,
                      this._outPtr,
                      this._size
                    )
                    // console.log(sampleRate);
                    this._wasm.exports.set_sr(sampleRate);
                    this._wasm.exports.set_seed(Math.random()*4096);
                })
                this.port.postMessage({type: 'ready'})
            } else if (e.data.type === "loadsample") {
              // console.log("data: ", e.data)
              let channels = e.data.channels;
              let length = e.data.sample.length;
              let sr = e.data.sr;

              let samplePtr = this._wasm.exports.alloc(length);
              let sampleArrayBuffer = new Float32Array(
                this._wasm.exports.memory.buffer,
                samplePtr,
                length
              );
              sampleArrayBuffer.set(e.data.sample)

              let nameLen = e.data.name.byteLength
              let namePtr = this._wasm.exports.alloc_uint8array(nameLen);
              let nameArrayBuffer = new Uint8Array(
                this._wasm.exports.memory.buffer, 
                namePtr, 
                nameLen
              );
              nameArrayBuffer.set(e.data.name);
              this._wasm.exports.add_sample(namePtr, nameLen, samplePtr, length, channels, sr)

              // recall this to ensure
              this._outBuf = new Float32Array(
                this._wasm.exports.memory.buffer,
                this._outPtr,
                this._size
              )
              this._result = new Uint8Array(
                this._wasm.exports.memory.buffer,
                this._resultPtr,
                256
              )
            } else if (e.data.type === "run") {
              let code = e.data.value
              let size = code.byteLength
              let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
              let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, size);
              codeUint8Array.set(code.slice(0, size));
              this._wasm.exports.update(codeUint8ArrayPtr, size)
            } else if (e.data.type === "msg") {
              let msg = e.data.value
              let size = msg.byteLength
              let msgUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
              let msgUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, msgUint8ArrayPtr, size);
              msgUint8Array.set(msg.slice(0, size));
              this._wasm.exports.send_msg(msgUint8ArrayPtr, size)
              
            } else if (e.data.type === "bpm") {
                this._wasm.exports.set_bpm(e.data.value);
            } else if (e.data.type === "livecoding") {
              this._wasm.exports.live_coding_mode(e.data.value);
            } else if (e.data.type === "amp") {
                this._wasm.exports.set_track_amp(e.data.value);
            } else {
                throw "unexpected.";
            }
        }
    }

    process(inputs, outputs, _parameters) {
        if(!this._wasm) {
            return true
        }

        if (this.useSAB) {
          let size = this._code_reader.dequeue(this._codeArray)
          if (size) {
              let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
              let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, size);
              codeUint8Array.set(this._codeArray.slice(0, size), "this._codeArray.slice(0, size)");
              this._wasm.exports.update(codeUint8ArrayPtr, size)
          }
          let size2 = this._param_reader.dequeue(this._paramArray)
          if (size2) {
              let paramUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size2);
              let paramUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, paramUint8ArrayPtr, size2);
              paramUint8Array.set(this._paramArray.slice(0, size2));
              this._wasm.exports.send_msg(paramUint8ArrayPtr, size2)
          }
        }

        if (inputs[0][0]) { // TODO: support stereo or multi-chan
            this._inPtr = this._wasm.exports.alloc(128)
            this._inBuf = new Float32Array(
                this._wasm.exports.memory.buffer,
                this._inPtr,
                128
            )
            this._inBuf.set(inputs[0][0])
        }

        this._wasm.exports.process(
          this._inPtr, this._outPtr, this._size, this._resultPtr)

        this._outBuf = new Float32Array(
          this._wasm.exports.memory.buffer,
          this._outPtr,
          this._size
        )

        this._result = new Uint8Array(
          this._wasm.exports.memory.buffer,
          this._resultPtr,
          256
        )
        
        if (this._result[0] !== 0) {
          // console.log(this._result.slice(0,256))
          this.port.postMessage({type: 'e', info: this._result.slice(0,256)})
        }
    
        outputs[0][0].set(this._outBuf.slice(0, 128))
        outputs[0][1].set(this._outBuf.slice(128, 256))
        return true
    }
}

registerProcessor('glicol-engine', GlicolEngine)
}