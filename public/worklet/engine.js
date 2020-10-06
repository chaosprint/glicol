class GlicolEngine extends AudioWorkletProcessor {
    static get parameterDescriptors() {
        return [
          {
            name: 'freq',
            defaultValue: 440.0
          },
          {
            name: 'cutoff',
            defaultValue: 1000.0
          },
        ]
    }
    constructor() {
        super()
        var sampleLength, samplePtr, sampleArray,
        ptrArr = [], lenArr = [],
        nameArr = [], nameLenArr = [];

        var allocUint32Array = (arr, wasmFunc, wasmBuffer) => {
            let len = arr.length
            let ptr = wasmFunc(len); // actually it's byteoffset
            let tempArr = new Uint32Array(wasmBuffer, ptr, len)
            tempArr.set(arr)
            return {ptr: ptr, len: len}
        }

        this.port.onmessage = e => {
            
            if (e.data.type === "load") {
                WebAssembly.instantiate(e.data.obj).then(obj => {
                    this._wasm = obj.instance
                    this._size = 128
                    this._outPtr = this._wasm.exports.alloc(this._size)
                    this._outBuf = new Float32Array(
                      this._wasm.exports.memory.buffer,
                      this._outPtr,
                      this._size
                    )
                })

            } else if (e.data.type === "samples") {
                // console.log("edatasample", e.data.sample)
                let _s = e.data.sample
                let s = Float32Array.from(_s, i => i/32768.0)

                sampleLength = s.length;
                samplePtr = this._wasm.exports.alloc(sampleLength);
                sampleArray = new Float32Array(
                    this._wasm.exports.memory.buffer,
                    samplePtr,
                    sampleLength
                );

                ptrArr.push(samplePtr)
                lenArr.push(sampleLength)
                
                sampleArray.set(s);

                let nameLen = e.data.name.byteLength
                let namePtr = this._wasm.exports.alloc_uint8array(nameLen);
                let name = new Uint8Array(this._wasm.exports.memory.buffer, namePtr, nameLen);
                name.set(e.data.name);
                           
                nameArr.push(namePtr)
                nameLenArr.push(nameLen)

                // need to reset this
                this._outBuf = new Float32Array(
                    this._wasm.exports.memory.buffer,
                    this._outPtr,
                    this._size
                )
            } else if (e.data.type === "run") {

                console.log("samplePtr, Length", samplePtr, sampleLength)

                // the code as Uint8 to parse; e.data.value == the code
                let length = e.data.value.byteLength
                let myWasmArrayPtr = this._wasm.exports.alloc_uint8array(length);
                let myWasmArray = new Uint8Array(this._wasm.exports.memory.buffer, myWasmArrayPtr, length);
                myWasmArray.set(e.data.value);

                let sampleInfo = allocUint32Array(ptrArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)
                let lengthInfo = allocUint32Array(lenArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)

                let nameInfo = allocUint32Array(nameArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)
                let nameLenInfo = allocUint32Array(nameLenArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)

                this._wasm.exports.run(
                    myWasmArrayPtr, length, 
                    sampleInfo.ptr, sampleInfo.len,
                    lengthInfo.ptr, lengthInfo.len,
                    nameInfo.ptr, nameInfo.len,
                    nameLenInfo.ptr, nameLenInfo.len
                )              
            } else if (e.data.type === "update") {

                // the code as Uint8 to parse
                let length = e.data.value.byteLength
                let myWasmArrayPtr = this._wasm.exports.alloc_uint8array(length);
                let myWasmArray = new Uint8Array(this._wasm.exports.memory.buffer, myWasmArrayPtr, length);
                myWasmArray.set(e.data.value);

                // for updating, no need to pass in samples
                this._wasm.exports.update(myWasmArrayPtr, length)         
            }
        }
    }

    process(_inputs, outputs, _parameters) {
        if(!this._wasm) {
            return true
        }
        let output = outputs[0]
        for (let channel = 0; channel < output.length; ++channel) {
            this._wasm.exports.process(this._outPtr, this._size)
            output[channel].set(this._outBuf)
            // console.log(this._outBuf)
        }
        return true
    }
}

registerProcessor('glicol-engine', GlicolEngine)