import art from './glicol-ascii'

const loadModule = async () => {
    // Note the the path is from public folder
    // console.log(audioContextOptions.sampleRate )

    window.AudioContext = window.AudioContext || window.webkitAudioContext;
    window.actx = new window.AudioContext({
      sampleRate: 44100
    })
    await window.actx.audioWorklet.addModule('./engine.js')
    window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})

    fetch('./glicol_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => {
      window.node.port.postMessage({
      type: "load", obj: arrayBuffer})
    })

    console.clear();
    console.log("%c"+art, "color: #3E999F; font-weight: bold")
    console.log(`\n\nAvailable nodes:`) //, "background: green; font-weight: bold");
    console.log(Object.keys(window.docs.about))

    console.log(`\n\nFetch help files using:`) //, "background: grey; font-weight: bold");
    console.log(`%chelp("the node name")`, "color: white; background: black; font-weight: bold") //, "background: green");
    // console.log(`%cOr move the cursor to the code and pr`, "background: green");

    // console.log("maxChannelCount", window.actx.destination.maxChannelCount)
    // window.actx.destination.channelCountMode = "explicit";
    window.actx.destination.channelInterpretation = "discrete";
    window.node.connect(window.actx.destination)  
    
    console.log("%cGlicol server is running...", "background: #3E999F; font-weight: bold")

    // navigator.getUserMedia = navigator.getUserMedia
    // || navigator.webkitGetUserMedia
    // || navigator.mozGetUserMedia;
    // navigator.getUserMedia( {audio:true}, stream => {
    // // window.AudioContext = window.AudioContext || window.webkitAudioContext;
    // var mediaStreamSource = window.actx.createMediaStreamSource( stream );
    // // Connect it to the destination to hear yourself (or any other node for processing!)
    // // mediaStreamSource.connect( window.actx.destination );
    // mediaStreamSource.connect( window.node );
    // }, ()=> console.warn("Error getting audio stream from getUserMedia")
    // )
  };

export {loadModule}