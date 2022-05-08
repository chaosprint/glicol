import worklet from './glicol-engine'
import wasm from './glicol_wasm.wasm'
import nosab from './nosab'
import { detectBrowser } from './detect'
import {TextParameterReader, TextParameterWriter, RingBuffer} from './ringbuf'

let text = `( ${String(worklet)} )(${TextParameterReader}, ${RingBuffer})`;

var isSharedArrayBufferSupported = false;
try {
  var sab = new SharedArrayBuffer(1);
  var {name, _} = detectBrowser();
  if (sab && !name.includes('Safari') ) { 
    isSharedArrayBufferSupported = true 
  }
} catch(e){
  console.warn(nosab)
}

class Engine {
    constructor({
      audioContext = new AudioContext(),
      isLiveCoding = false,
      loadSamples = false,
      connectTo,
      onLoaded = () => {}
    }={}) {
        // console.log("audioContext", audioContext);
        // console.log("connectTo", connectTo, "!connectTo", !connectTo);
        (async () => {
            // isLiveCoding = true
            this.encoder = new TextEncoder('utf-8');
            this.decoder = new TextDecoder('utf-8');

            this.audioContext = audioContext;
            this.audioContext.suspend()

            // console.log(text)
            const blob = new Blob([text], { type: "application/javascript" });
            // console.log(blob)
            const module = URL.createObjectURL(blob);
            // console.log(module)
            await this.audioContext.audioWorklet.addModule(module)
            
            if (isSharedArrayBufferSupported) {
              
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
                    paramQueue: sab2,
                    useSAB: true,
                    isLiveCoding: isLiveCoding,
                  },
              })
            } else {
              this.node = new AudioWorkletNode(this.audioContext, 'glicol-engine', {
                outputChannelCount: [2],
                processorOptions: {
                  useSAB: false,
                  isLiveCoding: isLiveCoding,  
                }
              })
            }

            this.sampleBuffers = {}

            this.node.port.onmessage = async e => {
              this.log("%c  GLICOL loaded.  ", "background:#3b82f6; color:white; font-weight: bold; font-family: Courier")
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
                  if (loadSamples) {
                    await this.loadSamples()
                  }
                }
                onLoaded()
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
            if (!connectTo) {
              this.node.connect(this.audioContext.destination)
            } else {
              this.node.connect(connectTo)
            }
            // wasm({env:{now:Date.now}}).then(res=>window._wasm=res);
            // this.log("the imported wasm:", wasm)
            // this.log("the imported wasm as str:", String(wasm))
            let url = String(wasm).replaceAll(' ', '')
            let urlSplit = url.split("/");
            urlSplit.shift()
            let urlNoHead = "/"+urlSplit.join("/")
            let finalUrl = urlNoHead.split(".wasm")[0] + ".wasm"
            // console.log(finalUrl)
            fetch(finalUrl).then(response => response.arrayBuffer())
            .then(arrayBuffer => {
                this.node.port.postMessage({
                    type: "load",
                    obj: arrayBuffer
                })
            })
            .catch(e=>{
              console.log(e)
              console.error("fail to load the wasm module. please report it here: https://github.com/chaosprint/glicol")
            })
        })();
    }
    run(code) {

      this.audioContext.resume()
      if (isSharedArrayBufferSupported) {
        // console.log("isSharedArrayBufferSupported", isSharedArrayBufferSupported);
        if (this.codeWriter.available_write()) {
          this.codeWriter.enqueue(this.encoder.encode(code))
        }
      } else {
        this.node.port.postMessage({
          type: "run",
          value: this.encoder.encode(code)
        })
      }
    }

    sendMsg(msg) {
      let str;
      str = msg.slice(-1) === ";"? msg : msg+";" // todo: not robust

      if (isSharedArrayBufferSupported) {
        if (this.paramWriter.available_write()) {
          this.paramWriter.enqueue(this.encoder.encode(str))
        }
      } else {
        this.node.port.postMessage({
          type: "msg",
          value: this.encoder.encode(str)
        })
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

    connect(target) {
      this.node.connect(target)
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

    stop() {
      this.run("")
    }

    showAllSamples() {
      console.table(Object.keys(this.sampleBuffers))
      return ``
    }

    async addSampleFiles(name, url) {
      if (url === undefined) {
          var input = document.createElement('input');
          input.type = 'file';
          input.multiple = true
  
          input.onchange = e => {
              var files = e.target.files;
              // log(files)
              for (var i = 0; i < files.length; i++) {
                  ((file) => {
                      var reader = new FileReader();
                      reader.onload = async (e) => {
                          let name = file.name.toLowerCase().replace(".wav", "").replace(".mp3", "").replaceAll("-","_").replaceAll(" ","_").replaceAll("#","_sharp_")
                          await this.audioContext.decodeAudioData(e.target.result, buffer => {
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
                              console.log("loading sample: ", name)
                              this.node.port.postMessage({
                                type: "loadsample",
                                sample: sample,
                                channels: buffer.numberOfChannels,
                                length: buffer.length,
                                name: this.encoder.encode("\\"+ name),
                                sr: buffer.sampleRate
                              })
                          })
                          // log(`Sample %c${key.replace(".wav", "")} %cloaded`, "color: green; font-weight: bold", "")
                      };
                      reader.readAsArrayBuffer(file);
                    })(files[i]);
              }
          }
          input.click();
      } else {
          this.audioContext.suspend()
          let myRequest = new Request(url);
          await fetch(myRequest).then(response => response.arrayBuffer())
          .then(arrayBuffer => {
              this.audioContext.decodeAudioData(arrayBuffer, buffer => {
                  // log(new Int16Array(buffer.getChannelData(0).buffer))
                  // let name = file.name.toLowerCase().replace(".wav", "").replace(".mp3", "").replace("-","_").replace(" ","_")
                  
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
                        name: this.encoder.encode("\\"+ name),
                        sr: buffer.sampleRate
                      })
              }, function(e){ console.log("Error with decoding audio data" + e.err); })
          });
          this.audioContext.resume()
      }
  }

    addSampleFromDataArray(name, sample, numberOfChannels, length, sampleRate) {
      this.node.port.postMessage({
        type: "loadsample",
        sample: sample,
        channels: numberOfChannels,
        length: length,
        name: this.encoder.encode("\\"+ name),
        sr: sampleRate
      });
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
        // log(this.showAllSamples())
      })
      // this.audioContext.suspend()
      // ['bd0000', 'clav', "pandrum", "panfx", "cb"]
  }

  log(...params) {
    setTimeout(console.log.bind(console, ...params));
  }
}

export default Engine
export * from './nodechain'