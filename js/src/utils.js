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
    }  else {
        warn(`Move your cursor to an non-empty place where you wish to search.
        \nFor example, if you wish to search "sin", your cursor should be inside "sin" like this: s|in`)
    }
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
    ['bd0000', 'clav', "pandrum", "panfx", "cb"].forEach(async name=>{
      let myRequest = new Request(`./assets/${name}.wav`);
      await fetch(myRequest).then(response => response.arrayBuffer())
      .then(arrayBuffer => {
          window.actx.decodeAudioData(arrayBuffer, buffer => {
              // log(new Int16Array(buffer.getChannelData(0).buffer))
              window.sampleBuffers[name] = buffer
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
                name: encoder.encode("\\"+ name.replace("-","_")),
                sr: buffer.sampleRate
              })
          }, function(e){ log("Error with decoding audio data" + e.err); })
      });
    })
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

window.stop = async () => {
  window.isGlicolRunning = false
  window.clear()
  await window.actx.close();
  await window.loadModule();
  window.displayInfo();
}

window.artsource = `
 ██████╗ ██╗     ██╗ ██████╗ ██████╗ ██╗     
██╔════╝ ██║     ██║██╔════╝██╔═══██╗██║     
██║  ███╗██║     ██║██║     ██║   ██║██║     
██║   ██║██║     ██║██║     ██║   ██║██║     
╚██████╔╝███████╗██║╚██████╗╚██████╔╝███████╗
 ╚═════╝ ╚══════╝╚═╝ ╚═════╝ ╚═════╝ ╚══════╝`

window.art = window.version ? window.artsource + "\n\n" + window.version : window.artsource + "\n\n" + "Local Test Version"

window.displayInfo = () => {
  log("%c"+window.art, "color: gray") //#3E999F
  log(
  `
  type %ch()%c in console to see some useful commands.
  
  %cpanic?%c don't panic. %cissue it here: %chttps://github.com/chaosprint/glicol/issues/new
  `,
  "font-weight: bold; color: green",
  "",
  "font-weight: bold; color: red",
  "","", "")
}

window.displayInfo()