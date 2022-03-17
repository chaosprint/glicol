window.help = async (token) => {
    if (!window.docs) {
      await window.loadDocs()
    }

    if (typeof token === "undefined") {
      table(window.showAllNodes())
      return window.emoj
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


window.emoj = '👇'

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


window.addSampleFolder = async () => {
    var input = document.createElement('input');
    input.type = 'file';
    input.webkitdirectory = true
    input.directory = true
    input.multiple = true

    var samplePath = {}
    input.onchange = async (e) => {
        var files = e.target.files;
        // log(`%cSome samples will be skiped as only mono samples are supported so far.`, "color: red; font-weight: bold", "")
        for (var i = 0; i < files.length; i++) {
            await (async function(file) {
                var reader = new FileReader();
                reader.onload = async function(e) {
                    log("file type", file.type)
                    if (file.type.includes("audio")) {
                        await window.actx.decodeAudioData(e.target.result, buffer => {
                            // if (buffer.numberOfChannels === 1) {
                            // log('file.webkitRelativePath',file.webkitRelativePath, file)
                            const path = file.webkitRelativePath.split("/")
                            const reversed = path.reverse()
                            const filename = reversed[0]
                            const folder = reversed[1]
                            // log("path reversed", reversed)
                            if (folder in samplePath) {
                              samplePath[folder] += 1
                            } else {
                              samplePath[folder] = 0
                            }
                            const name = folder.toLowerCase() + "_" + String(samplePath[folder])
                            log("loading sample: ", name)
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
                              name: encoder.encode("\\"+ name),
                              sr: buffer.sampleRate
                            })
                        })
                    }
                };
                reader.readAsArrayBuffer(file);
            })(files[i]);
        }
    }
    input.click();
}

window.loadSamples = async () => {
    fetch(source+'sample-list.json')
    .then(response => response.json())
    .then(data => {
      // log(Object.keys(data))
      Object.keys(data).filter(name=>name!=="2json.js").forEach(async name=>{
        let myRequest = new Request(source.replace("src/", "")+`assets/${name}.wav`);
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
            }, function(e){ log("Error with decoding audio data" + e.err + name); })
        });
      })
      // log(window.showAllSamples())
    })
    // window.actx.suspend()
    // ['bd0000', 'clav', "pandrum", "panfx", "cb"]
}

window.addSampleFiles = async (name, url) => {
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
                        let name = file.name.toLowerCase().replace(".wav", "").replace(".mp3", "").replace("-","_").replace(" ","_")
                        await window.actx.decodeAudioData(e.target.result, buffer => {
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
                              name: encoder.encode("\\"+ name),
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
        window.actx.suspend()
        let myRequest = new Request(url);
        await fetch(myRequest).then(response => response.arrayBuffer())
        .then(arrayBuffer => {
            window.actx.decodeAudioData(arrayBuffer, buffer => {
                // log(new Int16Array(buffer.getChannelData(0).buffer))
                // let name = file.name.toLowerCase().replace(".wav", "").replace(".mp3", "").replace("-","_").replace(" ","_")
                
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
                      name: encoder.encode("\\"+ name),
                      sr: buffer.sampleRate
                    })
            }, function(e){ log("Error with decoding audio data" + e.err); })
        });
        window.actx.resume()
    }
}
window.showAllSamples = () => Object.keys(window.sampleBuffers)


window.getRandSample = (filter) => {
  var array
  if (filter) {
    array = Object.keys(window.sampleBuffers).filter(x=>x.includes(filter))
  } else {
    array = Object.keys(window.sampleBuffers)
  }
  let result = array[Math.floor(Math.random() * array.length)]
  log(result)
  return result
}

window.rnds = window.getRandSample

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
%cget docs for a node, e.g. help("sin"). if no parameter is given, will list all nodes.
on glicol web editor, you can use key shortcut alt-d (win) / option-d (mac) to trigger this function.
      
%csetBPM(someNumber)\n%cset the BPM. the default is 120.

