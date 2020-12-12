const errors = [
    "NonExistControlNodeError",
    "HandleNodeError",
    "ParameterError",
    "SampleNotExistError"
]

class GlicolEngine extends AudioWorkletProcessor {
    static get parameterDescriptors() {
        return [
          {
            name: 'amp',
            defaultValue: 1.0
          }
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
            // this.port.postMessage({value: "hi"})
            
            if (e.data.type === "load") {
                WebAssembly.instantiate(e.data.obj).then(obj => {
                    this._wasm = obj.instance
                    this._size = 256
                    this._outPtr = this._wasm.exports.alloc(this._size)
                    this._outBuf = new Float32Array(
                      this._wasm.exports.memory.buffer,
                      this._outPtr,
                      this._size
                    )
                })

            } else if (e.data.type === "samples") {
                if(this._wasm) {
                // console.log("sample data: ", e.data.sample)
                // console.log("sampler \\" + e.data.name)

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
                }
            } else if (e.data.type === "run") {

                // console.log("samplePtr, Length", samplePtr, sampleLength)

                // the code as Uint8 to parse; e.data.value == the code
                this.code = e.data.value;
                let codeLen = e.data.value.byteLength
                let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(codeLen);
                let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, codeLen);
                codeUint8Array.set(e.data.value);

                let sampleInfo = allocUint32Array(ptrArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)
                let lengthInfo = allocUint32Array(lenArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)

                let nameInfo = allocUint32Array(nameArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)
                let nameLenInfo = allocUint32Array(nameLenArr, this._wasm.exports.alloc_uint32array, this._wasm.exports.memory.buffer)

                this._wasm.exports.run(
                    codeUint8ArrayPtr, codeLen,
                    sampleInfo.ptr, sampleInfo.len,
                    lengthInfo.ptr, lengthInfo.len,
                    nameInfo.ptr, nameInfo.len,
                    nameLenInfo.ptr, nameLenInfo.len
                )

            } else if (e.data.type === "update") {

                // the code as Uint8 to parse
                let codeLen = e.data.value.byteLength
                let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(codeLen);
                let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, codeLen);
                codeUint8Array.set(e.data.value);

                // for updating, no need to pass in samples
                this._wasm.exports.update(codeUint8ArrayPtr, codeLen)
            }
        }
    }

    process(inputs, outputs, _parameters) {
        if(!this._wasm) {
            return true
        }

        // console.log(outputs)
        // let output = outputs[0]
        // console.log("outlen", output.length);
        // for (let channel = 0; channel < output.length; ++channel) {
        if (inputs[0][0]) {
            this._inPtr = this._wasm.exports.alloc(128)
            this._inBuf = new Float32Array(
                this._wasm.exports.memory.buffer,
                this._inPtr,
                128
            )
            this._inBuf.set(inputs[0][0])
        }

        let resultPtr = this._wasm.exports.process(this._inPtr, this._outPtr, this._size)

        this._outBuf = new Float32Array(
            this._wasm.exports.memory.buffer,
            this._outPtr,
            this._size
        )
    
        let result = new Uint8Array(
            this._wasm.exports.memory.buffer,
            resultPtr,
            256
        )
    
        if (result[0] !== 0) {
            console.log("%cNon exist sample.", "color: white; background: red")
            console.log("%cAt line "+String(result[1]+1)+".", "color: white; background: green")
            this.port.postMessage(result.slice(2))
        }
            // console.log(result[0])
        outputs[0][0].set(this._outBuf.slice(0, 128))
        outputs[0][1].set(this._outBuf.slice(128, 256))
            // outputs[0][0].set(this._inBuf);
            // console.log(this._inBuf);
        // console.log(this._inBuf);



        // if (inputs[0][0]) {
        //     outputs[0][0].set(inputs[0][0]);
        // }
        // outputs[0][0].set(this._outBuf.slice(0, 128))
        // outputs[0][1].set(this._outBuf.slice(128, 256))
        // console.log(outputs[0][0], outputs[0][1])
        return true
    }
}

registerProcessor('glicol-engine', GlicolEngine)