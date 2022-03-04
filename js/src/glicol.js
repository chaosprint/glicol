// when publish, change the exact version number
// in local testing, comment the version out!
// window.version = "v0.8.13"
const source = window.version ? `https://cdn.jsdelivr.net/gh/chaosprint/glicol@${version}/js/src/` : "src/"

window.loadDocs = async () => {
  fetch(source+'glicol-api.json')
  .then(response => response.json())
  .then(data => window.docs = data)
}

window.loadDocs()

window.help = async (token) => {
    if (!window.docs) {
      await window.loadDocs()
    }

    if (typeof token === "undefined") {
      window.showAllNodes()
      return {}
    }

    if (token in window.docs) {
      log(
`
%c ${token} %c
${window.docs[token]["description"]}

%c input %c
${window.docs[token]["input"]}

%c output %c
${window.docs[token]["output"]}

%c parameters %c
${JSON.stringify(window.docs[token]["parameters"])}

%c example %c
${window.docs[token]["example"]}
`,
"background: #3b82f6; color:white; font-weight: bold","",
"font-weight: bold; background: #f472b6; color:white", "",
"font-weight: bold; background: #f472b6; color:white", "",
"font-weight: bold; background: #f472b6; color:white", "",
"font-weight: bold; background: #f472b6; color:white", "",
)
        // clear()
        // let node = window.docs[token]
        // log(`%cName: %c${token}`, "color: red", "")
        // log(`%cParameters: %c${"description" in node ? node["description"] : null }`, "color: orange", "")
        // table(node["parameters"])
        // log(`%cIutput: %c${node["input"] !== null ? node["input"].description : null }`, "color: yellow", "")
        // if (node["input"] !== null) {table(node["input"].range)}
        // log(`%cOutput: %c${node["output"].description}`, "color: green", "")
        // table(node["output"].range)
        // log(`%cExample:`, "color: cyan")
        // node["example"].forEach(e=>log(e))
    }  else {
        warn(`Move your cursor to an non-empty place where you wish to search.
        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
    }
}

window.setBPM = (beats_per_minute) => {
  if (typeof beats_per_minute === "number") {
      window.node.port.postMessage({
      type: "bpm", value: beats_per_minute})
      log(`%cBPM set to: ${beats_per_minute}`, "background: green");
      log("%cThis will be effective when you make some changes to the code.", "background: blue");
  } else {
      warn("BPM should be a number.")
  }
}

window.trackAmp = (amp) => {
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
            await (async function(file) {
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
                              window.sampleBuffers[key.replace(".wav", "")] = buffer.getChannelData(0)
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

window.loadSamples = async () => {
    // window.actx.suspend()
    ['bd0000', 'clav-delay-pan-loop-68'].forEach(async name=>{
      let myRequest = new Request(`./assets/${name}.wav`);
      await fetch(myRequest).then(response => response.arrayBuffer())
      .then(arrayBuffer => {
          window.actx.decodeAudioData(arrayBuffer, buffer => {
              // log(new Int16Array(buffer.getChannelData(0).buffer))
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
                name: encoder.encode(name.replace("-","_")),
                sr: buffer.sampleRate
              })
          }, function(e){ log("Error with decoding audio data" + e.err); })
      });
    })

    // window.actx.resume()
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
            }, function(e){ log("Error with decoding audio data" + e.err); })
        });
        window.actx.resume()
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

window.ampVisualColor = '#3b82f6';
// window.visualizerBackground = "rgba(255, 255, 255, 0.5)"
window.visualizerBackground = "white"
window.freqVisualColor = '#f472b6'

window.visualizeTimeDomainData = ({canvas, analyser}) => {
  let ctx = canvas.getContext("2d");
  let bufferLength = analyser.fftSize;
  let dataArray = new Uint8Array(bufferLength);

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  function draw() {

    requestAnimationFrame(draw);

    analyser.getByteTimeDomainData(dataArray);

    ctx.fillStyle = window.visualizerBackground;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.lineWidth = 1;
    ctx.strokeStyle = window.ampVisualColor;

    ctx.beginPath();

    let sliceWidth = canvas.width * 1.0 / bufferLength;
    let x = 0;

    for(let i = 0; i < bufferLength; i++) {
 
      let v = dataArray[i] / 128.0;
      
      let y = canvas.height - v * canvas.height/2;

      if(i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }

      x += sliceWidth;
    }

    ctx.lineTo(canvas.width, canvas.height/2);
    ctx.stroke();
  };

  draw();
}

window.visualizeFrequencyData = ({canvas, analyser}) => {
  let ctx = canvas.getContext("2d");

  let bufferLength = analyser.frequencyBinCount;
  let dataArray = new Uint8Array(bufferLength);

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  function draw() {
    requestAnimationFrame(draw);

    analyser.getByteFrequencyData(dataArray);

    ctx.fillStyle = window.visualizerBackground;
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const barWidth = (canvas.width / bufferLength) * 2.5;

    for(let i = 0; i < bufferLength; i++) {
    	let fractionalVolume = dataArray[i]/255
      let barHeight = fractionalVolume*canvas.height;

      // ctx.fillStyle = 'rgb(' + Math.round(fractionalVolume*155 + 100) + ',20,20)';
      ctx.fillStyle = window.freqVisualColor;
      ctx.fillRect(
      	(barWidth + 1)*i,
        canvas.height-barHeight,
        barWidth,
        barHeight
       );
    }
  };

  draw();
}

window.sampleBuffers = {}

window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    // sampleRate: 44100
  })
  window.analyser = window.actx.createAnalyser();

  await URLFromFiles([source+'glicol-engine.js']).then((e) => {
    
    window.actx.audioWorklet.addModule(e).then(() => {

      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})
      
      fetch(source+'glicol_wasm.wasm')
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        // WebAssembly.instantiateStreaming(arrayBuffer)
        window.node.port.postMessage({
          type: "load", obj: arrayBuffer
        })
      })

      // var importObject = { imports: { imported_func: arg => console.log(arg) } };
      // WebAssembly.instantiateStreaming(source+'glicol_wasm.wasm', importObject).then(res => console.log("ins stream res", res));

      window.actx.destination.channelInterpretation = "discrete";
      window.node.connect(analyser)
      window.analyser.connect(window.actx.destination)
      

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

      window.node.port.onmessage = async e => {
    
          if (e.data.type === 'ready') {
            // log('ready')
            if (Object.keys(window.sampleBuffers).length !== 0) {
              for (let key in window.sampleBuffers) {
                // log(`Sample %c${key} %cloaded`, "color: green; font-weight: bold", "")
                window.node.port.postMessage({
                  type: "samples",
                  sample: window.sampleBuffers[key],
                  name: encoder.encode(key)
                })
              }
            }
          } else if (e.date.type === 'e') {
            if (e.data.info[0] > errors.length) {
              log(e.data.info[0]-1)
            }
            log(`%cError: ${errors[e.data.info[0]-1]}`, "color: white; background: red")
            if (e.data.info[0] === 2) {
                let name = decoder.decode(e.data.info.slice(2).filter(v => v !== 0.0))
                let index = window.code.indexOf(name)
                let code = window.code.slice(0, index)
        
                let line = code.split("\n").length;
                log("%cAt line "+String(line)+".", "color: white; background: green")
            } else {
                log("%cAt line "+String(e.data.info[1]+1)+".", "color: white; background: green")
            }
            log("%cError element: "+String(
              decoder.decode(e.data.info.slice(2))).replace(/[^ -~]+/g, ""),
               "color:white;background:pink");
          }
    }

      // clear();
      // log("%cGlicol has now launched an official website 🚀: \n\nhttps://glicol.org\n\nStill, this playground will continue to be used for quick prototyping, solo live coding and code sharing.", "font-size: 16px")
      log("%c"+window.art, "color: gray") //#3E999F
      // log("%c"+window.version, "background: black; color:white")
      // log(`\n\n%c Available nodes: `, "background: black; color:white; font-weight: bold");
      // log(["seq","speed","choose","mul","add","apfdecay","delayn",
      // "sin","saw","squ","imp","envperc","sampler","noiz","lpf","plate","onepole",
      // "hpf","pha","pan","delay","apfgain","comb","mix","monosum",
      // "const_sig","*","sp","spd","tri","noise","amplfo","balance"])
  
      // log(`\n\n%c Fetch help files by: `, "background: black; color:white; font-weight: bold")
      // log(`Move the cursor to a keyword and press %cAlt+D`, "color:green;font-weight:bold", "color: default", "color:green; font-weight:bold", "color:default", "color: green; font-weight:bold");

      // log(`\n\n%c Useful console commands: `, "background: black; color:white; font-weight: bold")
      log(
`
type %ch()%c in console to see some useful commands.

%cpanic?%c don't panic. %cissue it here: %chttps://github.com/chaosprint/glicol/issues/new
`,
      "font-weight: bold; color: green",
      "",
      "font-weight: bold; color: red",
      "","", "")
    })
  })

}

window.h = () => {
  log(
` 
%cUseful console commands

%chelp("someNodeName")
%cget docs for a node, e.g. help("sin").
if no parameter is given, will list all nodes.
on glicol web editor, you can use key shortcut alt-d (win) / option-d (mac) to trigger this function.
      
%csetBPM(someNumber)\n%cset the BPM. the default is 120. best to do it before you run any code.

%csampleFolder()
%cchoose a folder that contains sub-folders that contain samples. for example:
(1) visit (https://github.com/chaosprint/Dirt-Samples), click [code] -> [download ZIP];
(2) extract {Dirt-Samples-master.zip} to {Dirt-Samples-master} folder;\n(3) run this command in the console and choose the folder.

%csampleCount()
%cuse it after calling the "sampleFolder()" function to see the total number of each sample folder.

%caddSample("some_name", "wav_sample_url")
%cadd your own samples. for example:

// in browser console
addSample("808bd_0", "https://cdn.jsdelivr.net/gh/chaosprint/glicol@0.8.10/js/assets/BD0000.WAV")

// in glicol
o: seq 60 >> sp \\808bd_0

for the first para, only lowercase letters, underscore and numbers are valid
keep the second augument empty to load local samples.
the files should end with .wav. The file name will become the keys.

%ctrackAmp(someFloat)
%cset the amplitude of each node chain. useful for preventing clipping.`, 

"background: black; color:white; font-weight: bold",
"color:green; font-weight:bold", "",
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", ""); return "👇"
}

window.showAllNodes = () => {
  let obj = {
    oscillator: ["sin", "squ", "saw", "tri"],
    sequencing: ["seq", "choose"],
    sampling: ["sp", "buf(wip)"],
    signal: ["const_sig", "imp", "noise", "pha"],
    operator: ["mul", "add"],
    envelope: ["envperc", "shape(wip)"],
    filter: ["lpf", "hpf", "onepole", "allpass", "apfgain", "apfdecay", "comb"],
    effect: ["pan", "balance(wip)"],
    dynamic: ["script"],
    extension: ["plate", "bd", "sn", "hh", "sawsynth", "squsynth", "trisynth"],
  }
  table(obj)
  return "_"
}

window.loadModule();

// window.code = `~gate: speed 2.0
// >> seq 60 _60 _42 48
// ~amp: ~gate >> envperc 0.001 0.1
// // mix js
// ~pit: ~gate >> mul {{Math.pow(2, (60-69)/12) * 440}}

// ~lead: saw ~pit >> mul ~amp >> lpf ~mod 5.0
// >> script \`
//     output = input.map(|x|x*0.1);
//     output
// \` // rhai script
// ~mod: sin 0.2 >> mul 1300 >> add 1500;
// mix: ~lead >> add ~drum >> plate 0.1 // optinal semicolon
// ~drum: speed 4.0 >> seq 60 >> bd 0.1;`

window.code = `o: imp 1 >> sp \\808_0`

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
      type: "update",
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

window.run = async (code) =>{

  // const regexp = /\{([^{}]|(\?R))*\}/g

  // a working JS mix
  
  const regexp = /(?<=\{\{)[^}]*(?=\}\})/g   // this is working but not for nested
  let match;
  let toreplace = [];
  while ((match = regexp.exec(code)) !== null) {
    toreplace.push(match[0])
  }
  toreplace.map((str)=>{

    let result = str.includes('\n') || str.includes(';') ?
    Function(`'use strict'; return ()=>{${str}}`)()() : 
    Function(`'use strict'; return ()=>(${str})`)()()

    if (typeof result !== "undefined") {
      code = code.replace(`{{${str}}}`, result)
    } else {
      code = code.replace(`{{${str}}}`, "")
    }
  })


  window.code = code
  if (!window.isGlicolRunning) {
    window.runCode(code)
  } else {
    window.updateCode(code)
  }

  if ( document.getElementById("visualizer")) {
    window.visualizeTimeDomainData({canvas: document.getElementById("visualizer"), analyser: window.analyser});
  }
  if ( document.getElementById("freqVisualizer")) {
    window.visualizeFrequencyData({canvas: document.getElementById("freqVisualizer"), analyser: window.analyser});
  }
}

window.stop = async () => {
  window.isGlicolRunning = false
  window.clear()
  await window.actx.close();
  await window.loadModule();

}

window.artsource = `
 ██████╗ ██╗     ██╗ ██████╗ ██████╗ ██╗     
██╔════╝ ██║     ██║██╔════╝██╔═══██╗██║     
██║  ███╗██║     ██║██║     ██║   ██║██║     
██║   ██║██║     ██║██║     ██║   ██║██║     
╚██████╔╝███████╗██║╚██████╗╚██████╔╝███████╗
 ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ╚═════╝ ╚══════╝`

window.art = window.version ? window.artsource + "\n\n" + window.version : window.artsource + "\n\n" + "Local Test Version"

// ${JSON.stringify([{"freq": "Modulable(440.0)"}])}