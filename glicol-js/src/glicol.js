window.sampleDict = {"808":["RS.WAV"],"909":["BT0A0A7.WAV"],"ab":["009_ab2ride.wav"],"bd":["BT0A0DA.wav"],"jazz":["007_SN.wav"],"casio":["high.wav"],"bass":["000_bass1.wav"],"coins":["coins.wav"],"wind":["002_wind2.wav"],"pluck":["BSC3PI.wav"],"short":["sampleoftheday-gtt-snare-drum-020.wav"],"crow":["001_crow2.wav"],"stomp":["004_3.wav"],"tink":["000_tink1.wav"],"perc":["000_perc0.wav"],"cr":["RIDED0.wav"],"bass3":["83249__zgump__bass-0205.wav"],"gtr":["0001_cleanC.wav"],"sax":["005_notes121c.wav"],"lt":["LTAD7.wav"],"peri":["hhx.wav"],"sid":["001_bas.wav"],"rm":["RIMA.wav"],"cc":["CSHD8.wav"],"psr":["002_03.wav"],"arp":["001_arp.wav"],"tech":["tn1kick1.wav"],"can":["006_2.wav"],"sf":["000_bass.wav"],"808ht":["HT75.WAV"],"808lt":["LT00.WAV"],"808bd":["BD7550.WAV"],"808sd":["SD7575.WAV"],"bassdm":["016_BT7A0DA.WAV"],"v":["000_b_blipp01.wav"],"jungle":["jungle4perc2.wav"],"techno":["006_7.wav"],"popkick":["10.wav"],"control":["1.wav"],"tabla2":["23689_loofa_bahia017.wav"],"glitch2":["007_SN.wav"],"808oh":["OH25.WAV"],"voodoo":["003_VoodooSnare.wav"],"tok":["000_0.wav"],"dr2":["000_DR110CHT.WAV"],"hand":["hand7-mono.wav"],"diphone":["023_kd1_025.wav"],"mash":["0.wav"],"tabla":["012_hi_hit3.wav"],"bin":["000_bin1.wav"],"msg":["000_msg0.wav"],"dork2":["4.wav"],"toys":["MusicalMedley-Words.wav"],"feelfx":["doing.wav"],"hmm":["hmm.wav"],"latibro":["002_Sound4.wav"],"ulgab":["gab1.wav"],"jvbass":["002_03.wav"],"h":["4_tock.wav"],"blip":["001_blipp02.wav"],"breaks165":["000_RAWCLN.WAV"]}

window.sampleList = {
    selected: "808 909 ab bd jazz casio bass coins wind short crow stomp tink perc cr bass3 gtr sax lt peri sid rm cc psr arp tech can sf 808ht 808lt 808bd 808sd bassdm v jungle techno popkick control tabla2 glitch2 808oh voodoo tok dr2 hand diphone mash tabla bin msg dork2 toys feelfx hmm latibro ulgab jvbass h blip breaks165".split(" ")
}

const audio = {
    range: {
        low: "-1.0",
        high: "1.0"
    }
}

const range = {
    sin: audio,
    saw: audio,
    squ: audio,
    noiz: audio,
    sampler: {
        range: {
            low: "depends on the sample",
            high: "depends on the sample"
        }
    },
    imp: {
        range: {
            low: "0.0",
            high: "1.0"
        }
    }
}

const params = {
    // sin: para(["freq"])
    sin: [
        ["freq", "determine the frequency of the sine wave", "modulable"]
    ],
    saw: [
        ["freq", "determine the frequency of the sawtooth wave", "modulable"]
    ],
    squ: [
        ["freq", "determine the frequency of the square wave", "modulable"]
    ],
    mul: [
        ["mul", "determine how much the input signal is multiplied/amplified", "modulable"]
    ],
    add: [
        ["add", "determine how much the input signal is added/shifted", "modulable"]
    ],
    imp: [
        ["freq", "determine the frequency of the impluse signal", "not modulable"]
    ],
    sampler: [
        ["sample_name", "determine which sample to use", "not modulable"]
    ],
}

