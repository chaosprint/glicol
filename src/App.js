import React, { useRef, useState, useEffect } from 'react'
import { Button } from '@material-ui/core'
import {exampleCode} from './example'
import './App.css'
import { WaveFile } from 'wavefile';
import sampleList from './samples.json';

// import React from "react";
// import { render } from "react-dom";
import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-javascript";
import "ace-builds/src-noconflict/theme-github";

export default function App() {

  const actx = useRef()
  const node = useRef()
  // const [url, setUrl] = useState('alex, 0')
  const [code, setCode] = useState(exampleCode)
  const codeRef = useRef(code)
  // const [isPlaying, setIsPlaying] = useState(false)

  const encoder = new TextEncoder('utf-8');

  const loadModule = async () => {
    // Note the the path is from public folder
    actx.current = new window.AudioContext()
    await actx.current.audioWorklet.addModule('worklet/engine.js')

    node.current = new AudioWorkletNode(actx.current, 'quaver-engine')
    fetch('wasm/quaver.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => node.current.port.postMessage({type: "load", obj: arrayBuffer}))
    node.current.connect(actx.current.destination)

  };

  useEffect(() => {
    loadModule()
    console.log(sampleList)
  }, []);

  const mySamples = ["909", "ab", "insect", "bd", "jazz", "dr",
  "moog", "gtr", "sax", "can", "sf", "fm", "808ht", "808lt", "808hc"]
  // const mySamples = ["bd", "fm"]

  const loadSamples = async () => {
    // var sample;
      // let tuple = url.split(",")
      // let key = tuple[0]
      // let sound = sampleList[key][parseInt(tuple[1])];

      // for (const key of Object.keys(sampleList)) {
      for (const key of mySamples) {
        try {
          let sound = sampleList[key][0];

          let u = 'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' + key + '/' + sound
          // let u = "./samples/" + key + '/' + sound
          let myRequest = new Request(u);
          await fetch(myRequest).then(response => response.arrayBuffer())
          .then(arrayBuffer => {
            console.log(arrayBuffer)
            let buffer = new Uint8Array(arrayBuffer)
            let wav = new WaveFile(buffer);
            let sample = wav.getSamples(true, Int16Array)

            // after loading, sent to audioworklet the sample array
            console.log("\\" + key)
            node.current.port.postMessage({type: "samples", sample: sample, name: encoder.encode("\\" + key)})
          });
        } catch(e) {
          console.log(e)
        }
      }
  }

  // const handlePause = () => {
  //   if (isPlaying) {
  //     actx.current.suspend();
  //     setIsPlaying(false)
  //   } else {
  //     actx.current.resume();
  //     setIsPlaying(true)
  //   }
  // }
  const change = (v) => {
    setCode(v)
    codeRef.current = v
    // console.log("change", code);
  }

  const handleRun = () => {
    actx.current.resume()
    console.log(codeRef.current)
    try {
      node.current.port.postMessage({type: "new_track", value: encoder.encode(codeRef.current)})
    } catch (e) {
      console.log(e)
    }
  }

  return (
    <div className='App'>
      <div className="main">
        {/* <form noValidate autoComplete="off">
          <TextField
            value = {url}
            multiline
            rows="2"
            style={{ width: 500, background: "grey", textAlign: "center"}}
            inputProps={{min: 0, style: { textAlign: 'center' }}}
            onChange = {e=>{e.preventDefault(); setUrl(e.target.value)}}
          />
        </form> */}
        {/* <br /> */}

        {/* <AppBar> */}
        <Button
          variant="contained"
          style={{borderRadius:0, fontFamily: 'Inconsolata'}}
          color="primary"
          onClick={loadSamples}
        >Load</Button>

        <br /> <br />
        {/* </AppBar> */}

        <AceEditor
          mode="javascript"
          theme="github"
          fontSize = {18}
          width = "800px"
          fontFamily = "Inconsolata"
          value = {code}
          onChange={change}
          name="UNIQUE_ID_OF_DIV"
          editorProps={{ $blockScrolling: true }}
          commands={[{   // commands is array of key bindings.
            name: 'Run', //name for the key binding.
            bindKey: {win: 'Ctrl-Enter', mac: 'Command-Enter'}, //key combination used for the command.
            exec: handleRun  //function to execute when keys are pressed.
          }]}
        />,
        {/* <Button
          variant="contained"
          style={{borderRadius:0, fontFamily: 'Inconsolata'}}
          color={isPlaying? "secondary" : "primary"}
          onClick={handlePause}
        >{isPlaying? "Off": "On"}</Button> */}

        {/* <Button
          variant="contained"
          color="default"
          style={{borderRadius:0, fontFamily: 'Inconsolata'}}
          onClick={handleRun}
        >Run</Button> */}

      </div>
    </div>
  );
}