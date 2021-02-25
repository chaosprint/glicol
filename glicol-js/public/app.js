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

      console.clear();
      console.log("%c"+window.art, "color: #3E999F")
      console.log(`\n\n%c Available nodes: `, "background: black; color:white; font-weight: bold");
      console.log(["sin", "saw", "squ", "mul", "add", "imp", "sampler", "sp", "buf", "seq", "linrange", "lpf", "hpf", "spd", "speed", "noiz", "choose", "envperc", "pha", "state", "pan", "delay", "apf", "comb", "mix", "plate", "onepole", "allpass", "delayn", "monosum", "const"])
  
      console.log(`\n\n%c Fetch help files by: `, "background: black; color:white; font-weight: bold")
      console.log(`typing %chelp("the node name")%c in the console, e.g. %chelp("sin")%c;\n\nor move the cursor to a keyword and press %cCtrl+Shift+/`, "color: green", "color: default", "color: green", "color: default", "color: purple");

      // paramWriter.enqueue_change(0, e.target.value)
    })
  })
}
window.loadModule();

