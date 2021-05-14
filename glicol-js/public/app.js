window.log = function consoleWithNoSource(...params) {
  setTimeout(console.log.bind(console, ...params));
}

window.clear = function consoleClear() {
  setTimeout(console.clear.bind());
}

window.loadModule = async () => {

  window.AudioContext = window.AudioContext || window.webkitAudioContext;
  window.actx = new window.AudioContext({
    sampleRate: 44100
  })

  URLFromFiles(['engine.js', 'index.js']).then((e) => {
    
    window.actx.audioWorklet.addModule(e).then(() => {
      window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})
      fetch('./glicol_wasm.wasm')
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

      // let sab2 = exports.RingBuffer.getStorageForCapacity(256, Uint8Array);
      // let rb2 = new exports.RingBuffer(sab2, Uint8Array);
      // window.resultWriter = new TextParameterWriter(rb2);
      // window.node.port.postMessage({
      //   type: "result",
      //   data: sab2
      // });

      
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

      // console.clear();
      log("%c"+window.art, "color: grey")
      // console.log("%c"+window.art, "color: grey") //#3E999F
      log(`\n\n%c Available nodes: `, "background: black; color:white; font-weight: bold");
      log(["sin", "saw", "squ", "mul", "add", "imp", "sampler", "sp", "buf", "seq", "linrange", "lpf", "hpf", "spd", "speed", "noiz", "choose", "envperc", "pha", "state", "pan", "delay", "apf", "comb", "mix", "plate", "onepole", "allpass", "delayn", "monosum", "const"])
  
      log(`\n\n%c Fetch help files by: `, "background: black; color:white; font-weight: bold")
      log(`Method 1: typing %chelp("the node name")%c in the console, e.g. %chelp("sin")%c;\n\nMethod 2: move the cursor to a keyword and press %cCtrl+Shift+/`, "color: green", "color: default", "color: green", "color: default", "color: grey");

      // console.log(`\n\n%c Useful console commands: `, "background: black; color:white; font-weight: bold")
      // console.log(``);
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


window.loadSamples = async (list) => {
  window.clear()
  console.log("\n\n%c Available samples: ", "background: black; color:white; font-weight: bold")
  console.log(list.sort())
  window.actx.suspend()
  let l = list.length
  let count = l
  for (const key of list) {
    count -= 1
    try {
      let sound = sampleDict[key][0];
      let u =
      'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' 
      + key + '/' + sound
      let myRequest = new Request(u);
      await fetch(myRequest).then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        // console.log("downloaded", arrayBuffer)
        let buffer = new Uint8Array(arrayBuffer)
        let wav = new WaveFile(buffer);
        let sample = wav.getSamples(true, Int16Array)
        window.node.port.postMessage({
          type: "samples",
          sample: sample,
          name: encoder.encode("\\" + key)
        })
      });
    } catch(e) {}
  }
}

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

window.art = `
 ██████╗ ██╗     ██╗ ██████╗ ██████╗ ██╗     
██╔════╝ ██║     ██║██╔════╝██╔═══██╗██║     
██║  ███╗██║     ██║██║     ██║   ██║██║     
██║   ██║██║     ██║██║     ██║   ██║██║     
╚██████╔╝███████╗██║╚██████╗╚██████╔╝███████╗
 ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ╚═════╝ ╚══════╝`