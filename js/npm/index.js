import worklet from './glicol-engine'
import wasm from './glicol_wasm.wasm'
import {TextParameterReader, TextParameterWriter, RingBuffer} from './ringbuf'

let text = `( ${String(worklet)} )(${TextParameterReader}, ${RingBuffer})`;
// console.log(text)

class Engine {
    constructor(isLiveCoding) {
        (async () => {
            this.encoder = new TextEncoder('utf-8');
            this.decoder = new TextDecoder('utf-8');
            this.audioContext = new AudioContext()
            this.audioContext.suspend()

            const blob = new Blob([text], { type: "application/javascript" });
            const module = URL.createObjectURL(blob);
            await this.audioContext.audioWorklet.addModule(module)

            let sab = RingBuffer.getStorageForCapacity(2048, Uint8Array);
            let rb = new RingBuffer(sab, Uint8Array);
            this.codeWriter = new TextParameterWriter(rb);

            let sab2 = RingBuffer.getStorageForCapacity(2048, Uint8Array);
            let rb2 = new RingBuffer(sab2, Uint8Array);
            this.paramWriter = new TextParameterWriter(rb2);

            this.node = new AudioWorkletNode(this.audioContext, 'glicol-engine', {
                outputChannelCount: [2],
                processorOptions: {
                  codeQueue: sab,
                  paramQueue: sab2
                },
                isLiveCoding: isLiveCoding === true ? true: false
            })

            this.sampleBuffers = {}

            this.node.port.onmessage = async e => {
              if (e.data.type === 'ready') {
          
                if (Object.keys(this.sampleBuffers).length !== 0) {
                  for (let key in this.sampleBuffers) {
                    let buffer = this.sampleBuffers[key];
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
                    this.node.port.postMessage({
                      type: "loadsample",
                      sample: sample,
                      channels: buffer.numberOfChannels,
                      length: buffer.length,
                      name: encoder.encode("\\"+ key.replace("-","_")),
                      sr: buffer.sampleRate
                    })
                  }
                } else {
                  await this.loadSamples()
                }
              } else if (e.data.type === 'e') {
                // let decoder = new TextDecoder("utf-8")
                if (e.data.info[0] === 1) {
                  // log("parsing error.")
                  
                  let info = this.decoder.decode(e.data.info.slice(2).filter(v => v !== 0.0));
                  console.log(info)
                  let pos = parseInt(info.split("pos[")[1].split("]")[0])
                  let line = parseInt(info.split("line[")[1].split("]")[0])
                  let col = parseInt(info.split("col[")[1].split("]")[0])
                  let positives = info.split("positives[")[1].split("]")[0].replace("EOI", "END OF INPUT").split(",").join(" ||")
                  let negatives = info.split("negatives[")[1].split("]")[0].split(",").join(" or")
                  console.log(`%cError at line ${line}`, "background: #3b82f6; color:white; font-weight: bold")
                  let errline = this.code.split("\n")[line-1];
                  let styleErrLine = errline.slice(0, col-1) + "%c %c" + errline.slice(col-1);
                  console.log(styleErrLine, "font-weight: bold; background: #f472b6; color:white", "");
      
                  let positiveResult = positives.length > 0?
                  "expecting "+positives:""
                  console.log(
                      `${"_".repeat(col-1 >=0?col-1:0)}%c^^^ ${positiveResult}${negatives.length > 0?"unexpected "+negatives:""}`,
                      "font-weight: bold; background: #f472b6; color:white");
                } else {
                  console.log(`%c${this.decoder.decode(e.data.info.slice(2).filter(v => v !== 0.0))}`,
                  "font-weight: bold; background: #f472b6; color:white")
                  // background: #3b82f6; color:white; font-weight: bold
                }
              }
            }
            this.node.connect(this.audioContext.destination)
            // wasm({env:{now:Date.now}}).then(res=>window._wasm=res);
            // console.log("wasm func; we don't call it, just want the url",wasm)
            let url = String(wasm).replaceAll(' ', '')
            // console.log("wasm url remove all spaces:",url)
            url = url.split(",\"/")[1];
            // console.log("wasm url trim prefix:",url)
            url = url.split("\")")[0]
            // console.log("wasm url trim end:",url)
            // console.log("url",url)
            fetch(url)
            .then(response => response.arrayBuffer())
            .then(arrayBuffer => {
                this.node.port.postMessage({
                    type: "load", obj: arrayBuffer
                })
            })
        })();
    }
    run(code) {
      this.audioContext.resume()
      if (this.codeWriter.available_write()) {
        this.codeWriter.enqueue(this.encoder.encode(code))
      }
    }

    sendMsg(msg) {
      let str;
      str = msg.slice(-1) === ";"? msg : msg+";"
      if (this.paramWriter.available_write()) {
        this.paramWriter.enqueue(this.encoder.encode(str))
      }
    }

    setBPM(bpm) {
      this.node.port.postMessage({
        type: "bpm", value: bpm
    })
    }

    liveCodingMode(yes_or_no) {
      this.node.port.postMessage({
        type: "livecoding", value: yes_or_no
      })
    }

    reset() {
      // this.node
    }

    play(obj) {
      let code = ``;
      for (let key in obj) {
        code += key + ": ";
        code += obj[key].code + ";\n\n"
      }
      console.log(code)
      this.code = code
      this.run(code)
    }

    showAllSamples() {
      window.table(Object.keys(this.sampleBuffers))
      return ``
    }
    async loadSamples() {

      let source = `https://cdn.jsdelivr.net/gh/chaosprint/glicol@v0.11.9/js/src/`
      fetch(source+'sample-list.json')
      .then(response => response.json())
      .then(data => {
        // log(Object.keys(data))
        Object.keys(data).filter(name=>name!=="2json.js").forEach(async name=>{
          let myRequest = new Request(source.replace("src/", "")+`assets/${name}.wav`);
          await fetch(myRequest).then(response => response.arrayBuffer())
          .then(arrayBuffer => {
              this.audioContext.decodeAudioData(arrayBuffer, buffer => {
                  // log(new Int16Array(buffer.getChannelData(0).buffer))
                  this.sampleBuffers[name] = buffer
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
                  this.node.port.postMessage({
                    type: "loadsample",
                    sample: sample,
                    channels: buffer.numberOfChannels,
                    length: buffer.length,
                    name: this.encoder.encode("\\"+ name.replace("-","_")),
                    sr: buffer.sampleRate
                  })
              }, function(e){ console.log("Error with decoding audio data" + e.err + name); })
          });
        })
        // log(window.showAllSamples())
      })
      // window.actx.suspend()
      // ['bd0000', 'clav', "pandrum", "panfx", "cb"]
  }
}

export default Engine
export * from './nodechain'