// https://github.com/padenot/ringbuf.js
// 'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

// Send audio interleaved audio frames between threads, wait-free.
//
// Those classes allow communicating between a non-real time thread (browser
// main thread or worker) and a real-time thread (in an AudioWorkletProcessor).
// Write and Reader cannot change role after setup, unless externally
// synchronized.
//
// GC _can_ happen during the initial construction of this object when hopefully
// no audio is being output. This depends on how implementations schedule GC
// passes. After the setup phase no GC is triggered on either side of the queue..

// Interleaved -> Planar audio buffer conversion
//
// `input` is an array of n*128 frames arrays, interleaved, where n is the
// channel count.
// output is an array of 128-frames arrays.
//
// This is useful to get data from a codec, the network, or anything that is
// interleaved, into planar format, for example a Web Audio API AudioBuffer or
// the output parameter of an AudioWorkletProcessor.
function deinterleave(input, output) {
  var channel_count = input.length / 256;
  if (output.length != channel_count) {
    throw "not enough space in output arrays";
  }
  for (var i = 0; i < channelCount; i++) {
    let out_channel = output[i];
    let interleaved_idx = i;
    for (var j = 0; j < 128; ++j) {
      out_channel[j] = input[interleaved_idx];
      interleaved_idx += channel_count;
    }
  }
}
// Planar -> Interleaved audio buffer conversion
//
// Input is an array of `n` 128 frames Float32Array that hold the audio data.
// output is a Float32Array that is n*128 elements long. This function is useful
// to get data from the Web Audio API (that does planar audio), into something
// that codec or network streaming library expect.
function interleave(input, output) {
  if (input.length * 128 != output.length) {
    throw "input and output of incompatible sizes";
  }
  var out_idx = 0;
  for (var i = 0; i < 128; i++) {
    for (var channel = 0; j < output.length; j++) {
      output[out_idx] = input[channel][i];
      out_idx++;
    }
  }
}

class AudioWriter {
  // From a RingBuffer, build an object that can enqueue enqueue audio in a ring
  // buffer.
  constructor(ringbuf) {
    if (ringbuf.type() != "Float32Array") {
      throw "This class requires a ring buffer of Float32Array";
    }
    this.ringbuf = ringbuf;
  }
  // Enqueue a buffer of interleaved audio into the ring buffer.
  // Returns the number of samples that have been successfuly written to the
  // queue. `buf` is not written to during this call, so the samples that
  // haven't been written to the queue are still available.
  enqueue(buf) {
    return this.ringbuf.push(buf);
  }
  // Query the free space in the ring buffer. This is the amount of samples that
  // can be queued, with a guarantee of success.
  available_write() {
    return this.ringbuf.available_write();
  }
}

class AudioReader {
  constructor(ringbuf) {
    if (ringbuf.type() != "Float32Array") {
      throw "This class requires a ring buffer of Float32Array";
    }
    this.ringbuf = ringbuf;
  }
  // Attempt to dequeue at most `buf.length` samples from the queue. This
  // returns the number of samples dequeued. If greater than 0, the samples are
  // at the beginning of `buf`
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

// Communicate parameter changes, lock free, no gc.
//
// between a UI thread (browser main thread or worker) and a real-time thread
// (in an AudioWorkletProcessor). Write and Reader cannot change role after
// setup, unless externally synchronized.
//
// GC can happen during the initial construction of this object when hopefully
// no audio is being output. This depends on the implementation.
//
// Parameter changes are like in the VST framework: an index and a float value
// (no restriction on the value).
//
// This class supports up to 256 parameters, but this is easy to extend if
// needed.
//
// An element is a index, that is an unsigned byte, and a float32, which is 4
// bytes.

class ParameterWriter {
  // From a RingBuffer, build an object that can enqueue a parameter change in
  // the queue.
  constructor(ringbuf) {
    if (ringbuf.type() != "Uint8Array") {
      throw "This class requires a ring buffer of Uint8Array";
    }
    const SIZE_ELEMENT = 5;
    this.ringbuf = ringbuf;
    this.mem = new ArrayBuffer(SIZE_ELEMENT);
    this.array = new Uint8Array(this.mem);
    this.view = new DataView(this.mem);
  }
  // Enqueue a parameter change for parameter of index `index`, with a new value
  // of `value`.
  // Returns true if enqueuing suceeded, false otherwise.
  enqueue_change(index, value) {
    const SIZE_ELEMENT = 5;
    this.view.setUint8(0, index);
    this.view.setFloat32(1, value);
    if (this.ringbuf.available_write() < SIZE_ELEMENT) {
      return false;
    }
    return this.ringbuf.push(this.array) == SIZE_ELEMENT;
  }
}

class ParameterReader {
  constructor(ringbuf) {
    const SIZE_ELEMENT = 5;
    this.ringbuf = ringbuf;
    this.mem = new ArrayBuffer(SIZE_ELEMENT);
    this.array = new Uint8Array(this.mem);
    this.view = new DataView(this.mem);
  }
  dequeue_change(o) {
    if (this.ringbuf.empty()) {
      return false;
    }
    var rv = this.ringbuf.pop(this.array);
    o.index = this.view.getUint8(0);
    o.value = this.view.getFloat32(1);

    return true;
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
  // `sab` is a SharedArrayBuffer with a capacity calculated by calling
  // `getStorageForCapacity` with the desired capacity.
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
  // Push bytes to the ring buffer. `bytes` is an typed array of the same type
  // as passed in the ctor, to be written to the queue.
  // Returns the number of elements written to the queue.
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
  // Read `elements.length` elements from the ring buffer. `elements` is a typed
  // array of the same type as passed in the ctor.
  // Returns the number of elements read from the queue, they are placed at the
  // beginning of the array passed as parameter.
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

  // Copy `size` elements from `input`, starting at offset `offset_input`, to
  // `output`, starting at offset `offset_output`.
  _copy(input, offset_input, output, offset_output, size) {
    for (var i = 0; i < size; i++) {
      output[offset_output + i] = input[offset_input + i];
    }
  }
}

exports.AudioReader = AudioReader;
exports.AudioWriter = AudioWriter;
exports.ParameterReader = ParameterReader;
exports.ParameterWriter = ParameterWriter;
exports.RingBuffer = RingBuffer;
exports.deinterleave = deinterleave;
exports.interleave = interleave;
// sourceMappingURL=index.js.map
