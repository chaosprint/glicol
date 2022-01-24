// when publish, change the exact version number
// in local testing, comment the version out!
window.version = "v0.2.24"
const source = window.version ? `https://cdn.jsdelivr.net/gh/chaosprint/glicol@${version}/js/src/` : "src/"

window.loadDocs = async () => {
  fetch(source+'glicol-docs.json')
  .then(response => response.json())
  .then(data => window.docs = data)
}

window.loadDocs()

window.help = async (token) => {
    if (!window.docs) {
      await window.loadDocs()
    }
    if (token in window.docs) {
        // clear()
        let node = window.docs[token]
        log(`%cName: %c${token}`, "color: red", "")
        log(`%cParameters: %c${"description" in node ? node["description"] : null }`, "color: orange", "")
        table(node["parameters"])
        log(`%cIutput: %c${node["input"] !== null ? node["input"].description : null }`, "color: yellow", "")
        if (node["input"] !== null) {table(node["input"].range)}
        log(`%cOutput: %c${node["output"].description}`, "color: green", "")
        table(node["output"].range)
        log(`%cExample:`, "color: cyan")
        node["example"].forEach(e=>log(e))
    }  else {
        warn(`Move your cursor to an non-empty place where you wish to search.
        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
    }
}

window.bpm = (beats_per_minute) => {
const t0 = performance.now();
if (typeof beats_per_minute === "number") {
    window.node.port.postMessage({
    type: "bpm", value: beats_per_minute})
    log(`%cBPM set to: ${beats_per_minute}`, "background: green");
    log("%c This will be effective when you make some changes to the code.", "background: yellow");
} else {
    warn("BPM should be a number.")
}
return `Execution time: ${(performance.now()-t0).toFixed(4)} ms`
}

window.trackAmp = (amp) => {
const t0 = performance.now();
if (typeof amp === "number") {
    if (amp <= 1.0) {
    window.node.port.postMessage({
        type: "amp", value: amp})
    log(`%cThe amplitude of each track is set to: ${amp}`,"background: green");
    } else {
    warn("Amplitude should not exceed 1.0.")
    }
} else {
    warn("Amplitude should be a number.")
}
return `Execution time: ${(performance.now()-t0).toFixed(4)} ms`
}

window.sampleFolder = async () => {
    var input = document.createElement('input');
    input.type = 'file';
    input.webkitdirectory = true
    input.directory = true
    input.multiple = true

    window.samples = {}
    input.onchange = async (e) => {
        var files = e.target.files;
        log(`%cSome samples will be skiped as only mono samples are supported so far.`, "color: red; font-weight: bold", "")
        for (var i = 0; i < files.length; i++) {
            (async function(file) {
                var reader = new FileReader();
                reader.onload = async function(e) {
                    if (file.type === "audio/wav") {

                        await window.actx.decodeAudioData(e.target.result, buffer => {
                            if (buffer.numberOfChannels === 1) {
                              let path = file.webkitRelativePath.split("/")
                              path.shift()
                              if (path[0] in window.samples) {
                                window.samples[path[0]] += 1
                              } else {
                                window.samples[path[0]] = 0
                              }
                              let key = path[0].toLowerCase() + "_" + String(window.samples[path[0]])
                              window.node.port.postMessage({
                                type: "samples",
                                sample: buffer.getChannelData(0),
                                name: encoder.encode(key.replace(".wav", ""))
                              })
                              log(`Sample %c${key.replace(".wav", "")} %cloaded`, "color: green; font-weight: bold", "")
                            }
                        })
                    }
                };
                reader.readAsArrayBuffer(file);
            })(files[i]);
        }
    }
    input.click();
}

window.sampleCount = () => {
  let a = []
  for (let key in window.samples) {
    let b = {}
    b[key] = window.samples[key] + 1
    a.push(b)
  }
  a.sort((a, b) => {
    if (String(a) > String(b)) {
      return 1
    } else {
      return -1
    }
  })
  log(...a)
  log("For example, if you load dirt samples, there are 25 808bd samples {808bd: 25}. You can write Glicol code:\n\n%cout: seq 60 >> sp \\808bd_24\n\n%cThe avalable range for samplename_index is from 0 to sampleAmount - 1.", "background-color: grey; font-weight: bold", "")
}

window.addSample = async (name, url) => {
    if (url === undefined) {

        var input = document.createElement('input');
        input.type = 'file';
        input.multiple = true

        input.onchange = e => {
            var files = e.target.files;
            // log(files)
            for (var i = 0; i < files.length; i++) {
                (function(file) {
                    var reader = new FileReader();
                    reader.onload = async function(e) {
                        let key = file.name.toLowerCase()
                        await window.actx.decodeAudioData(e.target.result, buffer => {
                            window.node.port.postMessage({
                              type: "samples",
                              sample: buffer.getChannelData(0),
                              name: encoder.encode(key.replace(".wav", ""))
                            })
                        })
                        log(`Sample %c${key.replace(".wav", "")} %cloaded`, "color: green; font-weight: bold", "")
                    };
                    reader.readAsArrayBuffer(file);
                  })(files[i]);
                // key = name[i] ? name[i] : files[i].name
            }
        }

        input.click();
    } else {
        window.actx.suspend()
        let myRequest = new Request(url);
        await fetch(myRequest).then(response => response.arrayBuffer())
        .then(arrayBuffer => {
            window.actx.decodeAudioData(arrayBuffer, buffer => {
                // log(new Int16Array(buffer.getChannelData(0).buffer))
                window.node.port.postMessage({
                  type: "samples",
                  sample: buffer.getChannelData(0),
                  name: encoder.encode(name)
                })
            })
        });
    }
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

exports.TextParameterReader = TextParameterReader;
exports.TextParameterWriter = TextParameterWriter;
exports.RingBuffer = RingBuffer;

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

window.log = function consoleWithNoSource(...params) {
  setTimeout(console.log.bind(console, ...params));
}

window.table = function consoleWithNoSource(...params) {
  setTimeout(console.table.bind(console, ...params));
}

window.clear = function consoleClear() {
  setTimeout(console.clear.bind());
}

window.warn = function consoleWithNoSource(...params) {
  setTimeout(console.warn.bind(console, ...params));
}

window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    sampleRate: 44100
  })

  URLFromFiles([source+'glicol-engine.js']).then((e) => {
    
    window.actx.audioWorklet.addModule(e).then(() => {
      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})
      fetch(source+'glicol_wasm.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        window.node.port.postMessage({
        type: "load", obj: arrayBuffer})
      })

      window.actx.destination.channelInterpretation = "discrete";
      window.node.connect(window.actx.destination)

      let sab = exports.RingBuffer.getStorageForCapacity(2048, Uint8Array);
      let rb = new exports.RingBuffer(sab, Uint8Array);
      window.paramWriter = new TextParameterWriter(rb);
      window.node.port.postMessage({
          type: "sab",
          data: sab
      });
      
      window.decoder = new TextDecoder('utf-8');

      const errors = [
        "SampleNotExistError",
        "NonExistControlNodeError",
        "ParameterError",
        "HandleNodeError",
        "ParsingError",
        "NodeNameError",
        "ParaTypeError",
        "NotModuableError",
        "InsufficientParameter",
        "UnknownError"
      ]

      window.node.port.onmessage = e => {
        if (e.data[0] > errors.length) {
          log(e.data[0]-1)
        }
        log(`%cError: ${errors[e.data[0]-1]}`, "color: white; background: red")
        if (e.data[0] === 2) {
            let name = decoder.decode(e.data.slice(2).filter(v => v !== 0.0))
            let index = window.code.indexOf(name)
            let code = window.code.slice(0, index)
    
            let line = code.split("\n").length;
            log("%cAt line "+String(line)+".", "color: white; background: green")
        } else {
            log("%cAt line "+String(e.data[1]+1)+".", "color: white; background: green")
        }
        log("%cError element: "+String(decoder.decode(e.data.slice(2))).replace(/[^ -~]+/g, ""), "color:white;background:pink");
    }

      clear();
      // log("%cGlicol has now launched an official website 🚀: \n\nhttps://glicol.org\n\nStill, this playground will continue to be used for quick prototyping, solo live coding and code sharing.", "font-size: 16px")
      log("%c"+window.art, "color: gray") //#3E999F
      // log("%c"+window.version, "background: black; color:white")
      log(`\n\n%c Available nodes: `, "background: black; color:white; font-weight: bold");
      log(["seq","speed","choose","mul","add","apfdecay","delayn",
      "sin","saw","squ","imp","envperc","sampler","noiz","lpf","plate","onepole",
      "hpf","pha","pan","delay","apfgain","comb","mix","monosum",
      "const_sig","*","sp","spd","tri","noise","amplfo","balance"])
  
      // log(`\n\n%c Fetch help files by: `, "background: black; color:white; font-weight: bold")
      // log(`Move the cursor to a keyword and press %cAlt+D`, "color:green;font-weight:bold", "color: default", "color:green; font-weight:bold", "color:default", "color: green; font-weight:bold");

      log(`\n\n%c Useful console commands: `, "background: black; color:white; font-weight: bold")
      log(`\n%chelp()\n%cGet docs for a node, e.g. help("sin").
      
%cbpm()\n%cSet the BPM. The default is 120.

%csampleFolder()\n%cChoose a folder that contains samples. The folder you select must have sub-folders that contain samples. For example, (1) visit (https://github.com/chaosprint/Dirt-Samples), click [code] -> [download ZIP]; (2) Extract {Dirt-Samples-master.zip} to {Dirt-Samples-master} folder; (3) Run this command in the console and choose the folder.

%csampleCount()\n%cUse it after calling the "sampleFolder()" function to see the total number of each sample folder.

%caddSample()\n%cAdd your own samples. The first argument is the sample name you wish to call, and the second arg is the url to the wav file. Keep the augument empty to load local samples. The files should end with .wav. The file name will become the keys. Only lowercase letters and numbers are valid keys, e.g 808bd.`, "color:green; font-weight:bold", "", "color:green; font-weight:bold", "", "color:green; font-weight:bold", "", "color:green; font-weight:bold", "", "color:green; font-weight:bold", "");
    })
  })
}
window.loadModule();

window.code = `~a: choose 48 55 51 58

~b: choose 36 60 0 0 0 0 0

// how about changing the speed to 4.0 and 
//click the update button above?
~trigger: speed 8.0 >> seq ~a ~b >> mul 2.0

~env: ~trigger >> envperc 0.01 0.1 >> mul 0.2

~pitch: ~trigger >> mul 261.626

lead: saw ~pitch >> mul ~env >> rlpf ~cut 3.0 
>> mul 0.6 >> plate 0.1

~cut: squ 0.5 >> mul 3700.0 >> add 4000.0`


window.isGlicolRunning = false

window.encoder = new TextEncoder('utf-8');

window.runCode = (code) => {


  try {
    window.actx.suspend()
    window.actx.resume()
    window.isGlicolRunning = true
  } catch (e) {
    console.log(e)
  }

  try {
    window.node.port.postMessage({
      type: "run",
      value: window.encoder.encode(code)
    })
  } catch (e) {
    console.log(e)
  }
}

window.updateCode = (code) => {
  try { window.actx.resume() } catch (e) {console.log(e)}
  if (window.paramWriter.available_write()) {
    window.paramWriter.enqueue(window.encoder.encode(code))
  }
  window.node.onmessage = (event) => {
    // Handling data from the processor.
    console.log(event);
  };
}

window.run = (code) =>{
  const regexp = /{.*}/g
  let match;
  while ((match = regexp.exec(code)) !== null) {
    let result = Function("return " + match[0].slice(1,match[0].length-1))
    code = code.slice(0, match.index) + result() + code.slice(regexp.lastIndex)
    // console.log(code)
  }

  if (!window.isGlicolRunning) {
    window.runCode(code)
  } else {
    window.updateCode(code)
  }
}

window.stop = async () => {
  window.isGlicolRunning = false
  window.clear()
  await window.actx.close();
  window.loadModule()
}

window.artsource = `
 ██████╗ ██╗     ██╗ ██████╗ ██████╗ ██╗     
██╔════╝ ██║     ██║██╔════╝██╔═══██╗██║     
██║  ███╗██║     ██║██║     ██║   ██║██║     
██║   ██║██║     ██║██║     ██║   ██║██║     
╚██████╔╝███████╗██║╚██████╗╚██████╔╝███████╗
 ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ╚═════╝ ╚══════╝`

window.art = window.version ? window.artsource + "\n\n" + window.version : window.artsource + "\n\n" + "Local Test Version"