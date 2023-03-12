// when publish, change the exact version number
// in local testing, comment the version out!

// window.version = "v0.12.12"

window.source = window.version ? `https://cdn.jsdelivr.net/gh/chaosprint/glicol@${version}/js/src/` : "src/"
fetch(source+`utils.js`).then(res=>res.text()).then( text => // ${window.version ? ".min": ""}
  eval(text)
)
window.loadDocs = async () => {
  fetch(source+'glicol-api.json')
  .then(response => response.json())
  .then(data => window.docs = data)
}
window.loadDocs()

// https://github.com/padenot/ringbuf.js
// From a series of URL to js files, get an object URL that can be loaded in an
// AudioWorklet. This is useful to be able to use multiple files (utils, data
// structure, main DSP, etc.) without either using static imports, eval, manual
// concatenation with or without a build step, etc.
function URLFromFiles(files) {
  const promises = files.map(file => fetch(file).then(
    response => response.text()
    )
  )
  return Promise
    .all(promises)
    .then((texts) => {
      const text = texts.join('');
      const blob = new Blob([text], {type: "application/javascript"});
      return URL.createObjectURL(blob);
    });
}

// https://github.com/padenot/ringbuf.js
// customised for Glicol
exports = {}

Object.defineProperty(exports, '__esModule', { value: true });

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

exports.TextParameterReader = TextParameterReader;
exports.TextParameterWriter = TextParameterWriter;
exports.RingBuffer = RingBuffer;

const detectOs = () => {
  var userAgent = window.navigator.userAgent,
    platform = window.navigator.platform,
    macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
    windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'],
    iosPlatforms = ['iPhone', 'iPad', 'iPod'],
    os = null;
  if (macosPlatforms.indexOf(platform) !== -1) {
    os = 'Mac OS';
  } else if (iosPlatforms.indexOf(platform) !== -1) {
    os = 'iOS';
  } else if (windowsPlatforms.indexOf(platform) !== -1) {
    os = 'Windows';
  } else if (/Android/.test(userAgent)) {
    os = 'Android';
  } else if (!os && /Linux/.test(platform)) {
    os = 'Linux';
  }
  return os;
}
const detectBrowser = () => {
  const { userAgent } = navigator
  // alert(userAgent)
  // alert(detectOs());
  let name = "";
  let version = "0.0";
  if (userAgent.includes('Firefox/')) {
    // Firefox
    name = detectOs() === "Android" ? "Firefox for Android": "Firefox"
    version =  userAgent.split("Firefox/")[1]
  // } else if (userAgent.includes('Edg/')) {
    // name = "Edge"
  } else if (userAgent.includes('Chrome/')) {
    name = detectOs() === "Android" ? "Chrome for Android": "Chrome"
    version = userAgent.split("Chrome/")[1].split(" ")[0].split(".")[0]
  } else if (userAgent.includes('Safari/') && userAgent.includes('Version/') ) {
    name = detectOs() === "iOS" ? "Safari on iOS": "Safari"
    version = userAgent.split("Version/")[1].split(" ")[0]
  }
  return {
    name: name,
    version: parseFloat(version)
  }
}