const about = {
    sin: "outputs sine wave audio signal",
    saw: "outputs sawtooth wave audio signal",
    squ: "outputs sawtooth wave audio signal",
    mul: "multiply the input signal by a constant value",
    add: "add the input signal by a constant value",
    imp: "outputs an impulse signal",
    sampler: "play back the sample based on the value of its input. 1.0 triggers the default pitch. a trigger of value 2.0 will make the playback speed double. note: every non-zero value will trigger the playback once.",
    sp: "play back the sample based on the value of its input. 1.0 triggers the default pitch. a trigger of value 2.0 will make the playback speed double. note: every non-zero value will trigger the playback once.",
    buf: "holds a buffer; the input signal should be within the range from 0.0 to 1.0, and the output depends on the input value.",
    seq: "outouts the trigger based on the given pattern.",
    linrange: "maps the input from (-1.0, 1.0) to the given range",
    lpf: "is a low pass filter.",
    hpf: "is a high pass filter.",
    spd: "controls the speed of a sequencer; should be used with seq",
    speed: "controls the speed of a sequencer; should be used with seq",
    noiz: "outputs noise signal.",
    choose: "can be used to select values from its arguments randomly.",
    envperc: "generates a percusive shape envelope.",
    pha: "generates phosor signal.",
    state: "is an experimental node which generated states.",
    pan: "determined the Left Right audio panning.",
    delay: "delays the signal based on the given parameter (milesecond).",
    apf: "is an all pass filter.",
    comb: "is a comb fiter.",
    mix: "mix stereo signals.",
    plate: "is the Dattoro reverb.",
    onepole: "is a one pole filter.",
    allpass: "is another all pass filter based on sample size.",
    delayn: "is the delay based on the sample numbers.",
    monosum: "sums up mono signals.",
    const: "generates a constance value.",
}

