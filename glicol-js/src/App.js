import './App.css'
import { BrowserRouter as Router, Switch, Route, useHistory } from "react-router-dom";
import React, { useRef, useState, useEffect } from 'react'
import { AppBar, Toolbar, IconButton, TextField, Fade } from '@material-ui/core'
import { Drawer, Divider, Typography, Modal, Tooltip } from '@material-ui/core'
import { FormGroup, FormControlLabel, Switch as IO} from '@material-ui/core'
import { ThemeProvider } from '@material-ui/styles';
import GitHubIcon from '@material-ui/icons/GitHub';
import SettingsIcon from '@material-ui/icons/Settings';

// import clsx from 'clsx';
import { useStyles, theme } from './styles'
import {Run, Reset, Pause, Menu, Update } from './components/ToolButton'
import MyList from "./components/MyList"

import { WaveFile } from 'wavefile';
import sampleDict from './samples.json';
import {sampleList} from './samples.js';
import {hello, am, fm, usesample, envelope, filter, demo2, demo1, welcome} from './examples'

import Editor from './Editor'
import { CodeContext } from './Context'
import docs from './docs'

import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-glicol";
import "ace-builds/src-noconflict/theme-glicol-night";
// import { setCompleters } from "ace-builds/src-noconflict/ext-language_tools";
// import comp from "./completion"

function Text() {
  let history = useHistory();

  function handleRoomSubmit(e) {
    e.preventDefault()
    // console.log("push", window.room)
    history.push("/"+window.room);
  }

  return (
    <form onSubmit={handleRoomSubmit}>
       {/* <TextField id="room" label="Filled" variant="filled" /> */}
    <TextField
      // id="room"
      // className={classes.text}
      label="Room"
      type="text"
      // name="room"
      variant="filled"
      onChange={e=>{window.room=e.target.value}}
      size="medium"
      fullWidth={true}
      // onChange={}
    />
  </form>  
  )
}

