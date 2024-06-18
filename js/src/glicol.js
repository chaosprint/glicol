// when publish, change the exact version number
// in local testing, comment the version out!

// window.version = "v0.13.5"

window.source = window.version ? `https://cdn.jsdelivr.net/gh/chaosprint/glicol@${version}/js/src/` : "./src/"
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
// customised for Glicol
exports = {}

Object.defineProperty(exports, '__esModule', { value: true });

window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    // sampleRate: 44100
  })

  let wasmBlob = await fetch("./src/glicol_wasm_bg.wasm")
    .then(response => response.arrayBuffer())

  window.actx.audioWorklet.addModule(source + "glicol-engine.js").then(() => {
    window.decoder = new TextDecoder('utf-8');
    window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {
      outputChannelCount: [2],
      processorOptions: {
        wasmBlob: wasmBlob,
      },
    })

    window.node.port.postMessage({
      type: "load"
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
              name: "\\"+ key.replace("-","_"),
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
}
window.loadModule();
window.code = `o: seq 60 >> sp \\cb`
window.isGlicolRunning = false
window.encoder = new TextEncoder('utf-8');

window.run = async (codeRaw) => {

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
  window.node.port.postMessage({
    type: "run", value: code
  })

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