%caddSampleFolder()
%cchoose a folder that contains samples. the sample name will be FOLDERNAME_ORDER in glicol.
for example:
(1) visit (https://github.com/chaosprint/Dirt-Samples), click [code] -> [download ZIP];
(2) extract {Dirt-Samples-master.zip} to {Dirt-Samples-master} folder;\n(3) run this command in the console and choose the folder.

%caddSampleFiles("some_name", "wav_sample_url")
%cadd your own samples. for example:
// in browser console
addSampleFiles("bd", "https://cdn.jsdelivr.net/gh/chaosprint/glicol@0.9.0/js/assets/808bd.wav")
// in glicol
o: seq 60 >> sp \\bd

for the first para, only lowercase letters, underscore and numbers are valid
keep the second augument empty to load local samples. if you load multiple samples, the name will be automatically created for you.

%cshowAllSamples()
%cshow current loaded samples.

%cgetRandSample("optionalFilter") or rnds("optionalFilter")
%cget a random sample name from current loaded samples.
e.g. if the filter is '0', it will only return a sample whose name contains '0'.

%ctrackAmp(someFloat)
%cset the amplitude of each node chain. useful for preventing clipping.`, 

"background: black; color:white; font-weight: bold",
"color:green; font-weight:bold", "",
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "",
"color:green; font-weight:bold", "", 
"color:green; font-weight:bold", "", 
); return window.emoj
}
  
window.showAllNodes = () => {
let obj = {
    oscillator: ["sin", "squ", "saw", "tri"],
    sequencing: ["seq", "choose"],
    sampling: ["sp", "buf(wip)"],
    signal: ["constsig", "imp", "noise"], //, "pha"
    operator: ["mul", "add"],
    envelope: ["envperc", "shape(wip)"],
    filter: ["lpf", "hpf", "onepole", "apfgain"], //"allpass", , "apfdecay", "comb"
    effect: ["pan(wip)", "balance"],
    dynamic: ["meta"],
    extension: ["plate", "bd", "sn", "hh", "sawsynth", "squsynth", "trisynth"],
}
return obj
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


// https://stackoverflow.com/questions/5916900/how-can-you-detect-the-version-of-a-browser
navigator.sayswho = ( function () {
  var ua = navigator.userAgent, tem,
      M = ua.match( /(opera|chrome|safari|firefox|msie|trident(?=\/))\/?\s*(\d+)/i ) || [];
  if ( /trident/i.test( M[1] ) ) {
      tem = /\brv[ :]+(\d+)/g.exec( ua ) || [];
      return 'IE ' + ( tem[1] || '' );
  }
  if ( M[1] === 'Chrome' ) {
      tem = ua.match( /\b(OPR|Edge)\/(\d+)/ );
      if ( tem != null ) return tem.slice( 1 ).join( ' ' ).replace( 'OPR', 'Opera' );
  }
  M = M[2] ? [M[1], M[2]] : [navigator.appName, navigator.appVersion, '-?'];
  if ( ( tem = ua.match( /version\/(\d+)/i ) ) != null ) M.splice( 1, 1, tem[1] );
  return M.join( ' ' );
} )();
//document.getElementById('printVer').innerHTML=navigator.sayswho
var str = navigator.sayswho;
var browser = str.substring( 0, str.indexOf( " " ) );
var version = str.substring( str.indexOf( " " ) );
version = version.trim();
version = parseInt( version );
// console.log( browser );
// console.log( version );

if (browser == "Chrome") {
  // if (version < 80) {}
} else if (browser = "Firefox") {
  // if (version < 80) {}
} else {
  alert("Glicol requires latest version of Chrome or Firefox browsers");
}

// https://stackoverflow.com/questions/38241480/detect-macos-ios-windows-android-and-linux-os-with-js
function getOS() {
  var userAgent = window.navigator.userAgent,
      platform = window.navigator.platform,
      macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
      windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'],
      iosPlatforms = ['iPhone', 'iPad', 'iPod'],
      os = null;

  if (macosPlatforms.indexOf(platform) !== -1) {
    os = 'Mac OS';
  } else if (iosPlatforms.indexOf(platform) !== -1) {
    os = 'iOS';
  } else if (windowsPlatforms.indexOf(platform) !== -1) {
    os = 'Windows';
  } else if (/Android/.test(userAgent)) {
    os = 'Android';
  } else if (!os && /Linux/.test(platform)) {
    os = 'Linux';
  }

  return os;
}

// log(getOS())