export default function App() {

  const classes = useStyles();
  const encoder = new TextEncoder('utf-8');
  const decoder = new TextDecoder('utf-8');
  // const actx = useRef()
  // const node = useRef()
  const codeRef = useRef(welcome)

  const [code, setCode] = useState(welcome)
  const [height, setHeight] = useState(800)
  const [width, setWidth] = useState(600)
  const [running, setRunning] = useState(false)
  // const loaded = useRef(false)
  const [prog, setProg] = useState(0)
  const [loading, setLoading] = useState(false)
  const [loaded, setLoaded] = useState(false)
  const [sideOpen, setSideOpen] = useState(false)
  const [settingsOpen, setSettingsOpen] = useState(false)
  const [useSamples, setUseSamples] = useState(false)
  // const [showTutorial, setShowTutorial] = useState(false)
  // const history = useHistory();

  window.docs = docs

  const loadModule = async () => {
    // Note the the path is from public folder
    // console.log(audioContextOptions.sampleRate )
    window.code = welcome

    
    window.AudioContext = window.AudioContext || window.webkitAudioContext;
    window.actx = new window.AudioContext({
      sampleRate: 44100
    })
    await window.actx.audioWorklet.addModule('./worklet/engine.js')
    window.node = new AudioWorkletNode(window.actx, 'glicol-engine', {outputChannelCount: [2]})

    fetch('wasm/glicol_wasm.wasm')
    .then(response => response.arrayBuffer())
    .then(arrayBuffer => {
      window.node.port.postMessage({
      type: "load", obj: arrayBuffer})
    })

    // console.log("maxChannelCount", window.actx.destination.maxChannelCount)

    // window.actx.destination.channelCountMode = "explicit";
    window.actx.destination.channelInterpretation = "discrete";
    window.node.connect(window.actx.destination)  
    console.log("Audio engine loaded.")

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

  useEffect(() => {
    setSize()
    try {
      loadModule()
    } catch (e) {
      console.log(e)
    }
  }, []);

  window.addSample = async (name, url) => {
    window.actx.suspend()
    let u = url;
    let myRequest = new Request(u);
    await fetch(myRequest).then(response => response.arrayBuffer())
    .then(arrayBuffer => {
      // console.log("downloaded", arrayBuffer)
      let buffer = new Uint8Array(arrayBuffer)
      let wav = new WaveFile(buffer);
      let sample = wav.getSamples(true, Int16Array)

      // after loading, sent to audioworklet the sample array
      // console.log("sampler \\" + key)
      window.node.port.postMessage({
        type: "samples",
        sample: sample,
        name: encoder.encode("\\" + name)
      })
    });
  }

  // window.addSampleFromGitHub = (ownerName, repoName, folder) => {
  //   // 'https://raw.githubusercontent.com/ownerName/repoName/master/'

  // }

  const loadSamples = async (list) => {
    // const arr = [1,2,3,4,5,6,7,8,9];
    // var arr = list.sort();
    // var newArr = [];
    // while(arr.length) newArr.push(arr.splice(0,4));
    // for (let i = 0; i < list.length; i+=3) {
    //   console.log("%c\\"+list.sort()[i], "color:white;background:green")
    //   console.log("%c\\"+list.sort()[i+1], "color:white;background:red")
    //   console.log("%c\\"+list.sort()[i+2], "color:white;background:blue")
    // }

    console.log("%cAvailable samples: ", "background: green; color:white")
    console.table(list.sort())
    let tempcode = window.code
    setLoading(true)
    window.actx.suspend()
    let l = list.length
    let count = l
    for (const key of list) {
      setProg((l-count)/l*100)
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

          // after loading, sent to audioworklet the sample array
          // console.log("sampler \\" + key)
          window.node.port.postMessage({
            type: "samples",
            sample: sample,
            name: encoder.encode("\\" + key)
          })
        });
      } catch(e) {
        // console.log(e)
      }
    }
    setLoading(false)
    setLoaded(true)
    window.code = tempcode
    // console.log(window.code)
  }

  const change = (v) => {
    setCode(v)
    window.code = v
  }

  const setSize = () => {
    // console.log("set size")
    // if (window.state === "coding") {
      try {
        let w = document.getElementById('AppBar').offsetWidth
        let border =  document.documentElement.clientWidth - w
        let h = document.documentElement.clientHeight
        h = h - document.getElementById('AppBar').offsetHeight - border
        window.editor.container.style.width = `${w}px`
        window.editor.container.style.height = `${h}px`
        window.editor.resize()
      } catch (e) {}
    // } else {
      try {
        let w = window.innerWidth;
        let h = window.innerHeight;
        h -= document.getElementById('AppBar').offsetHeight
        setHeight(h)
        setWidth(w)
      } catch (e) {}
    // }
  }
  window.onresize = setSize

  const handleUpdate = () => {
    
    setRunning(true)
    // console.log(codeRef.current)
    try {
      window.actx.resume()
      window.node.port.postMessage({
        type: "update",
        value: encoder.encode(window.code?window.code:"")
      })
      window.node.onmessage = (event) => {
        // Handling data from the processor.
        console.log(event);
      };
    } catch (e) {
      console.log(e)
    }
  }

  const handleRun = () => {

    window.node.port.onmessage = e => {
      console.log("%cError element: "+decoder.decode(e.data), "color:white;background:pink");
    };

    try {
      window.actx.suspend()
      window.actx.resume()
      setRunning(true)
    } catch (e) {
      console.log(e)
    }
    try {
      window.node.port.postMessage({
        type: "run",
        value: encoder.encode(window.code?window.code:"")
      })
    } catch (e) {
      console.log(e)
    }
  }

  const handlePause = () => {
    window.actx.suspend()
    setRunning(false)
  }

  const handleStop = () => {
    let codetemp = window.code
    try {
      window.actx.close();
      loadModule();
      setRunning(false)
      setLoaded(false)
      setUseSamples(false)
    } catch (e) {
      console.log(e)
    }
    console.log("stop") 
    window.code = codetemp
  }

  const handleSettings = () => {
    setSettingsOpen(true)
  }

  const handleSettingsClose = () => {
    setSettingsOpen(false)
  }

  const handleUseSamples = (e) => {
    setUseSamples(e.target.checked)
    // console.log(e.target.checked)
    if (e.target.checked && !loaded) {
      loadSamples(sampleList.selected)
    }
  }

  const handleList = async (code) => {
    // setShowTutorial(true)
    // history.push("/")
    // window.editor.destroy();
    // console.log("should go to turorial")
    setCode(code);
    window.code = code
    setSize()
    setSideOpen(false);
    codeRef.current=code
    setRunning(false)
    // handleStop();
    // window.actx.close();
    // await loadModule();
    // loadSamples(list)
  }

  return (
    <Router>
    <CodeContext.Provider value={{code, setCode}}>
    <div className="App">
        <ThemeProvider theme={theme}>
        <AppBar
          className={classes.appBar}
          id="AppBar"
        >
        <Toolbar>

        <div className={classes.menu} >
        <Menu onClick = {()=>setSideOpen(true)} />
        </div>

        {loading ? <div></div> : <div id="text"><Text /></div> }

        <div id="control">
        {loading ?
        <Typography className={classes.text}
        >loading samples...[
          {Math.floor(prog)}%] </Typography>
         : <div>
        {!running ? <Run onClick={handleRun}/> :
        (<Pause onClick={handlePause}/> )}
        <Update onClick={handleUpdate} />
        <Reset onClick={handleStop} />

        <Tooltip title="settings">
        <IconButton
          color="inherit"
          edge="end"
          onClick={handleSettings}
        >
        <SettingsIcon fontSize="large" />
        </IconButton>
        </Tooltip>
       </div>}
       </div>
      
        <Drawer
          className={classes.drawer}
          // variant="persistent"
          anchor="left"
          open={sideOpen}
          onClose={()=>setSideOpen(false)}
          classes={{
            paper: classes.drawerPaper,
          }}
        >
        <Toolbar>
        <Typography>v0.1.0</Typography>
        <IconButton
          href="https://github.com/glicol/"
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
        <MyList onClick={()=>{handleList(usesample)}}
          title="use samples." />
        <MyList onClick={()=>handleList(envelope)} title="envelope." />
        <Divider />
        <MyList onClick={()=>handleList(filter)} title="filter." />
        {/* <MyList onClick={()=>{handleList("lead: sin 110.0")}}
          title="template - synthesis." />
        <MyList onClick={()=>{
          handleList("bd: seq 60 >> sampler \\bd", sampleList.selected)}}
          title="template - use samples." /> */}
        <Divider />
        <MyList onClick={()=>{
          handleList(demo1)}}
          title="demo 1." />
         <MyList onClick={()=>{
          handleList(demo2)}}
          title="demo 2." />
        </Drawer>

        </Toolbar> 
        </AppBar>
        <Toolbar />
        
        <Modal
          aria-labelledby="transition-modal-title"
          aria-describedby="transition-modal-description"
          className={classes.modal}
          open={settingsOpen}
          onClose={handleSettingsClose}
          closeAfterTransition
          // onRendered={() => modalRef.current.children[1].children[0].focus()}
          // BackdropComponent={Backdrop}
          BackdropProps={{
            timeout: 500,
          }}
        >
          <Fade in={settingsOpen}>
            <div className={classes.paper}>
            <FormGroup>
            <FormControlLabel
              control={
                <IO
                  checked={useSamples}
                  onChange={handleUseSamples}
                  name="useSamples"
                  color="primary"
                />
              }
              label="use samples?"
              labelPlacement="start"
            />
            </FormGroup>
            </div>
          </Fade>
        </Modal>

        </ThemeProvider>
     </div>
     {/* <div> */}
      {/* <button onClick={()=>{console.log(code)}}>run</button> */}
      {/* <h2>Accounts</h2> */}
      <Switch>
        <Route exact path="/" children={
            <div>
              <AceEditor
                className={classes.editor}
                mode="glicol"
                theme="tomorrow-night"
                fontSize = {20}
                height = {height+"px"}
                width = {width+"px"}
                // style={{ height: "100%", width: "100%"}}
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
            </div> }
        />
        <Route path="/:id" children={<Editor
          handleRun={handleRun}
          handleUpdate={handleUpdate}
          handleStop={handleStop}
          handlePause={handlePause}
        />} />
      </Switch>
    {/* </div> */}
    </CodeContext.Provider>
     </Router>
  )
}