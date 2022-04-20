// from: https://github.com/padenot/ringbuf.js
// author: @padenot Paul Adenot and other contributors
// customised for Glicol by Qichao Lan (chaosprint)
// license: MPL-2.0 License


// TextParameter has a varied length
export class TextParameterWriter {
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

// }

// export function TextParameterReaderDef() {

export class TextParameterReader {
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

// }

// export function RingBufferDef() {


// A Single Producer - Single Consumer thread-safe wait-free ring buffer.
//
// The producer and the consumer can be separate thread, but cannot change role,
// except with external synchronization.

export class RingBuffer {
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
    if (wr == rd) {return 0;}
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
// }
// export {TextParameterReader, TextParameterWriter, RingBuffer}