window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    // sampleRate: 44100
  })
  await URLFromFiles([source+`glicol-engine.js`]).then((e) => { //${window.version ? ".min": ""}
    window.actx.audioWorklet.addModule(e).then(() => {
      let sab = exports.RingBuffer.getStorageForCapacity(2048, Uint8Array);
      let rb = new exports.RingBuffer(sab, Uint8Array);
      window.paramWriter = new TextParameterWriter(rb);
      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {
        outputChannelCount: [2],
        processorOptions: {
          codeQueue: sab,
        },
      })
      fetch(source+'glicol_wasm.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        window.node.port.postMessage({
          type: "load", obj: arrayBuffer
        })
      })
      window.actx.destination.channelInterpretation = "discrete";
      window.splitter = window.actx.createChannelSplitter(2);
      window.analyserL = window.actx.createAnalyser();
      window.analyserR = window.actx.createAnalyser();
      window.merger =  window.actx.createChannelMerger(2);

      window.node.connect(window.splitter)
      window.splitter.connect(window.analyserL, 0);
      window.splitter.connect(window.analyserR, 1);
      window.analyserL.connect(window.merger, 0, 0)
      window.analyserR.connect(window.merger, 0, 1)
      window.merger.connect(window.actx.destination)
      window.decoder = new TextDecoder('utf-8');
      window.node.port.onmessage = async e => {
        if (e.data.type === 'ready') {
          
          if (Object.keys(window.sampleBuffers).length !== 0) {
            for (let key in window.sampleBuffers) {
              let buffer = window.sampleBuffers[key];
              var sample;
              if (buffer.numberOfChannels === 1) {
                sample = buffer.getChannelData(0);
              } else if (buffer.numberOfChannels === 2) {
                sample = new Float32Array( buffer.length * 2);
                sample.set(buffer.getChannelData(0), 0);
                sample.set(buffer.getChannelData(1), buffer.length);
              } else {
                throw(Error("Only support mono or stereo samples."))
              }
              window.node.port.postMessage({
                type: "loadsample",
                sample: sample,
                channels: buffer.numberOfChannels,
                length: buffer.length,
                name: encoder.encode("\\"+ key.replace("-","_")),
                sr: buffer.sampleRate
              })
            }
          } else {
            await window.loadSamples()
          }
        } else if (e.data.type === 'e') {
          if (e.data.info[0] === 1) {
            // log("parsing error.")
            let info = decoder.decode(e.data.info.slice(2).filter(v => v !== 0.0));
            log(info)
            let pos = parseInt(info.split("pos[")[1].split("]")[0])
            let line = parseInt(info.split("line[")[1].split("]")[0])
            let col = parseInt(info.split("col[")[1].split("]")[0])
            let positives = info.split("positives[")[1].split("]")[0].replace("EOI", "END OF INPUT").split(",").join(" ||")
            let negatives = info.split("negatives[")[1].split("]")[0].split(",").join(" or")
            log(`%cError at line ${line}`, "background: #3b82f6; color:white; font-weight: bold")
            let errline = window.code.split("\n")[line-1];
            let styleErrLine = errline.slice(0, col-1) + "%c %c" + errline.slice(col-1);
            log(styleErrLine, "font-weight: bold; background: #f472b6; color:white", "");

            let positiveResult = positives.length > 0?
            "expecting "+positives:""
            log(
                `${"_".repeat(col-1 >=0?col-1:0)}%c^^^ ${positiveResult}${negatives.length > 0?"unexpected "+negatives:""}`,
                "font-weight: bold; background: #f472b6; color:white");
          } else {
            log(`%c${decoder.decode(e.data.info.slice(2).filter(v => v !== 0.0))}`,
            "font-weight: bold; background: #f472b6; color:white")
            // background: #3b82f6; color:white; font-weight: bold
          }
        }
      }
    })
  })
}
window.loadModule();
window.code = `o: seq 60 >> sp \\cb`
window.isGlicolRunning = false
window.encoder = new TextEncoder('utf-8');

var {name, _} = detectBrowser();

window.run = async (codeRaw) =>{
  
  let regex = /(##.*?#)/s
  let parse = codeRaw.split(regex).filter(Boolean)
  let code = parse.map(str => {
      if (str.includes("#")) {
        try {
          let result = str.includes('\\n') || str.includes(';') ?
          Function(`'use strict'; return ()=>{${str.replaceAll("#", "")}}`)()() : 
          Function(`'use strict'; return ()=>(${str.replaceAll("#", "")})`)()();
          // log("result", result)
          return typeof result === "undefined"? "": String(result)
        } catch (e) {
          warn(e)
          return ""
        }
      } else {
        return str
      }
  }).join("")

  window.code = code
  try { window.actx.resume() } catch (e) {console.log(e)}
  if (!name.includes("Safari")) {
    if (window.paramWriter.available_write()) {
      window.paramWriter.enqueue(window.encoder.encode(code))
    }
  } else {
    window.node.port.postMessage({
      type: "run", value: window.encoder.encode(code)
    })
  }

  if (!window.isGlicolRunning) {
    if ( document.getElementById("visualizer")) {
      window.visualizeTimeDomainData({canvas: document.getElementById("visualizer"), analyserL: window.analyserL, analyserR: window.analyserR});
    }
    if ( document.getElementById("freqVisualizer")) {
      window.visualizeFrequencyData({canvas: document.getElementById("freqVisualizer"), analyserL: window.analyserL, analyserR: window.analyserR});
    }
    window.isGlicolRunning = true   
  }
}