const example = {
    sin: ["%ca_ref%c: %csin %c440", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    saw: ["%ca_ref%c: %csaw %c110", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    squ: ["%ca_ref%c: %csqu %c110", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    noiz: ["%ca_ref%c: %cnoiz %c42", "color: #C99E00", "color: #808080", "color: #a84275", "color: #5a9c54"],
    mul: ["%ca_ref%c: %csin %c110 %c>> %cmul %c0.1", "color: #C99E00", "color: #808080", "color: #a84275", 
    "color: #5a9c54", "color: #808080", "color: #8959A8", "color: #3E999F"],
    choose: ["// loadModules() first!\n~a: choose 60 72 0 0\n\nlead: speed 4.0 >> seq ~a >> sp \\blip", ]
    // add: "a_ref: sin 440 >> mul ~am\n\n~am: sin 0.2 >> mul 0.3 >> add 0.5"
    // sin: () => { console.log("%csome_ref: %csin %c440.0", "color: #C99E00", "color: #8959A8", "color: #3E999F") }
}

const note = {
    sin: "The default output range is from 0.0 to 1.0.",
    saw: "The default output range is from 0.0 to 1.0.",
    squ: "The default output range is from 0.0 to 1.0.",
    noiz: "The default output range is from 0.0 to 1.0.",
    choose: "The 0 means rests and the number of zeros can influence the probability."
}

window.docs = { about, params, range, example, note }

window.help = (token) => {
    clear()
    if (token in window.docs.about) {
        log("\n\n%c About: ", "background: black; color:white; font-weight: bold")
        log("%c"+token, `color: yellow`, `${window.docs.about[token]}`)
    }  else {
        error(`Move your cursor to an non-empty place where you wish to search.
        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
    }

    if (token in window.docs.params) {
        log("\n\n%c Parameters: ", "background: black; color:white; font-weight: bold")
        let p = window.docs.params[token];
        p.forEach(a=>{
            let c = a[2] === "modulable" ? "green" : "red"
            let name = `%c(${a[2]}) ${a[0]}`;
            let des = `- ${a[1]}`
            conse.log(name, `color: ${c}`, des)
        })
    }

    if (token in window.docs.example) {
        log("\n\n%c Example: ", "background: black; color:white; font-weight: bold")
        log(...window.docs.example[token])
    }

    if (token in window.docs.note) {
        log("\n\n%c Note: ", "background: black; color:white; font-weight: bold")
        log(window.docs.note[token])
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

window.loadSamples = async (arg) => {
    let list = arg ? arg : window.sampleList.selected
    window.actx.suspend()
    let l = list.length
    let count = l
    for (const key of list) {
      count -= 1
      try {
        let sound = window.sampleDict[key][0];
        log(`Sample %c${key} %cloaded`, "color: green; font-weight: bold", "")
        let u =
        'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' 
        + key + '/' + sound
        let myRequest = new Request(u);
        await fetch(myRequest).then(response => response.arrayBuffer())
        .then(arrayBuffer => {
          window.node.port.postMessage({
            type: "samples",
            sample: new Int16Array(arrayBuffer),
            name: encoder.encode("\\" + key)
          })
        }).catch(e=>log(e));
      } catch(e) {log(e)}
    }
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
                        // console.log(key)
                        // console.log(e.target.result)
                        await window.node.port.postMessage({
                            type: "samples",
                            sample: new Int16Array(e.target.result),
                            name: encoder.encode("\\" + key.replace(".wav", ""))
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
            // console.log("downloaded", arrayBuffer)
            // let buffer = new Uint8Array(arrayBuffer)
            // let wav = new WaveFile(buffer);
            // let sample = wav.getSamples(true, Int16Array)
            // after loading, sent to audioworklet the sample array
            // console.log("sampler \\" + key)
            window.node.port.postMessage({
            type: "samples",
            sample: new Int16Array(arrayBuffer),
            name: encoder.encode("\\" + name)
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

class TextParameterReader {
  constructor(ringbuf) {
    if (ringbuf.type() != "Uint8Array") {
      throw "This class requires a ring buffer of Uint8Array";
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

  URLFromFiles(['./src/glicol-engine.js']).then((e) => {
    
    window.actx.audioWorklet.addModule(e).then(() => {
      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})
      fetch('./src/glicol_wasm.wasm')
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
        "trying to use a non-existent sample.",
        "trying to connect to an invalid reference.",
        "this node parameter only accepts a number.",
        "unable to build the node.",
      ]

      window.node.port.onmessage = e => {
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
      log(`\n\n%c Available nodes: `, "background: black; color:white; font-weight: bold");
      log(["sin", "saw", "squ", "mul", "add", "imp", "sampler", "sp", "buf", "seq", "linrange", "lpf", "hpf", "spd", "speed", "noiz", "choose", "envperc", "pha", "state", "pan", "delay", "apf", "comb", "mix", "plate", "onepole", "allpass", "delayn", "monosum", "const"])
  
      log(`\n\n%c Fetch help files by: `, "background: black; color:white; font-weight: bold")
      log(`Method 1: typing %chelp("the node name")%c in the console, e.g. %chelp("sin")%c;\n\nMethod 2: move the cursor to a keyword and press %cAlt+D`, "color:green;font-weight:bold", "color: default", "color:green; font-weight:bold", "color:default", "color: green; font-weight:bold");

      log(`\n\n%c Useful console commands: `, "background: black; color:white; font-weight: bold")
      log(`\n%cloadSamples()\n%cKeep the argument empty to load the selected samples by us.\n\n%cbpm()\n%cSet the BPM. The default is 120.\n\n%caddSample()\n%cAdd your own samples. The first argument is the sample name you wish to call, and the second arg is the url to the wav file. Keep the augument empty to load local samples. The files should end with .wav. The file name will become the keys. Only lowercase letters and numbers are valid keys, e.g 808bd.`, "color:green; font-weight:bold", "", "color:green; font-weight:bold", "", "color:green; font-weight:bold", "");
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

lead: saw ~pitch >> mul ~env >> lpf ~cut 3.0 
>> mul 0.6 >> plate 0.1

~cut: squ 0.5 >> linrange 300.0 8000.0`


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

window.runGlicolCode = (code) =>{

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

window.art = `
 ██████╗ ██╗     ██╗ ██████╗ ██████╗ ██╗     
██╔════╝ ██║     ██║██╔════╝██╔═══██╗██║     
██║  ███╗██║     ██║██║     ██║   ██║██║     
██║   ██║██║     ██║██║     ██║   ██║██║     
╚██████╔╝███████╗██║╚██████╗╚██████╔╝███████╗
 ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ╚═════╝ ╚══════╝`