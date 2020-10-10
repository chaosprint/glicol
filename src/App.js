import './App.css'
import React, { useRef, useState, useEffect } from 'react'
import { AppBar, Toolbar, IconButton } from '@material-ui/core'
import { Drawer, Divider, Typography} from '@material-ui/core'
// import CircularProgress from '@material-ui/core/CircularProgress';
import { ThemeProvider } from '@material-ui/styles';
import GitHubIcon from '@material-ui/icons/GitHub';

// import clsx from 'clsx';
import { useStyles, theme } from './styles'
import {Run, Reset, Pause, Menu} from './components/ToolButton'
import MyList from "./components/MyList"

import { WaveFile } from 'wavefile';
import sampleDict from './samples.json';
import {sampleList} from './samples.js';
import {hello, am, fm, usesample, envelope, filter, demo2, demo1} from './examples'

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-glicol";
import "ace-builds/src-noconflict/theme-glicol-night";

export default function App() {

  const classes = useStyles();
  const encoder = new TextEncoder('utf-8');

  const actx = useRef()
  const node = useRef()
  const codeRef = useRef(filter)

  const [code, setCode] = useState(filter)
  const [height, setHeight] = useState(800)
  const [width, setWidth] = useState(600)
  const [running, setRunning] = useState(false)
  const loaded = useRef(false)
  const [prog, setProg] = useState(0)
  const [loading, setLoading] = useState(false)
  const [sideOpen, setSideOpen] = useState(false)

  const loadModule = async () => {
    // Note the the path is from public folder
    actx.current = new window.AudioContext()
    await actx.current.audioWorklet.addModule('./worklet/engine.js')
    node.current = new AudioWorkletNode(actx.current, 'glicol-engine')

    fetch('wasm/glicol_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => node.current.port.postMessage({
      type: "load", obj: arrayBuffer}))
    node.current.connect(actx.current.destination)
    console.log("Audio engine loaded.")
  };

  useEffect(() => {
    setSize()
    try {
      loadModule()
    } catch (e) {
      console.log(e)
    }
  }, []);

  const loadSamples = async (list) => {
    console.log(list)
    setLoading(true)
    actx.current.suspend()
    let l = list.length
    let count = l
    // document.title = "loading samples..."
    for (const key of list) {
      setProg((l-count)/l*100)
      count -= 1
      try {
        // let u = 
        // `https://raw.githubusercontent.com/chaosprint/sema/master/assets/samples/`
        // u += key
        // u += ".wav"

        let sound = sampleDict[key][0];
        let u =
        'https://raw.githubusercontent.com/chaosprint/Dirt-Samples/master/' 
        + key + '/' + sound
        // let u = "./samples/" + key + '/' + sound
        let myRequest = new Request(u);
        await fetch(myRequest).then(response => response.arrayBuffer())
        .then(arrayBuffer => {
          // console.log(arrayBuffer)
          let buffer = new Uint8Array(arrayBuffer)
          let wav = new WaveFile(buffer);
          let sample = wav.getSamples(true, Int16Array)

          // after loading, sent to audioworklet the sample array
          console.log("sampler \\" + key)
          node.current.port.postMessage({
            type: "samples",
            sample: sample,
            name: encoder.encode("\\" + key)
          })
        });
      } catch(e) {
        // console.log(e)
      }
    }
    // document.title = "glicol."
    setLoading(false)
    loaded.current = true
  }

  const change = (v) => {
    setCode(v)
    codeRef.current = v
  }


  const setSize = () => {
    try {
        // let w1 = document.getElementById('AppBar').offsetWidth;
        // let w1 = 0;
        let w = window.innerWidth;
        // let w = w1 < w2 ? w1 : w2
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
        // console.log(w, h)
    } catch (e) {console.log(e)}
  }
  window.onresize = setSize

  const handleUpdate = () => {
    actx.current.resume()
    setRunning(true)
    // console.log(codeRef.current)
    try {
      node.current.port.postMessage({
        type: "update",
        value: encoder.encode(codeRef.current)
      })
    } catch (e) {
      console.log(e)
    }
  }

  const handleRun = async () => {
    actx.current.suspend()
    // if (!loaded.current) {
    //   await loadSamples(sampleList.demo)
    // }
    try {
      actx.current.resume()
      setRunning(true)
    } catch (e) {
      console.log(e)
    }
    // console.log(codeRef.current)
    try {
      node.current.port.postMessage({
        type: "run",
        value: encoder.encode(codeRef.current)
      })
    } catch (e) {
      console.log(e)
    }
  }

  const handlePause = () => {
    actx.current.suspend()
    setRunning(false)
    // console.log(codeRef.current)
  }

  const handleStop = () => {
    try {
      actx.current.close();
      loadModule();
      setRunning(false)
    } catch (e) {
      console.log(e)
    }
    console.log("stop") 
  }

  const handleList = async (code, list=[]) => {
    setCode(code);
    setSideOpen(false);
    codeRef.current=code
    handleStop()
    loadSamples(list)
  }

  return (
    <div className="App">
        <ThemeProvider theme={theme}>
        <AppBar
          className={classes.appBar}
          id="AppBar"
        >
        <Toolbar>

        {loading ?
        <Typography className={classes.text}
        >loading samples. please wait. use [ctrl + shift + i] (or cmd +shift + i on Mac) to see available samples. {
          Math.floor(prog)}% </Typography>:
        <div> 
        {!running ? <Run onClick={handleRun}/> :
        (<Pause onClick={handlePause}/> )}
        <Reset onClick={handleStop} />
       </div>}

        <Menu onClick = {()=>setSideOpen(true)} />

        <Drawer
          className={classes.drawer}
          // variant="persistent"
          anchor="right"
          open={sideOpen}
          onClose={()=>setSideOpen(false)}
          classes={{
            paper: classes.drawerPaper,
          }}
        >
        <Toolbar>
        <Typography>v0.1.0</Typography>
        <IconButton
          href="https://github.com/glicol/glicol"
          target="_blank"
          rel="noopener noreferrer"
          color="inherit"
          style={{marginLeft: 'auto'}}
        ><GitHubIcon /></IconButton>
        </Toolbar>

        <Divider />
        <MyList onClick={()=>handleList(hello)} title="hello world." />
        <MyList onClick={()=>handleList(am)} title="am." />
        <MyList onClick={()=>handleList(fm)} title="fm." />
        <Divider />
        <MyList onClick={()=>{handleList(usesample, sampleList.selected)}}
          title="use samples." />
        <MyList onClick={()=>handleList(envelope)} title="envelope." />
        <Divider />
        <MyList onClick={()=>handleList(filter)} title="filter." />
        <Divider />
        <MyList onClick={()=>{handleList("~sin: sin 110.0")}}
          title="template - synthesis." />
        <MyList onClick={()=>{
          handleList(demo2, sampleList.selected)}}
          title="template - use samples." />
        <Divider />
        <MyList onClick={()=>{
          console.log(sampleList.demo)
          handleList(demo1, sampleList.demo)}}
          title="demo 1." />
        </Drawer>

        </Toolbar> 
        </AppBar>
        <Toolbar />

        <AceEditor
          className={classes.editor}
          mode="glicol"
          theme="tomorrow-night"
          fontSize = {18}
          height = {height+"px"}
          width = {width+"px"}
          fontFamily = "Inconsolata"
          value = {code}
          onChange={change}
          name="UNIQUE_ID_OF_DIV"
          editorProps={{ $blockScrolling: true }}
          commands={[{   // commands is array of key bindings.
            name: 'Run', //name for the key binding.
            bindKey: {win: 'Ctrl-Enter', mac: 'Command-Enter'},
            exec: handleRun  //function to execute when keys are pressed.
          }, {
            name: 'Update',
            bindKey: {win: 'Shift-Enter', mac: 'Shift-Enter'},
            exec: handleUpdate
          }, {
            name: 'Stop',
            bindKey: {win: 'Ctrl-Shift-.', mac: 'Command-Shift-.'},
            exec: handleStop
          }]}
        />
        </ThemeProvider>
     </div>
  )
}