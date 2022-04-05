// https://github.com/padenot/ringbuf.js
// customised for Glicol
// TextParameter has a varied length
class TextParameterWriter {
  // From a RingBuffer, build an object that can enqueue a parameter change in
  // the queue.
  constructor(ringbuf) {
    if (ringbuf.type() != "Uint8Array") {
      throw "This class requires a ring buffer of Uint8Array";
    }
    // const SIZE_ELEMENT = 5;
    this.ringbuf = ringbuf
  }
  enqueue(buf) {
    return this.ringbuf.push(buf);
  }
  // Query the free space in the ring buffer. This is the amount of samples that
  // can be queued, with a guarantee of success.
  available_write() {
    return this.ringbuf.available_write();
  }
}

class TextParameterReader {
  constructor(ringbuf) {
    if (ringbuf.type() != "Uint8Array") {
      throw "This class requires a ring buffer of Uint8Array";
    }
    this.ringbuf = ringbuf;
  }
  dequeue(buf) {
    if (this.ringbuf.empty()) {
      return 0;
    }
    return this.ringbuf.pop(buf);
  }
  // Query the occupied space in the queue. This is the amount of samples that
  // can be read with a guarantee of success.
  available_read() {
    return this.ringbuf.available_read();
  }
}

// A Single Producer - Single Consumer thread-safe wait-free ring buffer.
//
// The producer and the consumer can be separate thread, but cannot change role,
// except with external synchronization.
class RingBuffer {
  static getStorageForCapacity(capacity, type) {
    if (!type.BYTES_PER_ELEMENT) {
      throw "Pass in a ArrayBuffer subclass";
    }
    var bytes = 8 + (capacity + 1) * type.BYTES_PER_ELEMENT;
    return new SharedArrayBuffer(bytes);
  }
  constructor(sab, type) {
    if (!ArrayBuffer.__proto__.isPrototypeOf(type) &&
      type.BYTES_PER_ELEMENT !== undefined) {
      throw "Pass a concrete typed array class as second argument";
    }
    // Maximum usable size is 1<<32 - type.BYTES_PER_ELEMENT bytes in the ring
    // buffer for this version, easily changeable.
    // -4 for the write ptr (uint32_t offsets)
    // -4 for the read ptr (uint32_t offsets)
    // capacity counts the empty slot to distinguish between full and empty.
    this._type = type;
    this.capacity = (sab.byteLength - 8) / type.BYTES_PER_ELEMENT;
    this.buf = sab;
    this.write_ptr = new Uint32Array(this.buf, 0, 1);
    this.read_ptr = new Uint32Array(this.buf, 4, 1);
    this.storage = new type(this.buf, 8, this.capacity);
  }
  // Returns the type of the underlying ArrayBuffer for this RingBuffer. This
  // allows implementing crude type checking.
  type() {
    return this._type.name;
  }
  push(elements) {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);

    if ((wr + 1) % this._storage_capacity() == rd) {
      // full
      return 0;
    }

    let to_write = Math.min(this._available_write(rd, wr), elements.length);
    let first_part = Math.min(this._storage_capacity() - wr, to_write);
    let second_part = to_write - first_part;

    this._copy(elements, 0, this.storage, wr, first_part);
    this._copy(elements, first_part, this.storage, 0, second_part);

    // publish the enqueued data to the other side
    Atomics.store(
      this.write_ptr,
      0,
      (wr + to_write) % this._storage_capacity()
    );

    return to_write;
  }
  pop(elements) {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);

    if (wr == rd) {
      return 0;
    }

    let to_read = Math.min(this._available_read(rd, wr), elements.length);

    let first_part = Math.min(this._storage_capacity() - rd, elements.length);
    let second_part = to_read - first_part;

    this._copy(this.storage, rd, elements, 0, first_part);
    this._copy(this.storage, 0, elements, first_part, second_part);

    Atomics.store(this.read_ptr, 0, (rd + to_read) % this._storage_capacity());

    return to_read;
  }

  // True if the ring buffer is empty false otherwise. This can be late on the
  // reader side: it can return true even if something has just been pushed.
  empty() {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);

    return wr == rd;
  }

  // True if the ring buffer is full, false otherwise. This can be late on the
  // write side: it can return true when something has just been poped.
  full() {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);

    return (wr + 1) % this.capacity != rd;
  }

  // The usable capacity for the ring buffer: the number of elements that can be
  // stored.
  capacity() {
    return this.capacity - 1;
  }

  // Number of elements available for reading. This can be late, and report less
  // elements that is actually in the queue, when something has just been
  // enqueued.
  available_read() {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);
    return this._available_read(rd, wr);
  }

  // Number of elements available for writing. This can be late, and report less
  // elemtns that is actually available for writing, when something has just
  // been dequeued.
  available_write() {
    var rd = Atomics.load(this.read_ptr, 0);
    var wr = Atomics.load(this.write_ptr, 0);
    return this._available_write(rd, wr);
  }

  // private methods //

  // Number of elements available for reading, given a read and write pointer..
  _available_read(rd, wr) {
    if (wr > rd) {
      return wr - rd;
    } else {
      return wr + this._storage_capacity() - rd;
    }
  }

  // Number of elements available from writing, given a read and write pointer.
  _available_write(rd, wr) {
    let rv = rd - wr - 1;
    if (wr >= rd) {
      rv += this._storage_capacity();
    }
    return rv;
  }

  // The size of the storage for elements not accounting the space for the index.
  _storage_capacity() {
    return this.capacity;
  }

  _copy(input, offset_input, output, offset_output, size) {
    for (var i = 0; i < size; i++) {
      output[offset_output + i] = input[offset_input + i];
    }
  }
}

class GlicolEngine extends AudioWorkletProcessor {
    static get parameterDescriptors() {
        return []
    }
    constructor(options) {
        super(options)
        this._codeArray = new Uint8Array(4096);
        const { codeQueue } = options.processorOptions;
        this._param_reader = new TextParameterReader(new RingBuffer(codeQueue, Uint8Array));
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
            } else if (e.data.type === "bpm") {
                this._wasm.exports.set_bpm(e.data.value);
            } else if (e.data.type === "amp") {
                this._wasm.exports.set_track_amp(e.data.value);
            // } else if (e.data.type === "sab") {
                
            // } else if (e.data.type === "result") {
                // this._result_reader = new TextParameterReader(new RingBuffer(e.data.data, Uint8Array));
            } else {
                throw "unexpected.";
            }
        }
    }

    process(inputs, outputs, _parameters) {
        if(!this._wasm) {
            return true
        }

        let size = this._param_reader.dequeue(this._codeArray)
        if (size) {
            let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
            let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, size);
            codeUint8Array.set(this._codeArray.slice(0, size));
            this._wasm.exports.update(codeUint8ArrayPtr, size)
        }

      //   if (midiSize) {
      //     let codeUint8ArrayPtr = this._wasm.exports.alloc_uint8array(size);
      //     let codeUint8Array = new Uint8Array(this._wasm.exports.memory.buffer, codeUint8ArrayPtr, size);
      //     codeUint8Array.set(this._codeArray.slice(0, size));

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
          this.port.postMessage({type: 'e', info: this._result.slice(0,256)})
        }
    
        outputs[0][0].set(this._outBuf.slice(0, 128))
        outputs[0][1].set(this._outBuf.slice(128, 256))
        return true
    }
}

registerProcessor('glicol-engine', GlicolEngine)